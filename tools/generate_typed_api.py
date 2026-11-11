#!/usr/bin/env python3
"""Generate typed Rust WAAPI bindings from the Wwise JSON schemas.

Reads the endpoint schemas and shared definition files from
tools/waapi_schemas/<version>/ and writes src/api/generated/.

Requires no running Wwise instance. Regenerate with:

    python tools/generate_typed_api.py
"""

import json
import re
import shutil
from pathlib import Path

SCHEMA_VERSION = "2024.1.7"
SCHEMA_DIR = Path(__file__).resolve().parent / "waapi_schemas" / SCHEMA_VERSION
OUT_DIR = Path(__file__).resolve().parent.parent / "src" / "api" / "generated"

PILOT_ENDPOINTS = [
    "ak.wwise.core.getInfo",
    "ak.wwise.core.object.get",
    "ak.wwise.core.object.create",
    "ak.wwise.core.object.delete",
    "ak.wwise.core.object.set",
    "ak.wwise.core.audio.import",
    "ak.wwise.core.project.save",
    "ak.wwise.ui.getSelectedObjects",
    "ak.soundengine.postEvent",
    "ak.soundengine.setRTPCValue",
]

# Constructor parameter order where the schema's `required` list differs from
# the established API.
CTOR_ORDER = {
    "ak.wwise.core.object.create": ["parent", "type", "name"],
}

# Rust field names for result properties that would otherwise collide with
# keywords or read poorly.
RESULT_FIELD_NAMES = {
    ("ak.wwise.core.object.get", "return"): "objects",
    ("ak.soundengine.postEvent", "return"): "playing_id",
}

# Global wire-name -> Rust field name overrides.
FIELD_NAME_OVERRIDES = {
    "type": "object_type",
}

# Property type overrides for properties whose schema is a plain string enum
# that already has a hand-written Rust type, and for inline nested objects
# that map to generated structs.
PROP_TYPE_OVERRIDES = {
    ("ak.wwise.core.object.create", "onNameConflict"): "OnNameConflict",
    ("ak.wwise.core.object.set", "onNameConflict"): "OnNameConflict",
    ("ak.wwise.core.object.set", "listMode"): "ListMode",
    ("ak.wwise.core.getInfo", "version"): "VersionInfo",
    ("ak.wwise.core.getInfo", "directories"): "DirectoriesInfo",
    ("ak.wwise.core.audio.import", "imports"): "Vec<ImportItem>",
    ("ak.wwise.core.audio.import", "default"): "ImportDefaults",
}

# Type aliases emitted for backwards compatibility with the previous
# (hand-curated) generated names.
COMPAT_ALIASES = {
    "WwiseCoreGetInfoResponse": ["WwiseCoreGetinfoResponse"],
    "SoundenginePostEventArgs": ["SoundenginePosteventArgs"],
    "SoundenginePostEventResponse": ["SoundenginePosteventResponse"],
    "SoundengineSetRtpcValueArgs": ["SoundengineSetrtpcvalueArgs"],
    "WwiseUiGetSelectedObjectsOptions": ["WwiseUiGetselectedobjectsOptions"],
    "WwiseUiGetSelectedObjectsResponse": ["WwiseUiGetselectedobjectsResponse"],
}

# Well-known $ref targets mapped to Rust types. Hand-written types live in
# crate::api::primitives; the rest are generated in generated/primitives.rs.
REF_TYPES = {
    "common_definitions.json#/definitions/guid": "String",
    "common_definitions.json#/definitions/uint32": "u32",
    "common_definitions.json#/definitions/int32": "i32",
    "common_definitions.json#/definitions/uint64": "u64",
    "common_definitions.json#/definitions/int64": "i64",
    "common_definitions.json#/definitions/uint16": "u16",
    "common_definitions.json#/definitions/int8": "i8",
    "common_definitions.json#/definitions/id64bits": "u64",
    "common_definitions.json#/definitions/propertyValue": "PropertyValue",
    "waapi_definitions.json#/definitions/objectArg": "ObjectRef",
    "waapi_definitions.json#/definitions/platformArg": "PlatformRef",
    "waapi_definitions.json#/definitions/soundEngineObjectArg": "SoundEngineObjectRef",
    "waapi_definitions.json#/definitions/gameObject": "GameObjectRef",
    "waapi_definitions.json#/definitions/objectReturn": "WwiseObject",
    "waapi_definitions.json#/definitions/importOperationType": "ImportOperation",
    "waapi_definitions.json#/definitions/logCentralSeverity": "LogSeverity",
    "waapi_definitions.json#/definitions/uniqueName": "String",
    "waapi_definitions.json#/definitions/uniqueQualifiedName": "String",
    "waapi_definitions.json#/definitions/objectPathArg": "String",
    "waapi_definitions.json#/definitions/shortId": "u32",
}

OBJECT_RETURN_OPTIONS_REF = "waapi_definitions.json#/definitions/objectReturnOptions"

RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "box",
}


# ─────────────────────────────────────────────────────────────────────────────
# Naming helpers
# ─────────────────────────────────────────────────────────────────────────────

