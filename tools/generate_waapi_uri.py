#!/usr/bin/env python3
"""
Generate src/waapi_function_api.rs and src/waapi_topics_api.rs from the
versioned WAAPI JSON schemas in tools/waapi_schemas/<version>/.

Usage:
    python tools/generate_waapi_uri.py
"""

import json
import re
from pathlib import Path
from typing import Dict, List, Tuple

RUST_KEYWORDS = {'move', 'type', 'mod', 'use', 'fn', 'let', 'if', 'else', 'for', 
                 'while', 'loop', 'match', 'return', 'break', 'continue', 'const',
                 'static', 'struct', 'enum', 'trait', 'impl', 'self', 'super',
                 'crate', 'pub', 'extern', 'unsafe', 'async', 'await', 'dyn'}


def uri_variant_name(uri: str) -> str:
    return uri.replace(".", "_")


def uri_const_path(uri: str) -> str:
    return "::".join(escape_if_keyword(s) for s in uri.split("."))


def parse_function_path(func: str) -> Tuple[List[str], str]:
    """
    Parse a function path like 'ak.wwise.core.object.get' into:
    - module path: ['ak', 'wwise', 'core', 'object']
    - function name: 'get'
    """
    parts = func.split('.')
    return parts[:-1], parts[-1]


def escape_if_keyword(name: str) -> str:
    return f'{name}_' if name in RUST_KEYWORDS else name


def build_module_tree(functions: List[str]) -> Dict:
    """
    Build a nested dictionary tree structure from function paths.
    
    Example:
    ['ak.soundengine.postEvent', 'ak.soundengine.getState']
    becomes:
    {'ak': {'soundengine': {'__functions': ['postEvent', 'getState']}}}
    """
    tree = {}
    
    for func in functions:
        module_path, func_name = parse_function_path(func)
        
        # Navigate to the correct position in the tree
        current = tree
        for module in module_path:
            if module not in current:
                current[module] = {}
            current = current[module]
        
        # Store the function at this level
        if '__functions' not in current:
            current['__functions'] = []
        current['__functions'].append((func_name, func))
    
    return tree


def generate_module_code(
    tree: Dict,
    descriptions: Dict[str, str],
    indent_level: int = 0,
    prefix_parts: Tuple[str, ...] = (),
) -> List[str]:
    """
    Recursively generate Rust module code from the tree structure.
    """
    lines = []
    indent = '    ' * indent_level
    
    # Get all items from tree
    items = [(k, v) for k, v in tree.items() if k != '__functions']
    items.sort(key=lambda x: x[0]) 
    
    functions = tree.get('__functions', [])
    functions.sort(key=lambda x: x[0])
    
    for func_name, full_path in functions:
        escaped_name = escape_if_keyword(func_name)
        lines.extend(format_doc_comment(descriptions.get(full_path, ""), full_path, indent))
        const_line = f'{indent}pub const {escaped_name}: &str = "{full_path}";'
        
        if len(const_line) > 100:
            lines.append(f'{indent}pub const {escaped_name}: &str =')
            lines.append(f'{indent}    "{full_path}";')
        else:
            lines.append(const_line)
    
    for module_name, subtree in items:
        if functions and not lines[-1] == '':
            lines.append('')
        
        mod_parts = (*prefix_parts, module_name)
        if indent_level > 0:
            mod_uri = ".".join(mod_parts)
            lines.append(f'{indent}/// The `{mod_uri}` WAAPI namespace.')
        lines.append(f'{indent}pub mod {module_name} {{')
        
        submodule_lines = generate_module_code(
            subtree, descriptions, indent_level + 1, mod_parts
        )
        lines.extend(submodule_lines)
        
        lines.append(f'{indent}}}')
    
    return lines


