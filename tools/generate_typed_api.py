#!/usr/bin/env python3
"""
Generate typed Rust bindings for a subset of WAAPI endpoints.

This generator is intentionally scoped to the Phase 2 pilot endpoints. It reads
static schema descriptions embedded below and writes typed Args/Options/Response
structs plus inherent WaapiClient methods into src/api/generated/.

Usage:
    python tools/generate_typed_api.py
    cargo build && cargo test
"""

from __future__ import annotations

import json
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple

ROOT = Path(__file__).parent.parent
OUT_DIR = ROOT / "src" / "api" / "generated"

RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
    "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
}


# ──────────────────────────────────────────────────────────────────────────────
# Schema model
# ──────────────────────────────────────────────────────────────────────────────

@dataclass
class Field:
    name: str
    ty: str
    required: bool = False
    doc: str = ""
    default: Optional[str] = None
    serialize_with: Optional[str] = None
    flatten: bool = False


@dataclass
class Endpoint:
    uri: str
    rust_method: str
    doc: str
    args: List[Field] = field(default_factory=list)
    options: List[Field] = field(default_factory=list)
    response: List[Field] = field(default_factory=list)
    response_is_unit: bool = False


# ──────────────────────────────────────────────────────────────────────────────
# Pilot endpoint schema descriptions
# ──────────────────────────────────────────────────────────────────────────────

