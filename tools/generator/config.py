"""Configuration, schema directory discovery, and overrides loading for WAAPI codegen."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Dict, List, Set, Tuple

TOOLS_DIR = Path(__file__).resolve().parent.parent
SCHEMAS_ROOT = TOOLS_DIR / "waapi_schemas"
OUT_DIR = TOOLS_DIR.parent / "src" / "api" / "generated"
OVERRIDES_PATH = TOOLS_DIR / "overrides.json"


def _version_sort_key(path: Path) -> Tuple[Tuple[int, int | str], ...]:
    return tuple(
        (0, int(part)) if part.isdigit() else (1, part)
        for part in path.name.split(".")
    )


def discover_schema_dirs(schemas_root: Path = SCHEMAS_ROOT) -> List[Path]:
    """Return all versioned schema directories sorted oldest-first."""
    dirs = [
        d
        for d in schemas_root.iterdir()
        if d.is_dir() and not d.name.startswith(".")
    ]
    dirs.sort(key=_version_sort_key)
    if not dirs:
        raise FileNotFoundError(
            f"No versioned schema directories under {schemas_root}"
        )
    return dirs


SCHEMA_DIRS = discover_schema_dirs()
SCHEMA_DIR = SCHEMA_DIRS[-1]


def load_overrides(path: Path = OVERRIDES_PATH) -> dict:
    return json.loads(path.read_text())


_OVERRIDES = load_overrides()

EXCLUDED_ENDPOINTS: Set[str] = set(_OVERRIDES.get("excluded_endpoints", []))
EXCLUDED_TOPICS: Set[str] = set(_OVERRIDES.get("excluded_topics", []))
CTOR_ORDER: Dict[str, List[str]] = dict(_OVERRIDES.get("ctor_order", {}))

RESULT_FIELD_NAMES: Dict[Tuple[str, str], str] = {}
for _k, _v in _OVERRIDES.get("result_field_names", {}).items():
    if isinstance(_v, dict):
        for _wire, _rust_name in _v.items():
            RESULT_FIELD_NAMES[(_k, _wire)] = _rust_name
    else:
        _uri, _wire = _k.split("|", 1)
        RESULT_FIELD_NAMES[(_uri, _wire)] = _v

RESULT_WRAPPED_OBJECT_RETURN: Set[str] = set(
    _OVERRIDES.get("result_wrapped_object_return", [])
)

RESPONSE_REQUIRED_OVERRIDES: Dict[str, Set[str]] = {
    uri: set(fields)
    for uri, fields in _OVERRIDES.get("response_required_overrides", {}).items()
}

FIELD_NAME_OVERRIDES: Dict[str, str] = dict(
    _OVERRIDES.get("field_name_overrides", {})
)

PROP_TYPE_OVERRIDES: Dict[Tuple[str, ...], str] = {}
_raw_pto = _OVERRIDES.get("prop_type_overrides", {})
if "by_uri_kind_wire" in _raw_pto or "by_uri_wire" in _raw_pto:
    for _uri, _by_kind in _raw_pto.get("by_uri_kind_wire", {}).items():
        for _kind, _by_wire in _by_kind.items():
            for _wire, _ty in _by_wire.items():
                PROP_TYPE_OVERRIDES[(_uri, _kind, _wire)] = _ty
    for _uri, _by_wire in _raw_pto.get("by_uri_wire", {}).items():
        for _wire, _ty in _by_wire.items():
            PROP_TYPE_OVERRIDES[(_uri, _wire)] = _ty
else:
    for _k, _ty in _raw_pto.items():
        PROP_TYPE_OVERRIDES[tuple(_k.split("|"))] = _ty

REF_TYPES: Dict[str, str] = dict(_OVERRIDES.get("ref_types", {}))

OBJECT_RETURN_OPTIONS_REF = (
    "waapi_definitions.json#/definitions/objectReturnOptions"
)
OBJECT_RETURN_REF = "waapi_definitions.json#/definitions/objectReturn"

RUST_KEYWORDS: Set[str] = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
}


def discover_all_endpoints(
    schema_dirs: List[Path] = SCHEMA_DIRS,
    excluded_endpoints: Set[str] = EXCLUDED_ENDPOINTS,
    excluded_topics: Set[str] = EXCLUDED_TOPICS,
) -> Tuple[List[str], List[str], Dict[str, List[str]]]:
    """Discover all function and topic URIs across all schema directories.

    Returns (functions, topics, endpoint_versions) preserving discovery order.
    """
    ordered_uris: List[str] = []
    seen_uris: Set[str] = set()
    topic_uris: Set[str] = set()
    endpoint_versions: Dict[str, List[str]] = {}

    for schema_dir in schema_dirs:
        ver = schema_dir.name
        for path in sorted(schema_dir.glob("*.json")):
            uri = path.stem
            if uri not in seen_uris:
                seen_uris.add(uri)
                ordered_uris.append(uri)
            endpoint_versions.setdefault(uri, []).append(ver)
            doc = json.loads(path.read_text())
            if "publishSchema" in doc:
                topic_uris.add(uri)

    functions = [
        u
        for u in ordered_uris
        if u not in topic_uris and u not in excluded_endpoints
    ]
    topics = [
        u
        for u in ordered_uris
        if u in topic_uris and u not in excluded_topics
    ]
    return functions, topics, endpoint_versions


DISCOVERED_ENDPOINTS, DISCOVERED_TOPICS, ENDPOINT_VERSIONS = (
    discover_all_endpoints()
)
# Alias kept for backward compatibility with helper scripts
PILOT_ENDPOINTS = DISCOVERED_ENDPOINTS
TOPIC_ENDPOINTS = DISCOVERED_TOPICS