def generate_enum_code(
    uris: List[str], enum_name: str, descriptions: Dict[str, str]
) -> List[str]:
    sorted_uris = sorted(uris)
    lines = [
        '',
        f'/// Typed URI enum for `{enum_name}` endpoints.',
        '#[allow(non_camel_case_types)]',
        '#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]',
        f'pub enum {enum_name} {{',
    ]
    for uri in sorted_uris:
        variant = uri_variant_name(uri)
        lines.extend(format_doc_comment(descriptions.get(uri, ""), uri, "    "))
        lines.append(f'    {variant},')
    lines.extend([
        '}',
        '',
        f'impl {enum_name} {{',
        '    /// Returns the WAAPI URI string for this endpoint.',
        "    pub const fn as_str(self) -> &'static str {",
        '        match self {',
    ])
    for uri in sorted_uris:
        variant = uri_variant_name(uri)
        const_path = uri_const_path(uri)
        lines.append(f'            Self::{variant} => {const_path},')
    lines.extend([
        '        }',
        '    }',
        '}',
        '',
        f'impl std::fmt::Display for {enum_name} {{',
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {",
        '        f.write_str(self.as_str())',
        '    }',
        '}',
        '',
        f'impl AsRef<str> for {enum_name} {{',
        '    fn as_ref(&self) -> &str {',
        '        self.as_str()',
        '    }',
        '}',
    ])
    return lines


def generate_api_file(
    functions: List[str],
    output_path: Path,
    enum_name: str,
    descriptions: Dict[str, str],
):
    """
    Generate the complete URI module and enum file.
    """
    # Build the module tree
    tree = build_module_tree(functions)
    
    lines = [
        '/// Wwise Authoring API URI constants organized by namespace.',
        '#[allow(non_upper_case_globals)]',
        '#[allow(non_snake_case)]',
    ]
    
    module_lines = generate_module_code(tree, descriptions, 0)
    lines.extend(module_lines)
    lines.extend(generate_enum_code(functions, enum_name, descriptions))
    content = '\n'.join(lines) + '\n'
    output_path.write_text(content, encoding='utf-8')
    
    print(f"Generated {output_path} with {len(functions)} entries ({enum_name})")


def _version_sort_key(path: Path) -> Tuple[int, ...]:
    return tuple(int(p) for p in re.findall(r"\d+", path.name))


def discover_schema_dirs(schemas_root: Path) -> List[Path]:
    """Return all versioned schema dirs under `schemas_root`, oldest first.

    Union generation parses every version so a superset of endpoints (and
    any 2024-only URI that a future version drops) is preserved.
    """
    dirs = [
        d for d in schemas_root.iterdir()
        if d.is_dir() and re.match(r"\d", d.name)
    ]
    return sorted(dirs, key=_version_sort_key)


_REF_DOC_MAP = {
    "akcurveinterpolation": "`AkCurveInterpolation`",
    "wobjects_index": "the Wwise Objects Reference",
    "wobjectnodetypes": "the Wwise Object Node Types reference",
    "wwiseobject_action": "the Wwise Action object reference",
    "waapi_import": "the WAAPI Import documentation",
    "waapi_create_objects_modes": "the WAAPI object creation modes documentation",
    "waapi_set_lists_modes": "the WAAPI list modes documentation",
    "waapi_path_on_mac": "the WAAPI Mac path documentation",
    "waapi_query": "the WAAPI query documentation",
    "waapi_query_return": "the WAAPI query return options documentation",
    "waql_introduction": "the WAQL introduction documentation",
    "globalcommandsids": "the Wwise global command IDs reference",
    "globalcountersids": "the Wwise global counter IDs reference",
    "defining_custom_commands": "the Wwise custom commands documentation",
    "defining_custom_properties": "the Wwise custom properties documentation",
}


def clean_doc(text: str, ref_uri_map: Dict[str, str]) -> str:
    if not text:
        return ""

    def _replace_verbatim(m: re.Match) -> str:
        inner = m.group(1).strip()
        trailing = ""
        if inner.endswith("."):
            inner = inner[:-1].rstrip()
            trailing = "."
        return f"`{inner}`{trailing}"

    text = re.sub(
        r"\\verbatim\s*(.*?)\s*\\endverbatim",
        _replace_verbatim,
        text,
        flags=re.DOTALL,
    )
    text = re.sub(
        r"<tt>(.*?)</tt>",
        lambda m: f"`{m.group(1).strip()}`",
        text,
        flags=re.DOTALL,
    )
    text = re.sub(
        r"<b>(.*?)</b>",
        lambda m: f"**{m.group(1).strip()}**",
        text,
        flags=re.DOTALL,
    )
    text = re.sub(r"<br\s*/?>", "\n", text, flags=re.IGNORECASE)
    text = text.replace(r"\@", "@")
    text = re.sub(r"\\c\s+'([^']+)'", r"`\1`", text)
    text = re.sub(r'\\c\s+"([^"]+)"', r"`\1`", text)

    def _replace_c_token(m: re.Match) -> str:
        tok = m.group(1)
        trailing = ""
        while tok and tok[-1] in ".,;:!?)":
            trailing = tok[-1] + trailing
            tok = tok[:-1]
        return f"`{tok}`{trailing}" if tok else trailing

    text = re.sub(r"\\c\s+(\S+)", _replace_c_token, text)
    text = re.sub(r"\bRefer\s+\\ref\b", r"Refer to \\ref", text)
    text = re.sub(r"\brefer\s+\\ref\b", r"refer to \\ref", text)

    def _replace_ref(m: re.Match) -> str:
        raw = m.group(1)
        trailing = ""
        while raw and raw[-1] in ".,;:!?)":
            trailing = raw[-1] + trailing
            raw = raw[:-1]
        key = raw.lower()
        if key in ref_uri_map:
            rep = ref_uri_map[key]
        elif key in _REF_DOC_MAP:
            rep = _REF_DOC_MAP[key]
        else:
            rep = f"`{raw}`"
        return f"{rep}{trailing}"

    text = re.sub(r"\\ref\s+(\S+)", _replace_ref, text)
    text = re.sub(r"(?<!`)\[([^\[\]`]+)\](?!`|\()", r"`[\1]`", text)

    lines = []
    for raw_line in text.splitlines():
        line = re.sub(r"[ \t]+", " ", raw_line).strip()
        line = re.sub(r"\s+([.,;:])", r"\1", line)
        lines.append(line)
    while lines and not lines[0]:
        lines.pop(0)
    while lines and not lines[-1]:
        lines.pop()
    cleaned_lines: List[str] = []
    prev_empty = False
    for line in lines:
        if not line:
            if not prev_empty:
                cleaned_lines.append("")
            prev_empty = True
        else:
            cleaned_lines.append(line)
            prev_empty = False
    return "\n".join(cleaned_lines)


def format_doc_comment(desc: str, uri: str, pad: str) -> List[str]:
    lines: List[str] = []
    if desc:
        for line in desc.split("\n"):
            if line:
                lines.append(f"{pad}/// {line}")
            else:
                lines.append(f"{pad}///")
        lines.append(f"{pad}///")
    lines.append(f"{pad}/// WAAPI URI: `{uri}`.")
    return lines


def load_endpoints_from_schemas(
    schema_dirs: List[Path],
) -> Tuple[List[str], List[str], Dict[str, str]]:
    all_uris: List[str] = []
    seen: set = set()
    topic_uris: set = set()
    raw_descriptions: Dict[str, str] = {}
    ref_uri_map: Dict[str, str] = {
        "ak_wwise_console_project_open": "`ak.wwise.console.project.open`"
    }
    for schema_dir in schema_dirs:
        if not schema_dir.exists():
            continue
        for path in sorted(schema_dir.glob("ak.*.json")):
            uri = path.stem
            ref_uri_map[uri.replace(".", "_").lower()] = f"`{uri}`"
            if uri not in seen:
                seen.add(uri)
                all_uris.append(uri)
            data = json.loads(path.read_text(encoding="utf-8"))
            if "publishSchema" in data:
                topic_uris.add(uri)
            desc = data.get("description", "")
            if desc:
                raw_descriptions[uri] = desc
    descriptions = {
        uri: clean_doc(desc, ref_uri_map)
        for uri, desc in raw_descriptions.items()
    }
    functions = [u for u in all_uris if u not in topic_uris]
    topics = [u for u in all_uris if u in topic_uris]
    return functions, topics, descriptions


def main():
    script_dir = Path(__file__).resolve().parent
    schemas_root = script_dir / "waapi_schemas"
    version_dirs = discover_schema_dirs(schemas_root)
    if not version_dirs:
        raise FileNotFoundError(
            f"No versioned schema dirs under {schemas_root}. "
            "Run tools/get_waapi_definitions.py first."
        )
    src_dir = script_dir.parent / "src"

    functions, topics, descriptions = load_endpoints_from_schemas(version_dirs)
    print(f"Union of {len(version_dirs)} version(s): {len(functions)} functions, {len(topics)} topics")
    generate_api_file(functions, src_dir / "waapi_function_api.rs", "WaapiFunction", descriptions)
    generate_api_file(topics, src_dir / "waapi_topics_api.rs", "WaapiTopic", descriptions)


if __name__ == '__main__':
    main()