ENDPOINTS: List[Endpoint] = [
    Endpoint(
        uri="ak.wwise.core.getInfo",
        rust_method="get_info",
        doc="Returns information about the currently running instance of Wwise.",
        args=[],
        options=[],
        response=[
            Field("displayName", "String", required=True, doc="Wwise display name."),
            Field("platform", "String", required=True, doc="Platform on which Wwise was built."),
            Field("version", "VersionInfo", required=True, doc="Wwise version information."),
            Field("sessionId", "String", required=True, doc="Wwise session id."),
            Field("apiVersion", "f64", required=True, doc="Version of the Wwise Authoring API."),
            Field("branch", "String", required=True, doc="Branch built."),
            Field("copyright", "String", required=True, doc="Copyright information."),
            Field("configuration", "String", required=True, doc="Release or debug build."),
            Field("isCommandLine", "bool", required=True, doc="True if Wwise is running in command line."),
            Field("processId", "u32", required=True, doc="The process identifier of Wwise."),
            Field("processPath", "String", required=True, doc="The process path of Wwise."),
            Field("directories", "DirectoriesInfo", required=True, doc="Collection of directories used by Wwise."),
        ],
    ),
    Endpoint(
        uri="ak.wwise.core.object.get",
        rust_method="object_get",
        doc="Performs a query and returns data for each object in the result.",
        args=[
            Field("from", "ObjectQuery", required=True, doc="The query starting point."),
            Field("transform", "Vec<Transform>", doc="Sequential transformations on the object list returned by from."),
        ],
        options=[
            Field("return", "Vec<String>", doc="List of fields to return for each object."),
            Field("platform", "PlatformRef", doc="Platform to use for platform-specific values."),
            Field("language", "LanguageRef", doc="Language to use for language-specific values."),
        ],
        response=[
            Field("objects", "Vec<WwiseObject>", required=True, doc="Objects matching the query."),
            Field("return", "Vec<String>", required=True, doc="Fields that were returned."),
        ],
    ),
    Endpoint(
        uri="ak.wwise.core.object.create",
        rust_method="object_create",
        doc="Creates the specified object under the specified parent.",
        args=[
            Field("parent", "ObjectRef", required=True, doc="The parent object."),
            Field("type", "String", required=True, doc="The Wwise object type to create."),
            Field("name", "String", required=True, doc="The name of the new object."),
            Field("onNameConflict", "OnNameConflict", doc="Action to take on a name conflict."),
            Field("platform", "PlatformRef", doc="Platform for the new object."),
            Field("notes", "String", doc="Notes for the new object."),
            Field("children", "Vec<ChildObject>", doc="Child objects to create."),
            Field("listMode", "ListMode", doc="How to modify named lists."),
        ],
        options=[
            Field("return", "Vec<String>", doc="Fields to return."),
            Field("platform", "PlatformRef", doc="Platform for return values."),
            Field("language", "LanguageRef", doc="Language for return values."),
        ],
        response=[
            Field("objects", "Vec<WwiseObject>", doc="Objects that were created."),
        ],
    ),
    Endpoint(
        uri="ak.wwise.core.object.delete",
        rust_method="object_delete",
        doc="Deletes the specified object.",
        args=[
            Field("object", "ObjectRef", required=True, doc="The object to delete."),
            Field("autoCheckOutToSourceControl", "bool", doc="Automatically check out affected work units."),
        ],
        options=[],
        response_is_unit=True,
    ),
    Endpoint(
        uri="ak.wwise.core.audio.import",
        rust_method="audio_import",
        doc="Imports audio files and creates Wwise objects.",
        args=[
            Field("importOperation", "ImportOperation", doc="How imported objects are created."),
            Field("imports", "Vec<ImportItem>", required=True, doc="Array of import commands."),
            Field("default", "ImportDefaults", doc="Default values for each import item."),
            Field("autoAddToSourceControl", "bool", doc="Automatically add imported files to source control."),
            Field("autoCheckOutToSourceControl", "bool", doc="Automatically check out modified files."),
        ],
        options=[
            Field("return", "Vec<String>", doc="Fields to return for each imported object."),
            Field("platform", "PlatformRef", doc="Platform for return values."),
            Field("language", "LanguageRef", doc="Language for return values."),
        ],
        response=[
            Field("files", "Vec<String>", required=True, doc="Imported original file paths."),
            Field("log", "Vec<ImportLogEntry>", required=True, doc="Audio import log."),
            Field("objects", "Vec<WwiseObject>", required=True, doc="Imported objects."),
        ],
    ),
    Endpoint(
        uri="ak.wwise.core.project.save",
        rust_method="project_save",
        doc="Saves the current project.",
        args=[
            Field("autoCheckOutToSourceControl", "bool", doc="Automatically check out affected work units."),
        ],
        options=[],
        response_is_unit=True,
    ),
    Endpoint(
        uri="ak.wwise.ui.getSelectedObjects",
        rust_method="ui_get_selected_objects",
        doc="Returns the list of objects currently selected in the UI.",
        args=[],
        options=[
            Field("return", "Vec<String>", doc="Fields to return."),
            Field("platform", "PlatformRef", doc="Platform for return values."),
            Field("language", "LanguageRef", doc="Language for return values."),
        ],
        response=[
            Field("objects", "Vec<WwiseObject>", required=True, doc="Selected objects."),
        ],
    ),
    Endpoint(
        uri="ak.soundengine.postEvent",
        rust_method="soundengine_post_event",
        doc="Posts an event to the sound engine.",
        args=[
            Field("event", "EventRef", required=True, doc="The event GUID, name, or short ID."),
            Field("gameObject", "GameObjectRef", required=True, doc="The game object identifier."),
        ],
        options=[],
        response=[
            Field("playingId", "u32", doc="The playing ID."),
        ],
    ),
    Endpoint(
        uri="ak.soundengine.setRTPCValue",
        rust_method="soundengine_set_rtpc_value",
        doc="Sets the value of a real-time parameter control.",
        args=[
            Field("rtpc", "RtpcRef", required=True, doc="The RTPC GUID, name, or short ID."),
            Field("value", "f64", required=True, doc="The new value."),
            Field("gameObject", "GameObjectRef", required=True, doc="The game object identifier."),
        ],
        options=[],
        response_is_unit=True,
    ),
]


# ──────────────────────────────────────────────────────────────────────────────
# Rust type helpers
# ──────────────────────────────────────────────────────────────────────────────

def pascal_case(s: str) -> str:
    return "".join(part.capitalize() for part in s.replace(".", "_").replace("-", "_").split("_"))


