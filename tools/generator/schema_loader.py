"""Schema loading, $ref resolution, union enum extraction, and rustdoc formatting."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Dict, List, Optional

from .config import ENDPOINT_VERSIONS, SCHEMA_DIRS


def _build_ref_uri_map() -> Dict[str, str]:
    m = {"ak_wwise_console_project_open": "`ak.wwise.console.project.open`"}
    for sdir in SCHEMA_DIRS:
        for p in sdir.glob("ak.*.json"):
            uri = p.stem
            m[uri.replace(".", "_").lower()] = f"`{uri}`"
    return m


_REF_URI_MAP = _build_ref_uri_map()

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


def clean_doc(text: Optional[str]) -> str:
    if not text:
        return ""

    def _replace_verbatim(m):
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
    text = re.sub(r"<(https?://[^>]+)>", r"`\1`", text)
    text = text.replace(r"\@", "@")
    text = re.sub(r"\\c\s+'([^']+)'", r"`\1`", text)
    text = re.sub(r'\\c\s+"([^"]+)"', r"`\1`", text)

    def _replace_c_token(m):
        tok = m.group(1)
        trailing = ""
        while tok and tok[-1] in ".,;:!?)":
            trailing = tok[-1] + trailing
            tok = tok[:-1]
        return f"`{tok}`{trailing}" if tok else trailing

    text = re.sub(r"\\c\s+(\S+)", _replace_c_token, text)
    text = re.sub(r"\bRefer\s+\\ref\b", r"Refer to \\ref", text)
    text = re.sub(r"\brefer\s+\\ref\b", r"refer to \\ref", text)

    def _replace_ref(m):
        raw = m.group(1)
        trailing = ""
        while raw and raw[-1] in ".,;:!?)":
            trailing = raw[-1] + trailing
            raw = raw[:-1]
        key = raw.lower()
        if key in _REF_URI_MAP:
            rep = _REF_URI_MAP[key]
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
    cleaned_lines = []
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


def endpoint_version_note(uri: str) -> str:
    """Return a version availability note if the endpoint is not present in all schema versions."""
    all_versions = [d.name for d in SCHEMA_DIRS]
    present = ENDPOINT_VERSIONS.get(uri, all_versions)
    if not all_versions or len(present) == len(all_versions):
        return ""
    if present[0] != all_versions[0] and present[-1] == all_versions[-1]:
        return f"Available since Wwise {present[0]}."
    return f"Available in Wwise {', '.join(present)}."


def schema_description(ctx: Optional["Ctx"], schema: dict) -> str:
    """Return the cleaned description for `schema`, following `$ref` chains if needed."""
    if not isinstance(schema, dict):
        return ""
    cur = schema
    seen = set()
    while isinstance(cur, dict):
        desc = cur.get("description", "")
        if desc:
            return clean_doc(desc)
        ref = cur.get("$ref")
        if not ref or ref in seen or ctx is None:
            break
        seen.add(ref)
        cur = ctx.schemas.resolve(ref, ctx.doc, ctx.local_root)
    return ""


def extract_string_enum_docs(schema) -> Dict[str, str]:
    """Extract per-variant descriptions from a `oneOf` string enum schema."""
    docs: Dict[str, str] = {}
    if isinstance(schema, dict):
        if "oneOf" in schema:
            for branch in schema["oneOf"]:
                if (
                    isinstance(branch, dict)
                    and "enum" in branch
                    and len(branch["enum"]) == 1
                    and isinstance(branch["enum"][0], str)
                ):
                    val = branch["enum"][0]
                    desc = clean_doc(branch.get("description", ""))
                    if desc:
                        docs[val] = desc
        for v in schema.values():
            if isinstance(v, (dict, list)):
                for k, d in extract_string_enum_docs(v).items():
                    docs.setdefault(k, d)
    elif isinstance(schema, list):
        for item in schema:
            for k, d in extract_string_enum_docs(item).items():
                docs.setdefault(k, d)
    return docs


def _compose_struct_doc(header: str, desc: str, uri: str = "") -> str:
    desc = clean_doc(desc)
    ver_note = endpoint_version_note(uri) if uri else ""
    out = header
    if desc:
        out = f"{out}\n\n{desc}"
    if ver_note:
        out = f"{out}\n\n{ver_note}"
    return out


def doc_lines(text: str, prefix: str = "") -> List[str]:
    text = clean_doc(text)
    if not text:
        return []
    out = []
    for line in text.split("\n"):
        if line:
            out.append(f"{prefix}/// {line}")
        else:
            out.append(f"{prefix}///")
    return out


class Schemas:
    def __init__(self, root: Path, fallback_roots=()):
        self.root = root
        self.fallback_roots = list(fallback_roots)
        self._defs: Dict[str, dict] = {}
        self._endpoints: Dict[str, dict] = {}

    def _find_file(self, rel_path) -> Path:
        primary = self.root / rel_path
        if primary.exists():
            return primary
        for fb in reversed(self.fallback_roots):
            candidate = fb / rel_path
            if candidate.exists():
                return candidate
        return primary

    def load_definitions(self, fname: str) -> dict:
        if fname not in self._defs:
            self._defs[fname] = json.loads(
                self._find_file(Path("Definitions") / fname).read_text()
            )
        return self._defs[fname]

    def load_endpoint(self, uri: str) -> dict:
        if uri not in self._endpoints:
            self._endpoints[uri] = json.loads(
                self._find_file(f"{uri}.json").read_text()
            )
        return self._endpoints[uri]

    @staticmethod
    def _walk(doc: dict, pointer: str):
        node = doc
        for part in pointer.strip("/").split("/"):
            if part == "":
                continue
            part = part.replace("~1", "/").replace("~0", "~")
            if not isinstance(node, dict) or part not in node:
                return None
            node = node[part]
        return node

    def resolve(self, ref: str, doc: dict, local_root=None):
        """Resolve a $ref. Returns the target schema dict, or None if dangling."""
        fname, _, pointer = ref.partition("#")
        if fname:
            if fname.endswith("definitions.json"):
                target = self.load_definitions(fname)
            else:
                target = json.loads(self._find_file(fname).read_text())
            return self._walk(target, pointer)
        node = self._walk(doc, pointer)
        if node is None and local_root is not None:
            node = self._walk(local_root, pointer)
        return node


class Ctx:
    """Per-endpoint generation context."""

    def __init__(self, schemas: Schemas, uri: str):
        self.schemas = schemas
        self.uri = uri
        self.doc = schemas.load_endpoint(uri)
        self.local_root = None


def extract_enum_values(node) -> List[str]:
    """Recursively collect `enum` string values from a JSON-schema node."""
    values = []
    if isinstance(node, dict):
        enum = node.get("enum")
        if isinstance(enum, list):
            values.extend(v for v in enum if isinstance(v, str))
        for v in node.values():
            values.extend(extract_enum_values(v))
    elif isinstance(node, list):
        for v in node:
            values.extend(extract_enum_values(v))
    return values


def union_definition_enum_values(
    schema_dirs, definition_file: str, key: str
) -> List[str]:
    """Union of `enum` string values under a definition key across all versions."""
    seen = set()
    ordered = []
    for schema_dir in schema_dirs:
        defs_path = schema_dir / "Definitions" / definition_file
        if not defs_path.exists():
            continue
        defs = json.loads(defs_path.read_text(encoding="utf-8"))
        node = defs.get("definitions", {}).get(key, {})
        for v in extract_enum_values(node):
            if v not in seen:
                seen.add(v)
                ordered.append(v)
    return ordered


def schema_required(schema: dict) -> List[str]:
    req = schema.get("required")
    if req:
        return list(req)
    branches = schema.get("anyOf") or schema.get("oneOf") or []
    branch_reqs = [
        b.get("required")
        for b in branches
        if isinstance(b, dict) and b.get("required")
    ]
    if branch_reqs:
        common = set(branch_reqs[0])
        for r in branch_reqs[1:]:
            common &= set(r)
        return [k for k in branch_reqs[0] if k in common]
    return []