def _words(name):
    out = []
    for part in re.split(r"[^a-zA-Z0-9]+", name):
        if not part:
            continue
        out.extend(re.findall(r"[A-Z]+(?![a-z])|[A-Z][a-z0-9]*|[a-z0-9]+", part))
    return out


def pascal_case(name):
    return "".join(w[:1].upper() + w[1:].lower() if len(w) > 1 else w.upper() for w in _words(name))


def snake_case(name):
    ident = "_".join(w.lower() for w in _words(name))
    if not ident:
        ident = "value"
    if ident[0].isdigit():
        ident = "n_" + ident
    if ident in RUST_KEYWORDS:
        ident += "_"
    return ident


def clean_doc(text):
    if not text:
        return ""
    text = re.sub(r"\\ref \S+", "", text)
    text = re.sub(r"\s+", " ", text).strip()
    return text


def unique_names(values, name_fn, suffix):
    """Map wire names to unique Rust identifiers.

    The schemas list both legacy colon accessors (``workunit:type``) and their
    modern equivalents (``workunitType``); these collapse to the same Rust
    name, so the legacy colon form gets ``suffix`` appended.
    """
    names = {}
    taken = {}
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


# ─────────────────────────────────────────────────────────────────────────────
# Schema loading and $ref resolution
# ─────────────────────────────────────────────────────────────────────────────

class Schemas:
    def __init__(self, root):
        self.root = root
        self._defs = {}
        self._endpoints = {}

    def load_definitions(self, fname):
        if fname not in self._defs:
            self._defs[fname] = json.loads((self.root / "Definitions" / fname).read_text())
        return self._defs[fname]

    def load_endpoint(self, uri):
        if uri not in self._endpoints:
            self._endpoints[uri] = json.loads((self.root / f"{uri}.json").read_text())
        return self._endpoints[uri]

    @staticmethod
    def _walk(doc, pointer):
        node = doc
        for part in pointer.strip("/").split("/"):
            if part == "":
                continue
            part = part.replace("~1", "/").replace("~0", "~")
            if not isinstance(node, dict) or part not in node:
                return None
            node = node[part]
        return node

    def resolve(self, ref, doc, local_root=None):
        """Resolve a $ref. Returns the target schema dict, or None if dangling."""
        fname, _, pointer = ref.partition("#")
        if fname:
            if fname.endswith("definitions.json"):
                target = self.load_definitions(fname)
            else:
                target = json.loads((self.root / fname).read_text())
            return self._walk(target, pointer)
        node = self._walk(doc, pointer)
        if node is None and local_root is not None:
            # Some Wwise schemas use pointers relative to the args/result
            # schema instead of the document root.
            node = self._walk(local_root, pointer)
        return node


class Ctx:
    """Per-endpoint generation context."""

    def __init__(self, schemas, uri):
        self.schemas = schemas
        self.uri = uri
        self.doc = schemas.load_endpoint(uri)
        self.local_root = None


# ─────────────────────────────────────────────────────────────────────────────
# Type mapping
# ─────────────────────────────────────────────────────────────────────────────

def rust_type(sch, ctx, self_name=None):
    if not isinstance(sch, dict):
        return "serde_json::Value"
    if "$ref" in sch:
        ref = sch["$ref"]
        if ref in REF_TYPES:
            return REF_TYPES[ref]
        if ref == "#" and self_name:
            return self_name
        resolved = ctx.schemas.resolve(ref, ctx.doc, ctx.local_root)
        if resolved is None:
            return "serde_json::Value"
        return rust_type(resolved, ctx, self_name)
    t = sch.get("type")
    if t == "string":
        return "String"
    if t == "integer":
        return "i64"
    if t == "number":
        return "f64"
    if t == "boolean":
        return "bool"
    if t == "array":
        return f"Vec<{rust_type(sch.get('items', {}), ctx)}>"
    if t == "object":
        props = sch.get("properties", {})
        if "id" in props and "name" in props:
            return "WwiseObjectRef"
        return "serde_json::Value"
    branches = sch.get("anyOf") or sch.get("oneOf")
    if branches:
        tys = {rust_type(b, ctx) for b in branches}
        if len(tys) == 1:
            return tys.pop()
    return "serde_json::Value"


def stringy_items(sch, ctx):
    """True if an array schema's items always resolve to plain strings."""
    items = sch.get("items", {})
    if not isinstance(items, dict):
        return False
    return rust_type(items, ctx) == "String"


# ─────────────────────────────────────────────────────────────────────────────
# Rust emission helpers
# ─────────────────────────────────────────────────────────────────────────────

def doc_lines(text, prefix=""):
    text = clean_doc(text)
    if not text:
        return []
    return [f"{prefix}/// {line}" for line in text.split(". ") if line]


class Field:
    def __init__(self, wire, rust_name, ty, optional, doc=""):
        self.wire = wire
        self.rust_name = rust_name
        self.ty = ty
        self.optional = optional
        self.doc = doc

    def serde_attrs(self, kind):
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

    def declaration(self, kind):
        if self.optional and kind == "response" and self.ty.startswith("Vec<"):
            ty = self.ty  # relies on #[serde(default)]
        elif self.optional and not self.ty.startswith("Option<"):
            ty = f"Option<{self.ty}>"
        else:
            ty = self.ty
        lines = doc_lines(self.doc, "    ")
        lines += [f"    {a}" for a in self.serde_attrs(kind)]
        lines.append(f"    pub {self.rust_name}: {ty},")
        return lines