def snake_case(s: str) -> str:
    out = []
    for i, ch in enumerate(s):
        if ch.isupper() and i > 0:
            out.append("_")
        out.append(ch.lower())
    return "".join(out)


FIELD_RENAMES: Dict[str, str] = {
    "return": "return_fields",
    "type": "object_type",
    "use": "use_mode",
    "where": "where_filter",
}


def safe_ident(s: str) -> str:
    """Avoid Rust keywords when using a string as an identifier."""
    if s in FIELD_RENAMES:
        return FIELD_RENAMES[s]
    if s in RUST_KEYWORDS:
        return f"{s}_"
    return s


def struct_name(endpoint: Endpoint, suffix: str) -> str:
    parts = endpoint.uri.split(".")
    base = "".join(pascal_case(p) for p in parts[1:])  # drop 'ak'
    return f"{base}{suffix}"


def rust_field_name(name: str) -> str:
    """Return a Rust-idiomatic snake_case field name without raw identifiers."""
    if name in FIELD_RENAMES:
        return FIELD_RENAMES[name]
    return safe_ident(snake_case(name))


def serde_rename_attr(name: str) -> str:
    """Return a serde rename attribute if the Rust name differs from the WAAPI name."""
    if rust_field_name(name) == name:
        return ""
    return f'#[serde(rename = "{name}")]'



def rust_type_for_schema(schema: dict) -> str:
    """Best-effort JSON schema to Rust type mapping for the pilots."""
    if "$ref" in schema:
        ref = schema["$ref"]
        if "objectArg" in ref:
            return "ObjectRef"
        if "platformArg" in ref:
            return "PlatformRef"
        if "objectReturnOptions" in ref:
            return "ObjectReturnOptions"
        if "objectReturn" in ref:
            return "WwiseObject"
        if "guid" in ref:
            return "String"
        if "uint32" in ref or "int8" in ref or "integer" in ref:
            return "i64"
        if "propertyValue" in ref:
            return "serde_json::Value"
        if "logCentralSeverity" in ref:
            return "LogSeverity"
        if "importOperationType" in ref:
            return "ImportOperation"
        # Fallback: resolve inline later if needed.
        return "serde_json::Value"

    if "enum" in schema:
        return "String"

    ty = schema.get("type")
    if ty == "string":
        return "String"
    if ty == "boolean":
        return "bool"
    if ty == "number":
        return "f64"
    if ty == "integer":
        return "i64"
    if ty == "array":
        items = schema.get("items", {})
        return f"Vec<{rust_type_for_schema(items)}>"
    if ty == "object":
        return "serde_json::Value"
    return "serde_json::Value"


# ──────────────────────────────────────────────────────────────────────────────
# Code generation
# ──────────────────────────────────────────────────────────────────────────────

def generate_struct(name: str, fields: List[Field], doc: str, derive_default: bool = False, derive_deserialize: bool = False) -> List[str]:
    derives = "Debug, Clone, serde::Serialize"
    if derive_deserialize:
        derives += ", serde::Deserialize"
    lines = ["", f"/// {doc}", f"#[derive({derives})]"]
    if derive_default:
        lines.append("#[derive(Default)]")
    lines.append(f"pub struct {name} {{")
    for f in fields:
        if f.doc:
            lines.append(f"    /// {f.doc}")
        attr = serde_rename_attr(f.name)
        if attr:
            lines.append(f"    {attr}")
        if not f.required:
            lines.append('    #[serde(skip_serializing_if = "Option::is_none")]')
        ty = f.ty
        if not f.required:
            ty = f"Option<{ty}>"
        lines.append(f"    pub {rust_field_name(f.name)}: {ty},")
    lines.append("}")
    return lines


