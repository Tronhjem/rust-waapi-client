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

SCHEMA_VERSION = "2024.1.7"

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


def generate_module_code(tree: Dict, indent_level: int = 0) -> List[str]:
    """
    Recursively generate Rust module code from the tree structure.
    
    Args:
        tree: The module tree dictionary
        indent_level: Current indentation level (multiples of 4 spaces)
    
    Returns:
        List of code lines
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
        
        const_line = f'{indent}pub const {escaped_name}: &str = "{full_path}";'
        
        if len(const_line) > 100:
            lines.append(f'{indent}pub const {escaped_name}: &str =')
            lines.append(f'{indent}    "{full_path}";')
        else:
            lines.append(const_line)
    
    for module_name, subtree in items:
        if functions and not lines[-1] == '':
            lines.append('')
        
        lines.append(f'{indent}pub mod {module_name} {{')
        
        submodule_lines = generate_module_code(subtree, indent_level + 1)
        lines.extend(submodule_lines)
        
        lines.append(f'{indent}}}')
    
    return lines


def generate_enum_code(uris: List[str], enum_name: str) -> List[str]:
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
        lines.append(f'    /// `{uri}`')
        lines.append(f'    {variant},')
    lines.extend([
        '}',
        '',
        f'impl {enum_name} {{',
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


def generate_api_file(functions: List[str], output_path: Path, enum_name: str):
    """
    Generate the complete URI module and enum file.
    
    Args:
        functions: List of WAAPI URI paths
        output_path: Path to write the Rust file
        enum_name: Name of the generated URI enum
    """
    # Build the module tree
    tree = build_module_tree(functions)
    
    lines = [
        '/// Wwise Authoring API endpoints',
        '#[allow(non_upper_case_globals)]',
        '#[allow(non_snake_case)]',
    ]
    
    module_lines = generate_module_code(tree, 0)
    lines.extend(module_lines)
    lines.extend(generate_enum_code(functions, enum_name))
    content = '\n'.join(lines) + '\n'
    output_path.write_text(content, encoding='utf-8')
    
    print(f"Generated {output_path} with {len(functions)} entries ({enum_name})")


def load_endpoints_from_schemas(schema_dir: Path) -> Tuple[List[str], List[str]]:
    if not schema_dir.exists():
        raise FileNotFoundError(
            f"{schema_dir} not found! Run tools/get_waapi_definitions.py first."
        )

    functions: List[str] = []
    topics: List[str] = []
    for path in sorted(schema_dir.glob("ak.*.json")):
        uri = path.stem
        data = json.loads(path.read_text(encoding="utf-8"))
        if "publishSchema" in data:
            topics.append(uri)
        else:
            functions.append(uri)

    return functions, topics


def main():
    script_dir = Path(__file__).resolve().parent
    schema_dir = script_dir / "waapi_schemas" / SCHEMA_VERSION
    src_dir = script_dir.parent / "src"

    functions, topics = load_endpoints_from_schemas(schema_dir)
    generate_api_file(functions, src_dir / "waapi_function_api.rs", "WaapiFunction")
    generate_api_file(topics, src_dir / "waapi_topics_api.rs", "WaapiTopic")


if __name__ == '__main__':
    main()
