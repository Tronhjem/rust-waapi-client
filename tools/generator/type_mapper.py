"""Naming helpers, JSON Schema to Rust type mapping, fallback tracking, and WObjects.xml loaders."""

from __future__ import annotations

import re
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from .config import (
    FIELD_NAME_OVERRIDES,
    PROP_TYPE_OVERRIDES,
    REF_TYPES,
    RESPONSE_REQUIRED_OVERRIDES,
    RESULT_FIELD_NAMES,
    RUST_KEYWORDS,
)
from .schema_loader import Ctx, doc_lines, schema_description, schema_required

# Records (uri, field_context, reason) whenever rust_type falls back to serde_json::Value
FALLBACK_WARNINGS: List[Tuple[str, str, str]] = []


def clear_fallback_warnings() -> None:
    FALLBACK_WARNINGS.clear()


def _record_fallback(ctx: Ctx, field_context: str, reason: str) -> None:
    if field_context:
        FALLBACK_WARNINGS.append((ctx.uri, field_context, reason))


def _words(s: str) -> List[str]:
    """Split an identifier like `originalWavFilePath` or `workunit:isDirty` into words."""
    s = re.sub(r"[^a-zA-Z0-9]+", " ", s)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", s)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1 \2", s)
    return [w for w in s.split() if w]


def pascal_case(s: str) -> str:
    ident = "".join(w[0].upper() + w[1:].lower() for w in _words(s))
    if ident and ident[0].isdigit():
        ident = f"_{ident}"
    return ident


def snake_case(s: str) -> str:
    ident = "_".join(w.lower() for w in _words(s))
    if ident and ident[0].isdigit():
        ident = f"_{ident}"
    if ident in RUST_KEYWORDS:
        ident = f"{ident}_"
    return ident


def escape_ident(name: str) -> str:
    return snake_case(name)


def unique_names(values: List[str], name_fn, suffix: str) -> Dict[str, str]:
    """Map wire names to unique Rust identifiers.

    The schemas list both legacy colon accessors (`workunit:type`) and their
    modern equivalents (`workunitType`); these collapse to the same Rust
    name, so the legacy colon form gets `suffix` appended.
    """
    names: Dict[str, str] = {}
    taken: Dict[str, str] = {}
    for v in values:
        n = name_fn(v)
        if n not in taken:
            names[v] = n
            taken[n] = v
            continue
        other = taken[n]
        if ":" in v:
            bumped, bumper = v, n
        else:
            bumped, bumper = other, n
        candidate = bumper + suffix
        i = 2
        while candidate in taken:
            candidate = f"{bumper}{suffix}{i}"
            i += 1
        names[bumped] = candidate
        taken[candidate] = bumped
        if bumped is other:
            names[v] = n
    return names


def endpoint_base_name(uri: str) -> str:
    """`ak.wwise.core.object.get` -> `CoreObjectGet`, `ak.soundengine.postEvent` -> `SoundenginePostEvent`."""
    parts = uri.split(".")
    if parts[:2] == ["ak", "wwise"]:
        parts = parts[2:]
    elif parts[:1] == ["ak"]:
        parts = parts[1:]
    return "".join(pascal_case(p) for p in parts)


def rust_type(
    sch: dict,
    ctx: Ctx,
    self_name: Optional[str] = None,
    field_context: str = "",
) -> str:
    if not isinstance(sch, dict) or not sch:
        _record_fallback(ctx, field_context, "empty schema")
        return "serde_json::Value"
    if "$ref" in sch:
        ref = sch["$ref"]
        if ref in REF_TYPES:
            mapped = REF_TYPES[ref]
            if mapped == "serde_json::Value":
                _record_fallback(ctx, field_context, f"REF_TYPES[{ref}]")
            return mapped
        if ref == "#" and self_name:
            return self_name
        resolved = ctx.schemas.resolve(ref, ctx.doc, ctx.local_root)
        if resolved is None:
            _record_fallback(ctx, field_context, f"unresolved $ref {ref}")
            return "serde_json::Value"
        return rust_type(
            resolved, ctx, self_name=self_name, field_context=field_context
        )
    t = sch.get("type")
    if isinstance(t, list):
        non_null = [x for x in t if x != "null"]
        if len(non_null) == 1:
            t = non_null[0]
        else:
            _record_fallback(ctx, field_context, f"union type list {t}")
            return "serde_json::Value"
    if t == "string":
        return "String"
    if t == "integer":
        return "i64"
    if t == "number":
        return "f64"
    if t == "boolean":
        return "bool"
    if t == "array":
        item_ty = rust_type(
            sch.get("items", {}),
            ctx,
            self_name,
            field_context=f"{field_context}[]" if field_context else "",
        )
        return f"Vec<{item_ty}>"
    if t == "object":
        props = sch.get("properties", {})
        if "id" in props and "name" in props:
            return "WwiseObjectRef"
        _record_fallback(ctx, field_context, "inline object without typed map")
        return "serde_json::Value"
    branches = sch.get("anyOf") or sch.get("oneOf")
    if branches:
        tys = {
            rust_type(b, ctx, self_name=self_name, field_context="")
            for b in branches
        }
        if len(tys) == 1:
            only = tys.pop()
            if only == "serde_json::Value":
                _record_fallback(
                    ctx, field_context, "anyOf/oneOf resolved to Value"
                )
            return only
        _record_fallback(
            ctx,
            field_context,
            f"multi-type anyOf/oneOf ({', '.join(sorted(tys))})",
        )
        return "serde_json::Value"
    _record_fallback(ctx, field_context, f"unrecognized schema type {t!r}")
    return "serde_json::Value"