def generate_builder(name: str, fields: List[Field], required_constructor: List[Field]) -> List[str]:
    lines = ["", f"impl {name} {{"]
    ctor_args = ", ".join(f"{safe_ident(snake_case(f.name))}: {f.ty}" for f in required_constructor)

    def field_init(f: Field) -> str:
        field = rust_field_name(f.name)
        local = safe_ident(snake_case(f.name))
        if field == local:
            return f"            {field},"
        return f"            {field}: {local},"

    ctor_body = "\n".join(field_init(f) for f in required_constructor)
    optional_inits = "\n".join(
        f"            {rust_field_name(f.name)}: None,"
        for f in fields if f not in required_constructor
    )
    if required_constructor:
        lines.append(f"    pub fn new({ctor_args}) -> Self {{")
        lines.append("        Self {")
        lines.append(ctor_body)
        if optional_inits:
            lines.append(optional_inits)
        lines.append("        }")
        lines.append("    }")
    else:
        lines.append("    pub fn new() -> Self {")
        lines.append("        Self::default()")
        lines.append("    }")

    for f in fields:
        if f in required_constructor:
            continue
        arg_name = safe_ident(snake_case(f.name))
        method_name = safe_ident(snake_case(f.name))
        arg_ty = f.ty
        lines.append("")
        lines.append(f"    pub fn {method_name}(mut self, {arg_name}: impl Into<{arg_ty}>) -> Self {{")
        lines.append(f"        self.{rust_field_name(f.name)} = Some({arg_name}.into());")
        lines.append("        self")
        lines.append("    }")
    lines.append("}")
    return lines


def generate_serialize(name: str, fields: List[Field]) -> List[str]:
    lines = ["", f"impl Serialize for {name} {{"]
    lines.append("    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        let mut map = serializer.serialize_map(None)?;")
    for f in fields:
        fn_name = f.serialize_with or "serialize_entry"
        accessor = f"self.{rust_field_name(f.name)}"
        if f.required:
            lines.append(f"        map.{fn_name}(&{json.dumps(f.name)}, &{accessor})?;")
        else:
            lines.append(f"        if let Some(v) = &{accessor} {{")
            lines.append(f"            map.{fn_name}(&{json.dumps(f.name)}, v)?;")
            lines.append("        }")
    lines.append("        map.end()")
    lines.append("    }")
    lines.append("}")
    return lines


def generate_client_method(endpoint: Endpoint) -> List[str]:
    args_name = struct_name(endpoint, "Args")
    opts_name = struct_name(endpoint, "Options")
    resp_name = struct_name(endpoint, "Response")
    sig_args = "args: " + args_name if endpoint.args else "_args: ()"
    sig_opts = "options: " + opts_name if endpoint.options else "_options: ()"
    lines = ["", f"    /// {endpoint.doc}", f"    /// URI: `{endpoint.uri}`.", f"    pub async fn {endpoint.rust_method}(",]
    lines.append("        &mut self,")
    lines.append(f"        {sig_args},")
    if endpoint.options:
        lines.append(f"        {sig_opts},")
    ret = "()" if endpoint.response_is_unit else resp_name
    lines.append(f"    ) -> WaapiResult<{ret}> {{")

    args_expr = "Some(serde_json::to_value(&args)?)" if endpoint.args else "None"
    opts_expr = "Some(serde_json::to_value(&options)?)" if endpoint.options else "None"

    if endpoint.response_is_unit:
        lines.append(f"        self.call({json.dumps(endpoint.uri)}, {args_expr}, {opts_expr}).await?;")
        lines.append("        Ok(())")
    else:
        lines.append(f"        let value = self.call({json.dumps(endpoint.uri)}, {args_expr}, {opts_expr}).await?;")
        lines.append(f"        let response: {resp_name} = serde_json::from_value(value)?;")
        lines.append("        Ok(response)")
    lines.append("    }")
    return lines