def field_name(uri, wire, kind):
    if kind == "response" and (uri, wire) in RESULT_FIELD_NAMES:
        return RESULT_FIELD_NAMES[(uri, wire)]
    if wire in FIELD_NAME_OVERRIDES:
        return FIELD_NAME_OVERRIDES[wire]
    return snake_case(wire)


def collect_fields(ctx, schema, kind, struct_name):
    fields = []
    for wire, sub in (schema.get("properties") or {}).items():
        ty = PROP_TYPE_OVERRIDES.get((ctx.uri, wire))
        if ty is None:
            ty = rust_type(sub, ctx, self_name=struct_name)
        optional = wire not in (schema.get("required") or [])
        fields.append(Field(wire, field_name(ctx.uri, wire, kind), ty, optional, sub.get("description", "")))
    return fields


def emit_struct(kind, struct_name, doc, fields, ctx, ctor_order=None, required_order=None, extra_flatten=False):
    """Emit an Args/Options (Serialize) or Response (Serialize+Deserialize) struct."""
    lines = doc_lines(doc) or [f"/// `{ctx.uri}` {kind}."]
    if kind == "response":
        lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    elif all(f.optional for f in fields):
        lines.append("#[derive(Debug, Clone, Default, serde::Serialize)]")
    else:
        lines.append("#[derive(Debug, Clone, serde::Serialize)]")
    lines.append(f"pub struct {struct_name} {{")
    for f in fields:
        lines += f.declaration(kind)
    if extra_flatten:
        lines.append("    #[serde(flatten)]")
        lines.append("    pub extra: std::collections::HashMap<String, serde_json::Value>,")
    lines.append("}")
    lines.append("")

    required = [f for f in fields if not f.optional]
    optional = [f for f in fields if f.optional]
    if kind == "response":
        return "\n".join(lines)

    lines.append(f"impl {struct_name} {{")
    if required:
        order = ctor_order or required_order or [f.wire for f in required]
        by_wire = {f.wire: f for f in required}
        ctor_fields = [by_wire[w] for w in order]
        params = ", ".join(f"{f.rust_name}: {f.ty}" for f in ctor_fields)
        lines.append(f"    pub fn new({params}) -> Self {{")
        lines.append("        Self {")
        for f in ctor_fields:
            lines.append(f"            {f.rust_name},")
        for f in optional:
            lines.append(f"            {f.rust_name}: None,")
        if extra_flatten:
            lines.append("            extra: std::collections::HashMap::new(),")
        lines.append("        }")
        lines.append("    }")
    elif kind != "response":
        lines.append("    pub fn new() -> Self {")
        lines.append("        Self::default()")
        lines.append("    }")
    if kind != "response":
        for f in optional:
            lines.append("")
            lines.append(f"    pub fn {f.rust_name}(mut self, {f.rust_name}: impl Into<{f.ty}>) -> Self {{")
            lines.append(f"        self.{f.rust_name} = Some({f.rust_name}.into());")
            lines.append("        self")
            lines.append("    }")
        if extra_flatten:
            lines.append("")
            lines.append("    /// Set an arbitrary `@`-prefixed property, reference, or list field.")
            lines.append("    /// The key must include the `@` prefix, e.g. `\"@Volume\"`.")
            lines.append("    pub fn extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {")
            lines.append("        self.extra.insert(key.into(), value.into());")
            lines.append("        self")
            lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


def emit_enum(name, doc, values, derive_both=True):
    """Emit a simple string enum with explicit serde renames."""
    lines = doc_lines(doc)
    derives = "serde::Serialize, serde::Deserialize" if derive_both else "serde::Serialize"
    lines.append(f"#[derive(Debug, Clone, Copy, {derives})]")
    lines.append(f"pub enum {name} {{")
    for v, variant in unique_names(values, pascal_case, "Legacy").items():
        lines.append(f'    #[serde(rename = "{v}")]')
        lines.append(f"    {variant},")
    lines.append("}")
    return "\n".join(lines)


# ─────────────────────────────────────────────────────────────────────────────
# primitives.rs
# ─────────────────────────────────────────────────────────────────────────────