def stringy_items(sch: dict, ctx: Ctx) -> bool:
    """True if an array schema's items always resolve to plain strings."""
    items = sch.get("items", {})
    if not isinstance(items, dict):
        return False
    return rust_type(items, ctx) == "String"


class Field:
    def __init__(
        self, wire: str, rust_name: str, ty: str, optional: bool, doc: str = ""
    ) -> None:
        self.wire = wire
        self.rust_name = rust_name
        self.ty = ty
        self.optional = optional
        self.doc = doc

    def serde_attrs(self, kind: str) -> List[str]:
        attrs = []
        rename = self.wire != self.rust_name
        pieces = []
        if rename:
            pieces.append(f'rename = "{self.wire}"')
        if self.optional:
            if kind in ("args", "options"):
                pieces.append('skip_serializing_if = "Option::is_none"')
            else:
                pieces.append("default")
                if self.ty.startswith("Option<"):
                    pieces.append('skip_serializing_if = "Option::is_none"')
        if pieces:
            attrs.append(f"#[serde({', '.join(pieces)})]")
        return attrs

    def declaration(self, kind: str) -> List[str]:
        if self.optional and kind == "response" and self.ty.startswith("Vec<"):
            ty = self.ty  # relies on #[serde(default)]
        elif self.optional and not self.ty.startswith("Option<"):
            ty = f"Option<{self.ty}>"
        else:
            ty = self.ty
        lines = doc_lines(self.doc, "    ") or [f"    /// `{self.wire}`."]
        lines += [f"    {a}" for a in self.serde_attrs(kind)]
        lines.append(f"    pub {self.rust_name}: {ty},")
        return lines


def field_name(uri: str, wire: str, kind: str) -> str:
    if kind == "response" and (uri, wire) in RESULT_FIELD_NAMES:
        return RESULT_FIELD_NAMES[(uri, wire)]
    if wire in FIELD_NAME_OVERRIDES:
        return FIELD_NAME_OVERRIDES[wire]
    return snake_case(wire)


def collect_fields(
    ctx: Ctx,
    schema: dict,
    kind: str,
    struct_name: Optional[str] = None,
) -> List[Field]:
    fields: List[Field] = []
    req_set = set(schema_required(schema))
    if kind == "response" and ctx.uri in RESPONSE_REQUIRED_OVERRIDES:
        req_set |= set(RESPONSE_REQUIRED_OVERRIDES[ctx.uri])
    for wire, sub in (schema.get("properties") or {}).items():
        ty = PROP_TYPE_OVERRIDES.get(
            (ctx.uri, kind, wire)
        ) or PROP_TYPE_OVERRIDES.get((ctx.uri, wire))
        if ty is None:
            if wire == "objectType" or (
                wire == "type"
                and "\\ref wobjects_index" in sub.get("description", "")
            ):
                ty = "ObjectType"
            else:
                ty = rust_type(
                    sub,
                    ctx,
                    self_name=struct_name,
                    field_context=f"{kind}.{wire}",
                )
        optional = wire not in req_set
        fdoc = schema_description(ctx, sub)
        fields.append(
            Field(wire, field_name(ctx.uri, wire, kind), ty, optional, fdoc)
        )
    return fields


def property_variant_name(wire: str) -> str:
    """Convert a Wwise property/reference/list name to a valid Rust enum variant identifier."""
    ident = re.sub(r"[^a-zA-Z0-9_]", "_", wire)
    if ident and ident[0].isdigit():
        ident = f"_{ident}"
    if ident in RUST_KEYWORDS:
        ident = f"{ident}_"
    return ident


def load_object_type_entries(schema_dirs: List[Path]):
    """Union `<WwiseObject>` definitions across all version directories."""
    entries: Dict[str, dict] = {}
    canonical_for_cid: Dict[int, str] = {}
    for schema_dir in schema_dirs:
        wobjects_path = schema_dir / "Definitions" / "WObjects.xml"
        if not wobjects_path.exists():
            continue
        tree = ET.parse(wobjects_path)
        root = tree.getroot()
        for elem in root.iter("WwiseObject"):
            name = elem.attrib.get("Name")
            if not name:
                continue
            company_id = int(elem.attrib.get("CompanyID", "0"))
            plugin_id = int(elem.attrib.get("PluginID", "0"))
            class_id = (plugin_id << 16) | (company_id << 4)
            entries[name] = {
                "class_id": class_id,
                "plugin_id": plugin_id,
                "version": schema_dir.name,
            }
            canonical_for_cid[class_id] = name
    return entries, canonical_for_cid


def load_property_entries(schema_dirs: List[Path]) -> Dict[str, dict]:
    """Union `<Property>`, `<Reference>`, and `<ObjectList>` definitions across all versions."""
    entries: Dict[str, dict] = {}
    for schema_dir in schema_dirs:
        wobjects_path = schema_dir / "Definitions" / "WObjects.xml"
        if not wobjects_path.exists():
            continue
        tree = ET.parse(wobjects_path)
        root = tree.getroot()
        for tag, kind in (
            ("Property", "property"),
            ("Reference", "reference"),
            ("ObjectList", "object list"),
        ):
            for elem in root.iter(tag):
                name = elem.attrib.get("Name")
                if not name:
                    continue
                if name not in entries:
                    entries[name] = {
                        "kind": kind,
                        "type": elem.attrib.get("Type"),
                        "meaning": elem.attrib.get("DataMeaning"),
                        "display": elem.attrib.get("DisplayName"),
                    }
                else:
                    info = entries[name]
                    info["type"] = elem.attrib.get("Type") or info["type"]
                    info["meaning"] = (
                        elem.attrib.get("DataMeaning") or info["meaning"]
                    )
                    info["display"] = (
                        elem.attrib.get("DisplayName") or info["display"]
                    )
    return entries