def generate_endpoint_module(endpoint: Endpoint) -> List[str]:
    args_name = struct_name(endpoint, "Args")
    opts_name = struct_name(endpoint, "Options")
    resp_name = struct_name(endpoint, "Response")

    lines = [
        f"// Generated bindings for {endpoint.uri}",
        "",
        "#![allow(unused_imports)]",
        "#![allow(dead_code)]",
        "",
        "use crate::api::generated::primitives::*;",
        "",
    ]
    args_has_required = any(f.required for f in endpoint.args)
    lines.extend(generate_struct(args_name, endpoint.args, f"Arguments for `{endpoint.uri}`.", derive_default=not args_has_required))
    lines.extend(generate_builder(args_name, endpoint.args, [f for f in endpoint.args if f.required]))

    opts_has_required = any(f.required for f in endpoint.options)
    lines.extend(generate_struct(opts_name, endpoint.options, f"Options for `{endpoint.uri}`.", derive_default=not opts_has_required))
    lines.extend(generate_builder(opts_name, endpoint.options, []))

    if not endpoint.response_is_unit:
        resp_has_required = any(f.required for f in endpoint.response)
        lines.extend(generate_struct(resp_name, endpoint.response, f"Response from `{endpoint.uri}`.", derive_default=not resp_has_required, derive_deserialize=True))

    return lines


def group_endpoints_by_module(endpoints: List[Endpoint]) -> Dict[Tuple[str, ...], List[Endpoint]]:
    groups: Dict[Tuple[str, ...], List[Endpoint]] = defaultdict(list)
    for ep in endpoints:
        parts = ep.uri.split(".")
        # e.g. ak.wwise.core.object.get -> ('ak', 'wwise', 'core', 'object')
        module_path = tuple(parts[:-1])
        groups[module_path].append(ep)
    return groups


def generate_module_tree(groups: Dict[Tuple[str, ...], List[Endpoint]], path: Tuple[str, ...]) -> List[str]:
    """Recursively generate mod.rs files for the namespace tree."""
    child_keys = {p[:len(path) + 1] for p in groups if len(p) > len(path) and p[:len(path)] == path}
    child_modules = sorted({p[len(path)] for p in child_keys})

    lines = [
        "// Auto-generated typed WAAPI bindings.",
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
        "",
    ]

    if path == ():
        lines.append("pub mod ak;")
        lines.append("pub mod primitives;")
        lines.append("mod client_impl;")
        return lines

    if child_modules:
        for child in child_modules:
            lines.append(f"pub mod {child};")
        lines.append("")

    # Endpoints at this exact level
    endpoints_here = groups.get(path, [])
    if endpoints_here:
        for ep in endpoints_here:
            module_file = f"{ep.rust_method}.rs"
            lines.append(f"mod {ep.rust_method};")
            lines.append(f"pub use {ep.rust_method}::{{")
            lines.append(f"    {struct_name(ep, 'Args')},")
            if ep.options:
                lines.append(f"    {struct_name(ep, 'Options')},")
            if not ep.response_is_unit:
                lines.append(f"    {struct_name(ep, 'Response')},")
            lines.append("};")

    return lines


def write_modules(groups: Dict[Tuple[str, ...], List[Endpoint]]) -> None:
    # Create directories and write per-endpoint files
    for path, endpoints in groups.items():
        dir_path = OUT_DIR / Path(*path)
        dir_path.mkdir(parents=True, exist_ok=True)
        for ep in endpoints:
            file_path = dir_path / f"{ep.rust_method}.rs"
            file_path.write_text("\n".join(generate_endpoint_module(ep)) + "\n", encoding="utf-8")

    # Write mod.rs files recursively
    all_paths = {()}
    for p in groups:
        for i in range(len(p) + 1):
            all_paths.add(p[:i])

    for path in all_paths:
        dir_path = OUT_DIR / Path(*path)
        mod_path = dir_path / "mod.rs"
        mod_path.write_text("\n".join(generate_module_tree(groups, path)) + "\n", encoding="utf-8")


def endpoint_module_path(uri: str) -> str:
    parts = uri.split(".")[:-1]
    return "::".join(parts)