def gen_object_return_field(waapi_defs):
    enum_vals = waapi_defs["definitions"]["returnExpression"]["items"]["anyOf"][0]["enum"]
    lines = doc_lines("A field that can be requested in `return` options.")
    lines.append("#[derive(Debug, Clone)]")
    lines.append("pub enum ObjectReturnField {")
    arms = []
    for v, variant in unique_names(enum_vals, pascal_case, "Legacy").items():
        lines.append(f"    {variant},")
        arms.append(f"            ObjectReturnField::{variant} => \"{v}\",")
    lines.append("    /// Any other accessor, e.g. `@Property` or a dot-separated query.")
    lines.append("    Custom(String),")
    lines.append("}")
    lines.append("")
    lines.append("impl serde::Serialize for ObjectReturnField {")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_str(match self {")
    lines += arms
    lines.append("            ObjectReturnField::Custom(value) => value.as_str(),")
    lines.append("        })")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<&str> for ObjectReturnField {")
    lines.append("    fn from(s: &str) -> Self { ObjectReturnField::Custom(s.to_string()) }")
    lines.append("}")
    lines.append("impl From<String> for ObjectReturnField {")
    lines.append("    fn from(s: String) -> Self { ObjectReturnField::Custom(s) }")
    lines.append("}")
    return "\n".join(lines)


def gen_wwise_object(ctx):
    waapi_defs = ctx.schemas.load_definitions("waapi_definitions.json")
    obj_return = waapi_defs["definitions"]["objectReturn"]
    props = obj_return.get("properties", {})
    names = unique_names(props.keys(), lambda w: field_name(ctx.uri, w, "response"), "_legacy")
    fields = []
    for wire, sub in props.items():
        ty = rust_type(sub, ctx)
        fields.append(Field(wire, names[wire], ty, True, sub.get("description", "")))
    lines = [
        "/// A Wwise object as returned by `object.get` and similar endpoints.",
        "/// Fields are optional because their presence depends on the `return` options;",
        "/// accessors not listed here (e.g. `@Property` values) land in `extra`.",
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        "pub struct WwiseObject {",
    ]
    for f in fields:
        lines += f.declaration("response")
    lines.append("    #[serde(flatten)]")
    lines.append("    pub extra: std::collections::HashMap<String, serde_json::Value>,")
    lines.append("}")
    return "\n".join(lines)


def gen_object_query(ctx):
    args = ctx.doc["argsSchema"]
    from_variants = args["properties"]["from"]["oneOf"]
    variants = []
    for v in from_variants:
        prop = list(v["properties"].keys())[0]
        sub = v["properties"][prop]
        item_ty = "String" if stringy_items(sub, ctx) else "serde_json::Value"
        variants.append((prop, pascal_case(prop), item_ty))
    lines = ["/// Argument to the `from` field of `object.get`."]
    lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    lines.append("pub enum ObjectQuery {")
    for wire, variant, item_ty in variants:
        lines.append(f'    #[serde(rename = "{wire}")]')
        lines.append(f"    {variant}(Vec<{item_ty}>),")
    lines.append("}")
    lines.append("")
    lines.append("impl From<&str> for ObjectQuery {")
    lines.append("    fn from(s: &str) -> Self { ObjectQuery::Path(vec![s.to_string()]) }")
    lines.append("}")
    lines.append("impl From<String> for ObjectQuery {")
    lines.append("    fn from(s: String) -> Self { ObjectQuery::Path(vec![s]) }")
    lines.append("}")
    lines.append("impl From<Vec<String>> for ObjectQuery {")
    lines.append("    fn from(v: Vec<String>) -> Self {")
    lines.append("        ObjectQuery::Id(v.into_iter().map(serde_json::Value::String).collect())")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


def gen_transform_select(ctx):
    args = ctx.doc["argsSchema"]
    select_variant = args["properties"]["transform"]["items"]["oneOf"][0]
    values = select_variant["properties"]["select"]["items"]["enum"]
    return emit_enum("TransformSelect", "Selector for an `object.get` select transformation.", values)


def gen_import_types(ctx):
    args = ctx.doc["argsSchema"]
    ctx.local_root = args
    item_fields = collect_fields(ctx, args["properties"]["imports"]["items"], "args", "ImportItem")
    default_fields = collect_fields(ctx, args["properties"]["default"], "args", "ImportDefaults")

    result = ctx.doc["resultSchema"]
    ctx.local_root = result
    log_fields = collect_fields(ctx, result["properties"]["log"]["items"], "response", "ImportLogEntry")
    ctx.local_root = None

    out = emit_struct("args", "ImportItem", "One item in the `imports` array for `audio.import`.",
                      item_fields, ctx, extra_flatten=True)
    out += "\n\n"
    out += emit_struct("args", "ImportDefaults", "Default values applied to every item in `audio.import`.",
                       default_fields, ctx, extra_flatten=True)
    out += "\n\n"
    # ImportLogEntry needs both Serialize and Deserialize.
    lines = ["/// One entry in the audio import log."]
    lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    lines.append("pub struct ImportLogEntry {")
    for f in log_fields:
        lines += f.declaration("response")
    lines.append("}")
    out += "\n".join(lines)
    return out


def gen_version_info(ctx):
    result = ctx.doc["resultSchema"]
    out = emit_struct("response", "VersionInfo", "Wwise version information returned by `getInfo`.",
                      collect_fields(ctx, result["properties"]["version"], "response", "VersionInfo"), ctx)
    out += "\n\n"
    out += emit_struct("response", "DirectoriesInfo", "Directories used by Wwise, returned by `getInfo`.",
                       collect_fields(ctx, result["properties"]["directories"], "response", "DirectoriesInfo"), ctx)
    # Response structs with all-optional fields should derive Default.
    out = out.replace("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\npub struct DirectoriesInfo",
                      "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]\npub struct DirectoriesInfo")
    return out


def gen_primitives(schemas, endpoints_docs):
    waapi_defs = schemas.load_definitions("waapi_definitions.json")
    get_ctx = endpoints_docs["ak.wwise.core.object.get"]
    info_ctx = endpoints_docs["ak.wwise.core.getInfo"]
    import_ctx = endpoints_docs["ak.wwise.core.audio.import"]

    import_operation_vals = [
        b["enum"][0] for b in waapi_defs["definitions"]["importOperationType"]["oneOf"]
    ]
    log_severity_vals = [
        b["enum"][0] for b in waapi_defs["definitions"]["logCentralSeverity"]["oneOf"]
    ]

    parts = [HEADER]
    parts.append("pub use crate::api::primitives::{")
    parts.append("    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, PropertyValue,")
    parts.append("};")
    parts.append("")
    parts.append(STATIC_REFS)
    parts.append(gen_object_return_field(waapi_defs))
    parts.append(STATIC_OBJECT_RETURN_OPTIONS)
    parts.append(emit_enum("BusReturnField", "Return fields for `ak.wwise.core.profiler.getBusses`.",
                           waapi_defs["definitions"]["busReturnOptions"]["items"]["enum"]))
    parts.append(emit_enum("AudioObjectReturnField", "Return fields for `ak.wwise.core.profiler.getAudioObjects`.",
                           waapi_defs["definitions"]["audioObjectReturnOptions"]["items"]["enum"]))
    parts.append(emit_enum("VoiceReturnField", "Return fields for `ak.wwise.core.profiler.getVoices`.",
                           waapi_defs["definitions"]["voiceReturnOptions"]["items"]["enum"]))
    parts.append(emit_enum("ImportOperation", "Determines how imported objects are created.", import_operation_vals))
    parts.append(emit_enum("LogSeverity", "Severity level of an audio import log entry.", log_severity_vals))
    parts.append(gen_wwise_object(get_ctx))
    parts.append(STATIC_WWISE_OBJECT_REF)
    parts.append(gen_object_query(get_ctx))
    parts.append(gen_transform_select(get_ctx))
    parts.append(STATIC_TRANSFORM)
    parts.append(gen_version_info(info_ctx))
    parts.append(gen_import_types(import_ctx))
    return "\n\n".join(parts) + "\n"


HEADER = """// Shared primitives referenced by generated typed bindings.
// This file is generated by tools/generate_typed_api.py; do not edit by hand.
"""

STATIC_REFS = """/// Object reference accepted by soundengine endpoints: name, GUID, or Short ID.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SoundEngineObjectRef {
    Name(String),
    Guid(String),
    ShortId(u32),
}

impl From<&str> for SoundEngineObjectRef {
    fn from(s: &str) -> Self { SoundEngineObjectRef::Name(s.to_string()) }
}
impl From<String> for SoundEngineObjectRef {
    fn from(s: String) -> Self { SoundEngineObjectRef::Name(s) }
}
impl From<u32> for SoundEngineObjectRef {
    fn from(id: u32) -> Self { SoundEngineObjectRef::ShortId(id) }
}

/// Event reference accepted by `soundengine.postEvent`.
pub type EventRef = SoundEngineObjectRef;

/// RTPC reference accepted by `soundengine.setRTPCValue`.
pub type RtpcRef = SoundEngineObjectRef;

/// Game object reference accepted by soundengine calls.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum GameObjectRef {
    Id(u64),
}

impl From<u64> for GameObjectRef {
    fn from(id: u64) -> Self { GameObjectRef::Id(id) }
}
impl From<u32> for GameObjectRef {
    fn from(id: u32) -> Self { GameObjectRef::Id(id as u64) }
}

/// Language reference used in return options.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LanguageRef {
    Name(String),
    Guid(String),
}

impl From<&str> for LanguageRef {
    fn from(s: &str) -> Self { LanguageRef::Name(s.to_string()) }
}
impl From<String> for LanguageRef {
    fn from(s: String) -> Self { LanguageRef::Name(s) }
}"""

STATIC_OBJECT_RETURN_OPTIONS = """/// Options controlling which fields are returned for each object.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ObjectReturnOptions {
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageRef>,
}

impl ObjectReturnOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_fields<I, T>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields = Some(fields.into_iter().map(Into::into).collect());
        self
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn language(mut self, language: impl Into<LanguageRef>) -> Self {
        self.language = Some(language.into());
        self
    }
}"""

STATIC_WWISE_OBJECT_REF = """/// A lightweight object reference returned inside other objects.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WwiseObjectRef {
    #[serde(default, rename = "id")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}"""

STATIC_TRANSFORM = """/// A single transformation step in an `object.get` query.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Transform {
    Select {
        select: Vec<TransformSelect>,
    },
    Distinct(String),
    Range {
        range: Vec<f64>,
    },
    Where {
        #[serde(rename = "where")]
        where_: Vec<serde_json::Value>,
    },
}"""


# ─────────────────────────────────────────────────────────────────────────────
# Endpoint files
# ─────────────────────────────────────────────────────────────────────────────

def endpoint_segments(uri):
    return uri.split(".")


def endpoint_base_name(uri):
    return "".join(pascal_case(s) for s in endpoint_segments(uri)[1:])


def endpoint_file_stem(uri):
    return snake_case(endpoint_segments(uri)[-1])


def endpoint_module_path(uri):
    return endpoint_segments(uri)[:-1]


def method_name(uri):
    return snake_case(endpoint_segments(uri)[-1])


def is_empty_schema(sch):
    return not sch.get("properties") and "$ref" not in sch


def gen_endpoint_file(ctx):
    base = endpoint_base_name(ctx.uri)
    args_schema = ctx.doc.get("argsSchema", {})
    options_schema = ctx.doc.get("optionsSchema", {})
    result_schema = ctx.doc.get("resultSchema", {})

    has_args = not is_empty_schema(args_schema)
    has_options = not is_empty_schema(options_schema)
    has_response = not is_empty_schema(result_schema)
    options_is_object_return = options_schema.get("$ref") == OBJECT_RETURN_OPTIONS_REF

    sections = [f"// Generated bindings for {ctx.uri}", "", "#![allow(unused_imports)]",
                "#![allow(dead_code)]", "", "use crate::api::generated::primitives::*;", ""]
    exports = []

    if has_args:
        name = f"{base}Args"
        exports.append(name)
        if ctx.uri == "ak.wwise.core.object.get":
            sections.append(gen_object_get_args(ctx, name))
        elif ctx.uri == "ak.wwise.core.object.set":
            item_name = f"{base}Item"
            exports.append(item_name)
            sections.append(gen_object_set_args(ctx, name, item_name))
        else:
            ctx.local_root = args_schema
            fields = collect_fields(ctx, args_schema, "args", name)
            ctx.local_root = None
            extra_flatten = bool(args_schema.get("patternProperties"))
            sections.append(emit_struct("args", name, f"Arguments for `{ctx.uri}`.",
                                        fields, ctx, ctor_order=CTOR_ORDER.get(ctx.uri),
                                        required_order=args_schema.get("required"),
                                        extra_flatten=extra_flatten))
    if has_options:
        name = f"{base}Options"
        exports.append(name)
        if options_is_object_return:
            sections.append(f"/// Options for `{ctx.uri}`.")
            sections.append(f"pub type {name} = ObjectReturnOptions;")
        else:
            ctx.local_root = options_schema
            fields = collect_fields(ctx, options_schema, "options", name)
            ctx.local_root = None
            sections.append(emit_struct("options", name, f"Options for `{ctx.uri}`.", fields, ctx))
    if has_response:
        name = f"{base}Response"
        exports.append(name)
        if ctx.uri == "ak.wwise.core.object.set":
            item_name = f"{base}Object"
            exports.append(item_name)
            sections.append(gen_object_set_response(ctx, name, item_name))
        else:
            ctx.local_root = result_schema
            fields = collect_fields(ctx, result_schema, "response", name)
            ctx.local_root = None
            sections.append(emit_struct("response", name, f"Response from `{ctx.uri}`.", fields, ctx))

    for new_name, aliases in COMPAT_ALIASES.items():
        if new_name in exports:
            for alias in aliases:
                exports.append(alias)
                sections.append(f"pub type {alias} = {new_name};")

    return "\n\n".join(sections) + "\n", exports, has_args, has_options, has_response


def gen_object_get_args(ctx, name):
    return f"""/// Arguments for `ak.wwise.core.object.get`. Specify either a WAQL query or
/// a `from` query; both are optional in the schema but one is required in practice.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct {name} {{
    /// A WAQL query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waql: Option<String>,
    /// The query starting point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<ObjectQuery>,
    /// Sequential transformations on the object list returned by `from`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
}}

impl {name} {{
    pub fn new(from: ObjectQuery) -> Self {{
        Self {{
            waql: None,
            from: Some(from),
            transform: None,
        }}
    }}

    pub fn from_waql(waql: impl Into<String>) -> Self {{
        Self {{
            waql: Some(waql.into()),
            from: None,
            transform: None,
        }}
    }}

    pub fn waql(mut self, waql: impl Into<String>) -> Self {{
        self.waql = Some(waql.into());
        self
    }}

    pub fn from(mut self, from: impl Into<ObjectQuery>) -> Self {{
        self.from = Some(from.into());
        self
    }}

    pub fn transform(mut self, transform: impl Into<Vec<Transform>>) -> Self {{
        self.transform = Some(transform.into());
        self
    }}
}}"""


# Field type overrides for the nested `objects` items of `object.set`.
OBJECT_SET_ITEM_OVERRIDES = {
    "children": "Vec<ChildObject>",
    "onNameConflict": "OnNameConflict",
    "listMode": "ListMode",
}


def gen_object_set_args(ctx, args_name, item_name):
    """Emit the `objects` item struct and the top-level args struct."""
    args_schema = ctx.doc["argsSchema"]
    item_schema = args_schema["properties"]["objects"]["items"]
    ctx.local_root = args_schema
    fields = []
    for wire, sub in (item_schema.get("properties") or {}).items():
        ty = OBJECT_SET_ITEM_OVERRIDES.get(wire)
        if ty is None:
            ty = rust_type(sub, ctx, self_name=item_name)
        optional = wire not in (item_schema.get("required") or [])
        fields.append(Field(wire, field_name(ctx.uri, wire, "args"), ty, optional,
                            sub.get("description", "")))
    out = emit_struct("args", item_name,
                      "One object specification in the `objects` array of `object.set`.",
                      fields, ctx, required_order=item_schema.get("required"),
                      extra_flatten=bool(item_schema.get("patternProperties")))
    out += "\n\n"
    out += f"""impl {item_name} {{
    /// Set a `@`-prefixed property, reference, or list field.
    /// Pass the bare name without the `@` prefix, e.g. `"Volume"`.
    pub fn property(mut self, name: impl Into<String>, value: impl serde::Serialize) -> Self {{
        self.extra.insert(
            format!("@{{}}", name.into()),
            serde_json::to_value(value).expect("value must serialize to JSON"),
        );
        self
    }}
}}"""
    fields = collect_fields(ctx, args_schema, "args", args_name)
    ctx.local_root = None
    for f in fields:
        if f.wire == "objects":
            f.ty = f"Vec<{item_name}>"
    out += "\n\n"
    out += emit_struct("args", args_name, f"Arguments for `{ctx.uri}`.", fields, ctx,
                       required_order=args_schema.get("required"))
    return out


def gen_object_set_response(ctx, resp_name, item_name):
    """Emit the response `objects` item struct and the response struct."""
    result_schema = ctx.doc["resultSchema"]
    item_schema = result_schema["properties"]["objects"]["items"]
    ctx.local_root = result_schema
    fields = collect_fields(ctx, item_schema, "response", item_name)
    out = emit_struct("response", item_name,
                      "One object entry in the `object.set` response.",
                      fields, ctx,
                      extra_flatten=bool(item_schema.get("patternProperties")))
    fields = collect_fields(ctx, result_schema, "response", resp_name)
    ctx.local_root = None
    for f in fields:
        if f.wire == "objects":
            f.ty = f"Vec<{item_name}>"
    out += "\n\n"
    out += emit_struct("response", resp_name, f"Response from `{ctx.uri}`.", fields, ctx)
    return out


# ─────────────────────────────────────────────────────────────────────────────
# Module tree
# ─────────────────────────────────────────────────────────────────────────────

def build_module_tree(endpoints):
    tree = {}
    for uri, info in endpoints.items():
        node = tree
        for seg in endpoint_module_path(uri):
            node = node.setdefault(seg, {})
        node.setdefault("__endpoints__", []).append((uri, info))
    return tree


def emit_module_tree(tree, dir_path):
    lines = ["// Auto-generated typed WAAPI bindings.",
             "// This file is generated by tools/generate_typed_api.py; do not edit by hand.", ""]
    for name in sorted(k for k in tree if k != "__endpoints__"):
        child_dir = dir_path / name
        child_dir.mkdir(parents=True, exist_ok=True)
        emit_module_tree(tree[name], child_dir)
        lines.append(f"pub mod {name};")
    for uri, info in sorted(tree.get("__endpoints__", [])):
        stem = endpoint_file_stem(uri)
        (dir_path / f"{stem}.rs").write_text(info["content"])
        lines.append(f"mod {stem};")
        lines.append(f"pub use {stem}::{{")
        for e in info["exports"]:
            lines.append(f"    {e},")
        lines.append("};")
    (dir_path / "mod.rs").write_text("\n".join(lines) + "\n")


# ─────────────────────────────────────────────────────────────────────────────
# Client code (chained namespace clients)
# ─────────────────────────────────────────────────────────────────────────────

def endpoint_call_shape(info):
    has_args, has_options, has_response = info["has_args"], info["has_options"], info["has_response"]
    params = []
    if has_args:
        params.append(("args", f"{info['base']}Args"))
    if has_options:
        params.append(("options", f"{info['base']}Options"))
    return params, has_response


def emit_call_body(uri, info, target="self.client"):
    has_args, has_options, has_response = info["has_args"], info["has_options"], info["has_response"]
    method = "call_typed" if has_response else "call_typed_void"

    type_args = []
    type_args.append("_" if has_args else "()")
    type_args.append("_" if has_options else "()")
    if has_response:
        type_args.append("_")

    turbofish = ""
    if any(t != "_" for t in type_args):
        turbofish = f"::<{', '.join(type_args)}>"

    args_expr = "Some(args)" if has_args else "None"
    options_expr = "Some(options)" if has_options else "None"
    uri_variant = f"WaapiFunction::{uri.replace('.', '_')}"
    call = f"{target}.{method}{turbofish}({uri_variant}, {args_expr}, {options_expr}).await"

    if has_response:
        return [call]
    else:
        return [f"{call}?;", "Ok(())"]


def use_path(uri, types):
    mod_path = "::".join(endpoint_module_path(uri))
    return f"use crate::api::generated::{mod_path}::{{{', '.join(types)}}};"


def client_struct_name(path_segments):
    if path_segments == ("ak",):
        return "AkClient"
    return "".join(pascal_case(s) for s in path_segments[1:]) + "Client"


def build_chain_tree(uris):
    """Build an ordered list of (struct_name, prefix, [(method, child_struct)]) from endpoint URIs."""
    children_by_path = {}
    all_paths = []

    for uri in uris:
        mod_segs = tuple(endpoint_module_path(uri))
        for depth in range(1, len(mod_segs) + 1):
            prefix_segs = mod_segs[:depth]
            if prefix_segs not in children_by_path:
                children_by_path[prefix_segs] = []
                all_paths.append(prefix_segs)
            if depth < len(mod_segs):
                next_seg = mod_segs[depth]
                child_segs = mod_segs[: depth + 1]
                entry = (snake_case(next_seg), client_struct_name(child_segs))
                if entry not in children_by_path[prefix_segs]:
                    children_by_path[prefix_segs].append(entry)

    # Sort breadth-first by depth while preserving discovery order at each depth.
    all_paths.sort(key=len)
    return [
        (client_struct_name(p), ".".join(p), children_by_path[p])
        for p in all_paths
    ]


def gen_clients(endpoints):
    lines = ["// Auto-generated chained namespace clients.",
             "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
             "", "use crate::{WaapiClient, WaapiFunction, WaapiResult};",
             ""]
    for uri in sorted(endpoints):
        info = endpoints[uri]
        types = [f"{info['base']}Args"] if info["has_args"] else []
        if info["has_options"]:
            types.append(f"{info['base']}Options")
        if info["has_response"]:
            types.append(f"{info['base']}Response")
        lines.append(use_path(uri, types))
    lines.append("")

    by_prefix = {}
    for uri in PILOT_ENDPOINTS:
        prefix = ".".join(endpoint_module_path(uri))
        by_prefix.setdefault(prefix, []).append(uri)

    for struct_name, prefix, children in build_chain_tree(PILOT_ENDPOINTS):
        lines.append(f"pub struct {struct_name}<'a> {{")
        lines.append("    client: &'a mut WaapiClient,")
        lines.append("}")
        lines.append("")
        lines.append(f"impl<'a> {struct_name}<'a> {{")
        for method, child in children:
            lines.append(f"    pub fn {method}(self) -> {child}<'a> {{")
            lines.append(f"        {child} {{ client: self.client }}")
            lines.append("    }")
            lines.append("")
        for uri in by_prefix.get(prefix or "", []):
            info = endpoints[uri]
            params, has_response = endpoint_call_shape(info)
            ret = f"WaapiResult<{info['base']}Response>" if has_response else "WaapiResult<()>"
            param_list = ",\n        ".join(f"{n}: {t}" for n, t in params)
            sig = f"        &mut self,\n        {param_list}," if params else "        &mut self,"
            lines.append(f"    /// `{uri}`.")
            lines.append(f"    pub async fn {method_name(uri)}(")
            lines.append(sig)
            lines.append(f"    ) -> {ret} {{")
            body = emit_call_body(uri, info)
            lines += [f"        {b}" for b in body]
            lines.append("    }")
            lines.append("")
        if lines[-1] == "":
            lines.pop()
        lines.append("}")
        lines.append("")

    lines.append("impl WaapiClient {")
    lines.append("    /// Chained namespace access starting at `ak`.")
    lines.append("    pub fn ak(&mut self) -> AkClient<'_> {")
    lines.append("        AkClient { client: self }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Shortcut for `client.ak().wwise()`.")
    lines.append("    pub fn wwise(&mut self) -> WwiseClient<'_> {")
    lines.append("        WwiseClient { client: self }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Shortcut for `client.ak().soundengine()`.")
    lines.append("    pub fn soundengine(&mut self) -> SoundengineClient<'_> {")
    lines.append("        SoundengineClient { client: self }")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


# ─────────────────────────────────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────────────────────────────────

def main():
    schemas = Schemas(SCHEMA_DIR)
    endpoints = {}
    for uri in PILOT_ENDPOINTS:
        ctx = Ctx(schemas, uri)
        content, exports, has_args, has_options, has_response = gen_endpoint_file(ctx)
        endpoints[uri] = {
            "content": content,
            "exports": exports,
            "has_args": has_args,
            "has_options": has_options,
            "has_response": has_response,
            "base": endpoint_base_name(uri),
            "ctx": ctx,
        }

    if OUT_DIR.exists():
        shutil.rmtree(OUT_DIR)
    OUT_DIR.mkdir(parents=True)

    tree = build_module_tree(endpoints)
    emit_module_tree(tree, OUT_DIR)

    ctxs = {uri: info["ctx"] for uri, info in endpoints.items()}
    (OUT_DIR / "primitives.rs").write_text(gen_primitives(schemas, ctxs))
    (OUT_DIR / "clients.rs").write_text(gen_clients(endpoints))
    (OUT_DIR / "mod.rs").write_text(
        "// Auto-generated typed WAAPI bindings.\n"
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.\n\n"
        "pub mod ak;\n"
        "pub mod clients;\n"
        "pub mod primitives;\n"
    )
    print(f"Generated {len(PILOT_ENDPOINTS)} endpoints into {OUT_DIR}")


if __name__ == "__main__":
    main()