def write_client_impl(groups: Dict[Tuple[str, ...], List[Endpoint]]) -> None:
    lines = [
        "// Auto-generated typed WaapiClient methods.",
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
        "",
        "use crate::{WaapiClient, WaapiResult};",
        "",
    ]
    for path, endpoints in sorted(groups.items()):
        for ep in endpoints:
            used = []
            if ep.args:
                used.append(struct_name(ep, "Args"))
            if ep.options:
                used.append(struct_name(ep, "Options"))
            if not ep.response_is_unit:
                used.append(struct_name(ep, "Response"))
            if used:
                mod_path = endpoint_module_path(ep.uri)
                lines.append(f"use crate::api::generated::{mod_path}::{{{', '.join(used)}}};")
    lines.append("")
    lines.append("impl WaapiClient {")
    for path, endpoints in sorted(groups.items()):
        for ep in endpoints:
            lines.extend(generate_client_method(ep))
    lines.append("}")

    file_path = OUT_DIR / "client_impl.rs"
    file_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def generate_primitives() -> List[str]:
    return [
        "// Shared primitives referenced by generated typed bindings.",
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
        "",
        "pub use crate::api::primitives::{",
        "    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, PropertyValue,",
        "};",
        "",
        '/// A Wwise object as returned by `object.get` and similar endpoints.',
        '/// Fields are optional because their presence depends on the `return` options.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        "pub struct WwiseObject {",
        '    #[serde(default, rename = "id")]',
        "    pub id: Option<String>,",
        '    #[serde(default)]',
        "    pub name: Option<String>,",
        '    #[serde(default)]',
        "    pub notes: Option<String>,",
        '    #[serde(default, rename = "type")]',
        "    pub object_type: Option<String>,",
        '    #[serde(default)]',
        "    pub path: Option<String>,",
        '    #[serde(default)]',
        "    pub parent: Option<WwiseObjectRef>,",
        '    #[serde(default)]',
        "    pub owner: Option<WwiseObjectRef>,",
        '    #[serde(default)]',
        "    pub workunit: Option<WwiseObjectRef>,",
        '    #[serde(default, rename = "isPlayable")]',
        "    pub is_playable: Option<bool>,",
        '    #[serde(default, rename = "shortId")]',
        "    pub short_id: Option<u32>,",
        '    #[serde(default, rename = "classId")]',
        "    pub class_id: Option<u32>,",
        '    #[serde(default)]',
        "    pub category: Option<String>,",
        '    #[serde(default, rename = "filePath")]',
        "    pub file_path: Option<String>,",
        '    #[serde(default, rename = "childrenCount")]',
        "    pub children_count: Option<u32>,",
        '    #[serde(default, rename = "totalSize")]',
        "    pub total_size: Option<u64>,",
        '    #[serde(default, rename = "mediaSize")]',
        "    pub media_size: Option<u64>,",
        '    #[serde(default, rename = "objectSize")]',
        "    pub object_size: Option<u64>,",
        '    #[serde(default, rename = "structureSize")]',
        "    pub structure_size: Option<u64>",
        "}",
        "",
        '/// A lightweight object reference returned inside other objects.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        "pub struct WwiseObjectRef {",
        '    #[serde(default, rename = "id")]',
        "    pub id: Option<String>,",
        '    #[serde(default)]',
        "    pub name: Option<String>,",
        "}",
        "",
        '/// Platform or language reference used in options.',
        "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]",
        '#[serde(untagged)]',
        "pub enum LanguageRef {",
        "    Name(String),",
        "    Guid(String),",
        "}",
        "",
        'impl From<&str> for LanguageRef {',
        "    fn from(s: &str) -> Self { LanguageRef::Name(s.to_string()) }",
        "}",
        'impl From<String> for LanguageRef {',
        "    fn from(s: String) -> Self { LanguageRef::Name(s) }",
        "}",
        "",
        '/// Event reference accepted by `soundengine.postEvent`.',
        "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]",
        '#[serde(untagged)]',
        "pub enum EventRef {",
        "    Name(String),",
        "    Guid(String),",
        "    ShortId(u32),",
        "}",
        "",
        'impl From<&str> for EventRef {',
        "    fn from(s: &str) -> Self { EventRef::Name(s.to_string()) }",
        "}",
        'impl From<String> for EventRef {',
        "    fn from(s: String) -> Self { EventRef::Name(s) }",
        "}",
        'impl From<u32> for EventRef {',
        "    fn from(id: u32) -> Self { EventRef::ShortId(id) }",
        "}",
        "",
        '/// Game object reference accepted by soundengine calls.',
        "#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]",
        '#[serde(untagged)]',
        "pub enum GameObjectRef {",
        "    Id(u64),",
        "}",
        "",
        'impl From<u64> for GameObjectRef {',
        "    fn from(id: u64) -> Self { GameObjectRef::Id(id) }",
        "}",
        'impl From<u32> for GameObjectRef {',
        "    fn from(id: u32) -> Self { GameObjectRef::Id(id as u64) }",
        "}",
        "",
        '/// RTPC reference accepted by `soundengine.setRTPCValue`.',
        "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]",
        '#[serde(untagged)]',
        "pub enum RtpcRef {",
        "    Name(String),",
        "    Guid(String),",
        "    ShortId(u32),",
        "}",
        "",
        'impl From<&str> for RtpcRef {',
        "    fn from(s: &str) -> Self { RtpcRef::Name(s.to_string()) }",
        "}",
        'impl From<String> for RtpcRef {',
        "    fn from(s: String) -> Self { RtpcRef::Name(s) }",
        "}",
        'impl From<u32> for RtpcRef {',
        "    fn from(id: u32) -> Self { RtpcRef::ShortId(id) }",
        "}",
        "",
        '/// Import operation mode for `audio.import`.',
        "#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]",
        'pub enum ImportOperation {',
        "    #[serde(rename = \"createNew\")]",
        "    CreateNew,",
        "    #[serde(rename = \"useExisting\")]",
        "    #[default]",
        "    UseExisting,",
        "    #[serde(rename = \"replaceExisting\")]",
        "    ReplaceExisting,",
        "}",
        "",
        '/// One item in the `imports` array for `audio.import`.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        'pub struct ImportItem {',
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "objectPath")]',
        "    pub object_path: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "objectType")]',
        "    pub object_type: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "importLanguage")]',
        "    pub import_language: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "importLocation")]',
        "    pub import_location: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "audioFile")]',
        "    pub audio_file: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "audioFileBase64")]',
        "    pub audio_file_base64: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "originalsSubFolder")]',
        "    pub originals_sub_folder: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none")]',
        "    pub event: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "audioSourceNotes")]',
        "    pub audio_source_notes: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "switchAssignation")]',
        "    pub switch_assignation: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "dialogueEvent")]',
        "    pub dialogue_event: Option<String>,",
        '    #[serde(flatten)]',
        "    pub extra: std::collections::HashMap<String, serde_json::Value>,",
        "}",
        "",
        'impl ImportItem {'
        "    pub fn new(object_path: impl Into<String>) -> Self {",
        "        Self {",
        "            object_path: Some(object_path.into()),",
        "            ..Default::default()",
        "        }",
        "    }",
        "}",
        "",
        '/// Default values applied to every item in `audio.import`.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        'pub struct ImportDefaults {',
        '    #[serde(skip_serializing_if = "Option::is_none")]',
        "    pub notes: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "objectType")]',
        "    pub object_type: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "importLanguage")]',
        "    pub import_language: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "importLocation")]',
        "    pub import_location: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "audioFile")]',
        "    pub audio_file: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "audioFileBase64")]',
        "    pub audio_file_base64: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "objectPath")]',
        "    pub object_path: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "originalsSubFolder")]',
        "    pub originals_sub_folder: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none")]',
        "    pub event: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "audioSourceNotes")]',
        "    pub audio_source_notes: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "switchAssignation")]',
        "    pub switch_assignation: Option<String>,",
        '    #[serde(skip_serializing_if = "Option::is_none", rename = "dialogueEvent")]',
        "    pub dialogue_event: Option<String>,",
        '    #[serde(flatten)]',
        "    pub extra: std::collections::HashMap<String, serde_json::Value>,",
        "}",
        "",
        '/// One entry in the audio import log.',
        "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]",
        'pub struct ImportLogEntry {'
        '    #[serde(rename = "severity")]',
        "    pub severity: LogSeverity,",
        '    #[serde(rename = "message")]',
        "    pub message: String,",
        '    #[serde(default, rename = "index")]',
        "    pub index: Option<i64>,",
        "}",
        "",
        '/// Severity of an audio import log entry.',
        "#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]",
        'pub enum LogSeverity {'
        '    #[serde(rename = "Message")]',
        "    Message,",
        '    #[serde(rename = "Warning")]',
        "    Warning,",
        '    #[serde(rename = "Error")]',
        "    Error,",
        '    #[serde(rename = "Fatal Error")]',
        "    FatalError,",
        "}",
        "",
        '/// Argument to the `from` field of `object.get`.',
        "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]",
        "pub enum ObjectQuery {",
        '    #[serde(rename = "ofType")]',
        "    OfType(Vec<String>),",
        '    #[serde(rename = "search")]',
        "    Search(Vec<String>),",
        '    #[serde(rename = "name")]',
        "    Name(Vec<String>),",
        '    #[serde(rename = "path")]',
        "    Path(Vec<String>),",
        '    #[serde(rename = "id")]',
        "    Id(Vec<String>),",
        "}",
        "",
        "impl From<&str> for ObjectQuery {",
        "    fn from(s: &str) -> Self {",
        "        ObjectQuery::Path(vec![s.to_string()])",
        "    }",
        "}",
        "impl From<String> for ObjectQuery {",
        "    fn from(s: String) -> Self {",
        "        ObjectQuery::Path(vec![s])",
        "    }",
        "}",
        "impl From<Vec<String>> for ObjectQuery {",
        "    fn from(v: Vec<String>) -> Self {",
        "        ObjectQuery::Id(v)",
        "    }",
        "}",
        "",
        '/// A single transformation step in an `object.get` query.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        'pub struct Transform {',
        '    #[serde(flatten)]',
        "    pub extra: std::collections::HashMap<String, serde_json::Value>,",
        "}",
        "",
        '/// Object return options reused across endpoints.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        'pub struct ObjectReturnOptions {',
        '    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]',
        "    pub return_fields: Option<Vec<String>>,",
        '    #[serde(skip_serializing_if = "Option::is_none")]',
        "    pub platform: Option<PlatformRef>,",
        '    #[serde(skip_serializing_if = "Option::is_none")]',
        "    pub language: Option<LanguageRef>,",
        "}",
        "",
        '/// Version information returned by `getInfo`.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        'pub struct VersionInfo {'
        '    #[serde(default, rename = "displayName")]',
        "    pub display_name: Option<String>,",
        '    #[serde(default)]',
        "    pub year: Option<u32>,",
        '    #[serde(default)]',
        "    pub major: Option<u32>,",
        '    #[serde(default)]',
        "    pub minor: Option<u32>,",
        '    #[serde(default)]',
        "    pub build: Option<u32>,",
        '    #[serde(default, rename = "nickname")]',
        "    pub nickname: Option<String>,",
        '    #[serde(default)]',
        "    pub schema: Option<u32>,",
        "}",
        "",
        '/// Directory information returned by `getInfo`.',
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        'pub struct DirectoriesInfo {'
        '    #[serde(default)]',
        "    pub user: Option<String>,",
        '    #[serde(default)]',
        "    pub install: Option<String>,",
        '    #[serde(default)]',
        "    pub authoring: Option<String>,",
        '    #[serde(default)]',
        "    pub bin: Option<String>,",
        '    #[serde(default)]',
        "    pub help: Option<String>,",
        "}",
    ]


def write_primitives() -> None:
    (OUT_DIR / "primitives.rs").write_text("\n".join(generate_primitives()) + "\n", encoding="utf-8")


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    groups = group_endpoints_by_module(ENDPOINTS)
    write_primitives()
    write_modules(groups)
    write_client_impl(groups)
    print(f"Generated typed bindings in {OUT_DIR}")


if __name__ == "__main__":
    main()
