"""Rust struct, enum, primitive, endpoint, topic, and module tree emission."""

from __future__ import annotations

import json
import re
import textwrap
import xml.etree.ElementTree as ET

from .config import (
    CTOR_ORDER,
    OBJECT_RETURN_OPTIONS_REF,
    OBJECT_RETURN_REF,
    PROP_TYPE_OVERRIDES,
    RESULT_FIELD_NAMES,
    RESULT_WRAPPED_OBJECT_RETURN,
    RUST_KEYWORDS,
    SCHEMA_DIRS,
)
from .schema_loader import (
    Ctx,
    Schemas,
    _compose_struct_doc,
    clean_doc,
    doc_lines,
    extract_string_enum_docs,
    schema_description,
    schema_required,
    union_definition_enum_values,
)
from .type_mapper import (
    Field,
    collect_fields,
    endpoint_base_name,
    field_name,
    load_object_type_entries,
    load_property_entries,
    pascal_case,
    property_variant_name,
    rust_type,
    snake_case,
    stringy_items,
    unique_names,
)


INTO_CTOR_TYPES = {
    "String",
    "ObjectRef",
    "GameObjectRef",
    "SoundEngineObjectRef",
    "EventRef",
    "RtpcRef",
    "PlatformRef",
    "LanguageRef",
    "PropertyValue",
    "ObjectQuery",
}


def emit_flat_map_deserialize(struct_name, fields):
    """Emit a single-pass MapAccess Deserialize impl that avoids #[serde(flatten)] Content buffering."""
    lines = [
        f"impl<'de> serde::Deserialize<'de> for {struct_name} {{",
        "    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {",
        f"        struct {struct_name}Visitor;",
        "",
        f"        impl<'de> serde::de::Visitor<'de> for {struct_name}Visitor {{",
        f"            type Value = {struct_name};",
        "",
        "            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {",
        f'                formatter.write_str("struct {struct_name}")',
        "            }",
        "",
        "            fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {",
    ]
    for f in fields:
        lines.append(f"                let mut {f.rust_name}: Option<{f.ty}> = None;")
    lines.append(
        "                let mut extra = std::collections::HashMap::<String, serde_json::Value>::new();"
    )
    lines.append(
        "                while let Some(key) = map.next_key::<std::borrow::Cow<'de, str>>()? {"
    )
    lines.append("                    match key.as_ref() {")
    for f in fields:
        if f.optional:
            lines.append(
                f'                        "{f.wire}" => {f.rust_name} = map.next_value()?,',
            )
        else:
            lines.append(
                f'                        "{f.wire}" => {f.rust_name} = Some(map.next_value()?),',
            )
    lines.append("                        _ => {")
    lines.append("                            extra.insert(key.into_owned(), map.next_value()?);")
    lines.append("                        }")
    lines.append("                    }")
    lines.append("                }")
    lines.append(f"                Ok({struct_name} {{")
    for f in fields:
        if f.optional and f.ty.startswith("Vec<"):
            lines.append(f"                    {f.rust_name}: {f.rust_name}.unwrap_or_default(),")
        elif f.optional:
            lines.append(f"                    {f.rust_name},")
        else:
            lines.append(
                f'                    {f.rust_name}: {f.rust_name}.ok_or_else(|| serde::de::Error::missing_field("{f.wire}"))?,'
            )
    lines.append("                    extra,")
    lines.append("                })")
    lines.append("            }")
    lines.append("        }")
    lines.append("")
    lines.append(f"        deserializer.deserialize_map({struct_name}Visitor)")
    lines.append("    }")
    lines.append("}")
    return lines


def emit_struct(kind, struct_name, doc, fields, ctx, ctor_order=None, required_order=None, extra_flatten=False):
    """Emit an Args/Options (Serialize) or Response (Serialize+Deserialize) struct."""
    lines = doc_lines(doc) or [f"/// `{ctx.uri}` {kind}."]
    if kind == "response":
        if extra_flatten:
            lines.append("#[derive(Debug, Clone, serde::Serialize)]")
        else:
            lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    elif all(f.optional for f in fields):
        lines.append("#[derive(Debug, Clone, Default, serde::Serialize)]")
    else:
        lines.append("#[derive(Debug, Clone, serde::Serialize)]")
    lines.append(f"pub struct {struct_name} {{")
    for f in fields:
        lines += f.declaration(kind)
    if extra_flatten:
        lines.append("    /// Additional `@`-prefixed properties, references, or list values.")
        lines.append("    #[serde(flatten)]")
        lines.append("    pub extra: std::collections::HashMap<String, serde_json::Value>,")
    lines.append("}")
    lines.append("")

    required = [f for f in fields if not f.optional]
    optional = [f for f in fields if f.optional]
    if kind == "response":
        has_opt_id = any(f.rust_name == "id" and f.ty == "String" and f.optional for f in fields)
        has_opt_name = any(f.rust_name == "name" and f.ty == "String" and f.optional for f in fields)
        if has_opt_id or has_opt_name or extra_flatten:
            lines.append(f"impl {struct_name} {{")
            method_blocks = []
            if has_opt_id:
                method_blocks.append([
                    "    /// Returns the object GUID string (`id`), if present in the response.",
                    "    pub fn id(&self) -> Option<&str> {",
                    "        self.id.as_deref()",
                    "    }",
                ])
            if has_opt_name:
                method_blocks.append([
                    "    /// Returns the object name (`name`), if present in the response.",
                    "    pub fn name(&self) -> Option<&str> {",
                    "        self.name.as_deref()",
                    "    }",
                ])
            if extra_flatten:
                method_blocks.append([
                    "    /// Look up a property value in `extra` (checks `\"@Property\"` first, then `\"Property\"`).",
                    "    pub fn property(&self, prop: Property) -> Option<&serde_json::Value> {",
                    "        self.extra",
                    "            .get(prop.as_at_str())",
                    "            .or_else(|| self.extra.get(prop.as_str()))",
                    "    }",
                    "",
                    "    /// Look up a numeric (`f64`) property value in `extra`.",
                    "    pub fn property_f64(&self, prop: Property) -> Option<f64> {",
                    "        self.property(prop).and_then(serde_json::Value::as_f64)",
                    "    }",
                    "",
                    "    /// Look up an integer (`i64`) property value in `extra`.",
                    "    pub fn property_i64(&self, prop: Property) -> Option<i64> {",
                    "        self.property(prop).and_then(serde_json::Value::as_i64)",
                    "    }",
                    "",
                    "    /// Look up a boolean (`bool`) property value in `extra`.",
                    "    pub fn property_bool(&self, prop: Property) -> Option<bool> {",
                    "        self.property(prop).and_then(serde_json::Value::as_bool)",
                    "    }",
                    "",
                    "    /// Look up a string (`&str`) property value in `extra`.",
                    "    pub fn property_str(&self, prop: Property) -> Option<&str> {",
                    "        self.property(prop).and_then(serde_json::Value::as_str)",
                    "    }",
                ])
            for idx, block in enumerate(method_blocks):
                if idx > 0:
                    lines.append("")
                lines.extend(block)
            lines.append("}")
        if extra_flatten:
            lines.append("")
            lines.extend(emit_flat_map_deserialize(struct_name, fields))
        return "\n".join(lines)

    ctor_fields = []
    lines.append(f"impl {struct_name} {{")
    if required:
        order = ctor_order or required_order or [f.wire for f in required]
        by_wire = {f.wire: f for f in required}
        ctor_fields = [by_wire[w] for w in order if w in by_wire]
        for f in required:
            if f not in ctor_fields:
                ctor_fields.append(f)
        params = ", ".join(
            f"{f.rust_name}: impl Into<{f.ty}>" if f.ty in INTO_CTOR_TYPES else f"{f.rust_name}: {f.ty}"
            for f in ctor_fields
        )
        lines.append(f"    /// Creates a new [`{struct_name}`] with the required fields.")
        lines.append(f"    pub fn new({params}) -> Self {{")
        lines.append("        Self {")
        for f in ctor_fields:
            if f.ty in INTO_CTOR_TYPES:
                lines.append(f"            {f.rust_name}: {f.rust_name}.into(),")
            else:
                lines.append(f"            {f.rust_name},")
        for f in optional:
            lines.append(f"            {f.rust_name}: None,")
        if extra_flatten:
            lines.append("            extra: std::collections::HashMap::new(),")
        lines.append("        }")
        lines.append("    }")
    elif kind != "response":
        lines.append(f"    /// Creates a new [`{struct_name}`] with all fields unset.")
        lines.append("    pub fn new() -> Self {")
        lines.append("        Self::default()")
        lines.append("    }")
    if kind != "response":
        for f in optional:
            lines.append("")
            lines += doc_lines(f.doc, "    ") or [f"    /// Sets `{f.wire}`."]
            if f.rust_name == "return_fields" and f.ty.startswith("Vec<") and f.ty.endswith(">"):
                elem_ty = f.ty[4:-1]
                lines.append(f"    pub fn {f.rust_name}<I, T>(mut self, {f.rust_name}: I) -> Self")
                lines.append("    where")
                lines.append("        I: IntoIterator<Item = T>,")
                lines.append(f"        T: Into<{elem_ty}>,")
                lines.append("    {")
                lines.append(f"        self.{f.rust_name} = Some({f.rust_name}.into_iter().map(Into::into).collect());")
                lines.append("        self")
                lines.append("    }")
                lines.append("")
                lines.append("    /// Alias for [`Self::return_fields`].")
                lines.append("    pub fn return_<I, T>(self, return_fields: I) -> Self")
                lines.append("    where")
                lines.append("        I: IntoIterator<Item = T>,")
                lines.append(f"        T: Into<{elem_ty}>,")
                lines.append("    {")
                lines.append("        self.return_fields(return_fields)")
                lines.append("    }")
            else:
                lines.append(f"    pub fn {f.rust_name}(mut self, {f.rust_name}: impl Into<{f.ty}>) -> Self {{")
                lines.append(f"        self.{f.rust_name} = Some({f.rust_name}.into());")
                lines.append("        self")
                lines.append("    }")
        if extra_flatten:
            lines.append("")
            lines.append("    /// Set a `@`-prefixed property, reference, or list field.")
            lines.append("    /// Accepts a [`Property`] variant (e.g. `Property::Volume`) or a bare property name (`\"Volume\"`).")
            lines.append("    pub fn property(mut self, name: impl AsRef<str>, value: impl serde::Serialize) -> Self {")
            lines.append("        let name = name.as_ref().strip_prefix('@').unwrap_or(name.as_ref());")
            lines.append("        self.extra.insert(")
            lines.append("            format!(\"@{name}\"),")
            lines.append("            serde_json::to_value(value).expect(\"value must serialize to JSON\"),")
            lines.append("        );")
            lines.append("        self")
            lines.append("    }")
            lines.append("")
            lines.append("    /// Set an arbitrary `@`-prefixed property, reference, or list field.")
            lines.append("    /// The key must include the `@` prefix, e.g. `\"@Volume\"`.")
            lines.append("    pub fn extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {")
            lines.append("        self.extra.insert(key.into(), value.into());")
            lines.append("        self")
            lines.append("    }")
    lines.append("}")

    if kind == "args" and len(ctor_fields) == 1:
        f = ctor_fields[0]
        from_types = []
        if f.ty != struct_name:
            from_types.append(f.ty)
        if f.ty in {"ObjectRef", "SoundEngineObjectRef", "EventRef", "RtpcRef", "PlatformRef", "LanguageRef", "ObjectQuery"}:
            from_types.extend(["&str", "String", "&String"])
        elif f.ty == "String":
            from_types.extend(["&str", "&String"])
        if f.ty in {"SoundEngineObjectRef", "EventRef", "RtpcRef"}:
            from_types.append("u32")
        elif f.ty == "GameObjectRef":
            from_types.extend(["u64", "u32"])
        for src_ty in from_types:
            lines.append("")
            lines.append(f"impl From<{src_ty}> for {struct_name} {{")
            lines.append(f"    fn from(value: {src_ty}) -> Self {{")
            lines.append("        Self::new(value)")
            lines.append("    }")
            lines.append("}")
    elif kind == "args" and len(ctor_fields) in (2, 3):
        t_params = [f"T{i}: Into<{f.ty}>" for i, f in enumerate(ctor_fields)]
        t_names = ", ".join(f"T{i}" for i in range(len(ctor_fields)))
        f_names = ", ".join(f.rust_name for f in ctor_fields)
        f_args = ", ".join(
            f.rust_name if f.ty in INTO_CTOR_TYPES else f"{f.rust_name}.into()"
            for f in ctor_fields
        )
        lines.append("")
        lines.append(f"impl<{', '.join(t_params)}> From<({t_names})> for {struct_name} {{")
        lines.append(f"    fn from(({f_names}): ({t_names})) -> Self {{")
        lines.append(f"        Self::new({f_args})")
        lines.append("    }")
        lines.append("}")
    elif kind == "options" and not required:
        lines.append("")
        lines.append(f"impl From<()> for {struct_name} {{")
        lines.append("    fn from((): ()) -> Self {")
        lines.append("        Self::default()")
        lines.append("    }")
        lines.append("}")
        for f in optional:
            if f.rust_name == "return_fields" and f.ty.startswith("Vec<") and f.ty.endswith(">"):
                elem_ty = f.ty[4:-1]
                lines.append("")
                lines.append(f"impl<const N: usize> From<[{elem_ty}; N]> for {struct_name} {{")
                lines.append(f"    fn from(return_fields: [{elem_ty}; N]) -> Self {{")
                lines.append("        Self::new().return_fields(return_fields)")
                lines.append("    }")
                lines.append("}")
                lines.append("")
                lines.append(f"impl From<Vec<{elem_ty}>> for {struct_name} {{")
                lines.append(f"    fn from(return_fields: Vec<{elem_ty}>) -> Self {{")
                lines.append("        Self::new().return_fields(return_fields)")
                lines.append("    }")
                lines.append("}")

    return "\n".join(lines)


def emit_enum(name, doc, values, derive_both=True, variant_docs=None):
    """Emit a simple string enum with explicit serde renames."""
    lines = doc_lines(doc)
    derives = "serde::Serialize, serde::Deserialize" if derive_both else "serde::Serialize"
    lines.append(f"#[derive(Debug, Clone, Copy, PartialEq, Eq, {derives})]")
    lines.append(f"pub enum {name} {{")
    for v, variant in unique_names(values, pascal_case, "Legacy").items():
        vdoc = (variant_docs or {}).get(v)
        if vdoc:
            lines.extend(doc_lines(vdoc, "    "))
        else:
            lines.append(f"    /// `{v}`.")
        lines.append(f'    #[serde(rename = "{v}")]')
        lines.append(f"    {variant},")
    lines.append("}")
    return "\n".join(lines)


def emit_int_enum(name, doc, values):
    """Emit an integer enum (serializes as a bare number) with explicit discriminants.

    `values` is a list of ``(VariantName, int_value)`` tuples.
    """
    lines = doc_lines(doc)
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    lines.append("#[repr(u32)]")
    lines.append(f"pub enum {name} {{")
    for variant, value in values:
        lines.append(f"    /// `{variant}` (`{value}`).")
        lines.append(f"    {variant} = {value},")
    lines.append("}")
    lines.append("")
    lines.append(f"impl {name} {{")
    lines.append("    /// Returns the raw integer discriminant for this variant.")
    lines.append(f"    pub const fn as_u32(self) -> u32 {{ self as u32 }}")
    lines.append("    /// Converts a raw integer discriminant into the corresponding variant, if valid.")
    lines.append("    pub const fn from_u32(value: u32) -> Option<Self> {")
    lines.append("        match value {")
    for variant, value in values:
        lines.append(f"            {value} => Some({name}::{variant}),")
    lines.append("            _ => None,")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append(f"impl serde::Serialize for {name} {{")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_u32(self.as_u32())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append(f"impl<'de> serde::Deserialize<'de> for {name} {{")
    lines.append("    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")
    lines.append("        let value = u32::deserialize(deserializer)?;")
    lines.append("        Self::from_u32(value).ok_or_else(|| serde::de::Error::custom(format!(")
    lines.append(f"            \"invalid {name} value: {{}}\", value")
    lines.append("        )))")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


# ─────────────────────────────────────────────────────────────────────────────
# object_types.rs and properties.rs (from WObjects.xml)
# ─────────────────────────────────────────────────────────────────────────────

def load_object_type_entries(schema_dirs):
    """Union of Wwise object types across all versions, keyed by class_id.

    A class_id that maps to multiple names across versions (a rename, e.g.
    ActorMixer -> PropertyContainer) yields one entry per name; the name from
    the latest version is marked canonical and drives `from_class_id`.
    """
    by_class_id = {}
    for version_index, schema_dir in enumerate(schema_dirs):
        wobjects_path = schema_dir / "Definitions" / "WObjects.xml"
        if not wobjects_path.exists():
            continue
        tree = ET.parse(wobjects_path)
        root = tree.getroot()
        for el in root.findall("WwiseObject"):
            name = el.attrib.get("Name")
            if not name:
                continue
            company_id = int(el.attrib.get("CompanyID", "1"))
            plugin_id = int(el.attrib.get("PluginID", "0"))
            class_id = (plugin_id << 16) | (company_id << 4)
            names = by_class_id.setdefault(class_id, {})
            if name not in names:
                names[name] = version_index

    entries = []
    for class_id, names in sorted(by_class_id.items()):
        canonical_name = max(names, key=lambda n: names[n])
        for name in sorted(names):
            entries.append({
                "name": name,
                "class_id": class_id,
                "canonical_name": canonical_name,
                "canonical": name == canonical_name,
            })
    return entries


def gen_object_types(schema_dirs) -> str:
    entries = load_object_type_entries(schema_dirs)
    variants = [(e["name"], property_variant_name(e["name"]), e) for e in entries]

    lines = [
        "// Auto-generated Wwise object types from WObjects.xml.",
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
        "",
        "/// Typed identifier for a Wwise object type (from the Wwise Objects Reference).",
        "///",
        "/// Derived from the union of `<WwiseObject>` definitions in `WObjects.xml`",
        "/// across all supported Wwise versions.",
        "#[allow(non_camel_case_types)]",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]",
        "pub enum ObjectType {",
    ]
    for wire, variant, meta in variants:
        doc = f"    /// `{wire}` (class ID `{meta['class_id']}`)."
        if not meta["canonical"]:
            doc += f" Wwise 2024 name; renamed to `{meta['canonical_name']}` in Wwise 2025."
        lines.append(doc)
        lines.append(f"    {variant},")
    lines.append("}")
    lines.append("")
    lines.append("impl ObjectType {")
    lines.append("    /// Returns the Wwise object type name (e.g. `\"Sound\"`, `\"PropertyContainer\"`).")
    lines.append("    pub const fn as_str(self) -> &'static str {")
    lines.append("        match self {")
    for wire, variant, _ in variants:
        lines.append(f'            Self::{variant} => "{wire}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the numeric Wwise class ID for this object type.")
    lines.append("    pub const fn class_id(self) -> u32 {")
    lines.append("        match self {")
    for _, variant, meta in variants:
        lines.append(f"            Self::{variant} => {meta['class_id']},")
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Looks up an [`ObjectType`] by its numeric Wwise class ID.")
    lines.append("    /// Renamed types (e.g. `ActorMixer`/`PropertyContainer`) resolve to the")
    lines.append("    /// canonical (latest Wwise) name.")
    lines.append("    pub const fn from_class_id(class_id: u32) -> Option<Self> {")
    lines.append("        match class_id {")
    for _, variant, meta in variants:
        if meta["canonical"]:
            lines.append(f"            {meta['class_id']} => Some(Self::{variant}),")
    lines.append("            _ => None,")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl AsRef<str> for ObjectType {")
    lines.append("    fn as_ref(&self) -> &str {")
    lines.append("        self.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl std::fmt::Display for ObjectType {")
    lines.append("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
    lines.append("        f.write_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl std::str::FromStr for ObjectType {")
    lines.append("    type Err = String;")
    lines.append("")
    lines.append("    fn from_str(s: &str) -> Result<Self, Self::Err> {")
    lines.append("        match s {")
    for wire, variant, _ in variants:
        lines.append(f'            "{wire}" => Ok(Self::{variant}),')
    lines.append('            _ => Err(format!("unknown Wwise ObjectType: {s}")),')
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<ObjectType> for &'static str {")
    lines.append("    fn from(object_type: ObjectType) -> Self {")
    lines.append("        object_type.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<ObjectType> for String {")
    lines.append("    fn from(object_type: ObjectType) -> Self {")
    lines.append("        object_type.as_str().to_string()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl serde::Serialize for ObjectType {")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl<'de> serde::Deserialize<'de> for ObjectType {")
    lines.append("    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")
    lines.append("        struct ObjectTypeVisitor;")
    lines.append("")
    lines.append("        impl<'de> serde::de::Visitor<'de> for ObjectTypeVisitor {")
    lines.append("            type Value = ObjectType;")
    lines.append("")
    lines.append("            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
    lines.append('                formatter.write_str("a Wwise ObjectType string")')
    lines.append("            }")
    lines.append("")
    lines.append("            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {")
    lines.append("                s.parse().map_err(E::custom)")
    lines.append("            }")
    lines.append("        }")
    lines.append("")
    lines.append("        deserializer.deserialize_str(ObjectTypeVisitor)")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


def property_variant_name(wire: str) -> str:
    variant = re.sub(r"[^A-Za-z0-9_]", "_", wire)
    if variant[0].isdigit():
        variant = f"_{variant}"
    if variant in RUST_KEYWORDS:
        variant = f"{variant}_"
    return variant


def load_property_entries(schema_dirs):
    """Union of property/reference/list names across all versions.

    Metadata (display name, type, meaning) is taken from the latest version
    that defines each name.
    """
    entries = {}
    for version_index, schema_dir in enumerate(schema_dirs):
        wobjects_path = schema_dir / "Definitions" / "WObjects.xml"
        if not wobjects_path.exists():
            continue
        tree = ET.parse(wobjects_path)
        root = tree.getroot()
        for tag, kind_label in (
            ("Property", "Property"),
            ("Reference", "Reference"),
            ("ObjectList", "ObjectList"),
        ):
            for el in root.findall(f".//{tag}"):
                name = el.attrib.get("Name")
                if not name:
                    continue
                ui = el.find("UserInterface")
                display = ui.attrib.get("DisplayName") if ui is not None else None
                entry = {
                    "kind": kind_label,
                    "type": el.attrib.get("Type"),
                    "meaning": el.attrib.get("DataMeaning"),
                    "display": display,
                    "version": version_index,
                }
                existing = entries.get(name)
                if existing is None or version_index > existing["version"]:
                    entries[name] = entry
    return entries


def gen_properties(schema_dirs) -> str:
    entries = load_property_entries(schema_dirs)

    sorted_names = sorted(entries)
    variants = [(name, property_variant_name(name), entries[name]) for name in sorted_names]

    lines = [
        "// Auto-generated Wwise object property, reference, and list names from WObjects.xml.",
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
        "",
        "/// Typed identifier for a Wwise object property, reference, or object list.",
        "///",
        "/// Derived from the union of `WObjects.xml` across all supported Wwise versions.",
        "/// Use [`Property::as_str`] for the bare name (e.g. `\"Volume\"`) or",
        "/// [`Property::as_at_str`] for the `@`-prefixed accessor (e.g. `\"@Volume\"`).",
        "#[allow(non_camel_case_types)]",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]",
        "pub enum Property {",
    ]
    for wire, variant, meta in variants:
        details = [f"`{wire}` ({meta['kind']})"]
        if meta["display"]:
            details.append(f"- {meta['display']}")
        qualifiers = [f"`{q}`" for q in (meta["type"], meta["meaning"]) if q]
        if qualifiers:
            details.append(f"({', '.join(qualifiers)})")
        lines.append(f"    /// {' '.join(details)}.")
        lines.append(f"    {variant},")
    lines.append("}")
    lines.append("")
    lines.append("impl Property {")
    lines.append("    /// Returns the bare property name without the `@` prefix (e.g. `\"Volume\"`).")
    lines.append("    pub const fn as_str(self) -> &'static str {")
    lines.append("        match self {")
    for wire, variant, _ in variants:
        lines.append(f'            Self::{variant} => "{wire}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the `@`-prefixed property accessor (e.g. `\"@Volume\"`).")
    lines.append("    pub const fn as_at_str(self) -> &'static str {")
    lines.append("        match self {")
    for wire, variant, _ in variants:
        lines.append(f'            Self::{variant} => "@{wire}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl AsRef<str> for Property {")
    lines.append("    fn as_ref(&self) -> &str {")
    lines.append("        self.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl std::fmt::Display for Property {")
    lines.append("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
    lines.append("        f.write_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<Property> for &'static str {")
    lines.append("    fn from(prop: Property) -> Self {")
    lines.append("        prop.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<Property> for String {")
    lines.append("    fn from(prop: Property) -> Self {")
    lines.append("        prop.as_str().to_string()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl serde::Serialize for Property {")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl<'de> serde::Deserialize<'de> for Property {")
    lines.append("    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")
    lines.append("        struct PropertyVisitor;")
    lines.append("")
    lines.append("        impl<'de> serde::de::Visitor<'de> for PropertyVisitor {")
    lines.append("            type Value = Property;")
    lines.append("")
    lines.append("            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
    lines.append('                formatter.write_str("a Wwise Property name")')
    lines.append("            }")
    lines.append("")
    lines.append("            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {")
    lines.append("                let bare = s.strip_prefix('@').unwrap_or(s);")
    lines.append("                match bare {")
    for wire, variant, _ in variants:
        lines.append(f'                    "{wire}" => Ok(Property::{variant}),')
    lines.append('                    _ => Err(E::custom(format!("unknown Wwise Property: {s}"))),')
    lines.append("                }")
    lines.append("            }")
    lines.append("        }")
    lines.append("")
    lines.append("        deserializer.deserialize_str(PropertyVisitor)")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


# ─────────────────────────────────────────────────────────────────────────────
# primitives.rs
# ─────────────────────────────────────────────────────────────────────────────

def extract_enum_values(node):
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


def union_definition_enum_values(schema_dirs, definition_file, key):
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


def load_object_return_props():
    props = {}
    for sdir in SCHEMA_DIRS:
        defs_path = sdir / "Definitions" / "waapi_definitions.json"
        if not defs_path.exists():
            continue
        waapi_defs = json.loads(defs_path.read_text())
        obj_return = waapi_defs.get("definitions", {}).get("objectReturn", {})
        for wire, sub in obj_return.get("properties", {}).items():
            props[wire] = sub
    return props


def gen_object_return_field(enum_vals, field_docs=None):
    lines = doc_lines("A field that can be requested in `return` options.")
    lines.append("#[derive(Debug, Clone)]")
    lines.append("pub enum ObjectReturnField {")
    arms = []
    for v, variant in unique_names(enum_vals, pascal_case, "Legacy").items():
        vdoc = (field_docs or {}).get(v)
        if vdoc:
            lines.extend(doc_lines(vdoc, "    "))
        else:
            lines.append(f"    /// `{v}`.")
        lines.append(f"    {variant},")
        arms.append(f"            ObjectReturnField::{variant} => \"{v}\",")
    lines.append("    /// Request a Wwise object property, reference, or list (`@Property`).")
    lines.append("    Property(Property),")
    lines.append("    /// Any other accessor, e.g. `@Property` or a dot-separated query.")
    lines.append("    Custom(String),")
    lines.append("}")
    lines.append("")
    lines.append("impl serde::Serialize for ObjectReturnField {")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_str(match self {")
    lines += arms
    lines.append("            ObjectReturnField::Property(prop) => prop.as_at_str(),")
    lines.append("            ObjectReturnField::Custom(value) => value.as_str(),")
    lines.append("        })")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl ObjectReturnField {")
    lines.append("    /// Request a Wwise object property, reference, or list (`@Property`).")
    lines.append("    pub fn property(prop: Property) -> Self { ObjectReturnField::Property(prop) }")
    lines.append("    /// Request a custom accessor, e.g. `@Property` or a dot-separated query.")
    lines.append("    pub fn custom(s: impl Into<String>) -> Self { ObjectReturnField::Custom(s.into()) }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<Property> for ObjectReturnField {")
    lines.append("    fn from(prop: Property) -> Self { ObjectReturnField::Property(prop) }")
    lines.append("}")
    lines.append("impl From<&str> for ObjectReturnField {")
    lines.append("    fn from(s: &str) -> Self { ObjectReturnField::Custom(s.to_string()) }")
    lines.append("}")
    lines.append("impl From<String> for ObjectReturnField {")
    lines.append("    fn from(s: String) -> Self { ObjectReturnField::Custom(s) }")
    lines.append("}")
    lines.append("impl From<&String> for ObjectReturnField {")
    lines.append("    fn from(s: &String) -> Self { ObjectReturnField::Custom(s.clone()) }")
    lines.append("}")
    return "\n".join(lines)


def gen_wwise_object(ctx):
    props = load_object_return_props()
    names = unique_names(props.keys(), lambda w: field_name(ctx.uri, w, "response"), "_legacy")
    fields = []
    for wire, sub in props.items():
        if wire == "type" and "\\ref wobjects_index" in sub.get("description", ""):
            ty = "ObjectType"
        else:
            ty = rust_type(sub, ctx)
        fields.append(Field(wire, names[wire], ty, True, schema_description(ctx, sub)))
    lines = [
        "/// A Wwise object as returned by `object.get` and similar endpoints.",
        "/// Fields are optional because their presence depends on the `return` options;",
        "/// accessors not listed here (e.g. `@Property` values) land in `extra`.",
        "#[derive(Debug, Clone, Default, serde::Serialize)]",
        "pub struct WwiseObject {",
    ]
    for f in fields:
        lines += f.declaration("response")
    lines.append("    /// Additional `@`-prefixed properties, references, or list values returned by the query.")
    lines.append("    #[serde(flatten)]")
    lines.append("    pub extra: std::collections::HashMap<String, serde_json::Value>,")
    lines.append("}")
    lines.append("")
    lines.append("impl WwiseObject {")
    lines.append("    /// Returns the object GUID string (`id`), if present in the response.")
    lines.append("    pub fn id(&self) -> Option<&str> {")
    lines.append("        self.id.as_deref()")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the object name (`name`), if present in the response.")
    lines.append("    pub fn name(&self) -> Option<&str> {")
    lines.append("        self.name.as_deref()")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the typed [`ObjectType`] (`type`), if present in the response.")
    lines.append("    pub fn object_type(&self) -> Option<ObjectType> {")
    lines.append("        self.object_type")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the object project path (`path`), if present in the response.")
    lines.append("    pub fn path(&self) -> Option<&str> {")
    lines.append("        self.path.as_deref()")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the object notes (`notes`), if present in the response.")
    lines.append("    pub fn notes(&self) -> Option<&str> {")
    lines.append("        self.notes.as_deref()")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Look up a property value in `extra` (checks `\"@Property\"` first, then `\"Property\"`).")
    lines.append("    pub fn property(&self, prop: Property) -> Option<&serde_json::Value> {")
    lines.append("        self.extra")
    lines.append("            .get(prop.as_at_str())")
    lines.append("            .or_else(|| self.extra.get(prop.as_str()))")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Look up a numeric (`f64`) property value in `extra`.")
    lines.append("    pub fn property_f64(&self, prop: Property) -> Option<f64> {")
    lines.append("        self.property(prop).and_then(serde_json::Value::as_f64)")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Look up an integer (`i64`) property value in `extra`.")
    lines.append("    pub fn property_i64(&self, prop: Property) -> Option<i64> {")
    lines.append("        self.property(prop).and_then(serde_json::Value::as_i64)")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Look up a boolean (`bool`) property value in `extra`.")
    lines.append("    pub fn property_bool(&self, prop: Property) -> Option<bool> {")
    lines.append("        self.property(prop).and_then(serde_json::Value::as_bool)")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Look up a string (`&str`) property value in `extra`.")
    lines.append("    pub fn property_str(&self, prop: Property) -> Option<&str> {")
    lines.append("        self.property(prop).and_then(serde_json::Value::as_str)")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.extend(emit_flat_map_deserialize("WwiseObject", fields))
    return "\n".join(lines)


def gen_object_query(ctx):
    args = ctx.doc["argsSchema"]
    from_variants = args["properties"]["from"]["oneOf"]
    variants = []
    for v in from_variants:
        prop = list(v["properties"].keys())[0]
        sub = v["properties"][prop]
        if prop == "ofType" or "\\ref wobjects_index" in sub.get("description", ""):
            item_ty = "ObjectType"
        elif stringy_items(sub, ctx):
            item_ty = "String"
        else:
            item_ty = "serde_json::Value"
        variants.append((prop, pascal_case(prop), item_ty, schema_description(ctx, sub)))
    lines = ["/// Argument to the `from` field of `object.get`."]
    lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    lines.append("pub enum ObjectQuery {")
    for wire, variant, item_ty, vdoc in variants:
        lines += doc_lines(vdoc, "    ") or [f"    /// `{wire}`."]
        lines.append(f'    #[serde(rename = "{wire}")]')
        lines.append(f"    {variant}(Vec<{item_ty}>),")
    lines.append("}")
    lines.append("")
    lines.append("impl ObjectQuery {")
    lines.append("    /// Query a single object by GUID or Short ID.")
    lines.append("    pub fn id(id: impl Into<String>) -> Self {")
    lines.append("        ObjectQuery::Id(vec![serde_json::Value::String(id.into())])")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query multiple objects by GUID strings.")
    lines.append("    pub fn ids<I, S>(ids: I) -> Self")
    lines.append("    where")
    lines.append("        I: IntoIterator<Item = S>,")
    lines.append("        S: Into<String>,")
    lines.append("    {")
    lines.append("        ObjectQuery::Id(ids.into_iter().map(|s| serde_json::Value::String(s.into())).collect())")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query a single object by backslash-separated hierarchy path.")
    lines.append("    pub fn path(path: impl Into<String>) -> Self {")
    lines.append("        ObjectQuery::Path(vec![path.into()])")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query multiple objects by backslash-separated hierarchy paths.")
    lines.append("    pub fn paths<I, S>(paths: I) -> Self")
    lines.append("    where")
    lines.append("        I: IntoIterator<Item = S>,")
    lines.append("        S: Into<String>,")
    lines.append("    {")
    lines.append("        ObjectQuery::Path(paths.into_iter().map(Into::into).collect())")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query a single object by unique qualified name (`Type:Name`).")
    lines.append("    pub fn name(name: impl Into<String>) -> Self {")
    lines.append("        ObjectQuery::Name(vec![name.into()])")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query multiple objects by unique qualified names (`Type:Name`).")
    lines.append("    pub fn names<I, S>(names: I) -> Self")
    lines.append("    where")
    lines.append("        I: IntoIterator<Item = S>,")
    lines.append("        S: Into<String>,")
    lines.append("    {")
    lines.append("        ObjectQuery::Name(names.into_iter().map(Into::into).collect())")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Search objects by name or notes text.")
    lines.append("    pub fn search(text: impl Into<String>) -> Self {")
    lines.append("        ObjectQuery::Search(vec![text.into()])")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query all objects of a single [`ObjectType`].")
    lines.append("    pub fn of_type(object_type: ObjectType) -> Self {")
    lines.append("        ObjectQuery::OfType(vec![object_type])")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query all objects matching any of the given [`ObjectType`] values.")
    lines.append("    pub fn of_types(object_types: impl IntoIterator<Item = ObjectType>) -> Self {")
    lines.append("        ObjectQuery::OfType(object_types.into_iter().collect())")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Run a saved Wwise Query object by GUID.")
    lines.append("    pub fn query(guid: impl Into<String>) -> Self {")
    lines.append("        ObjectQuery::Query(vec![guid.into()])")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<String> for ObjectQuery {")
    lines.append("    fn from(s: String) -> Self {")
    lines.append("        if s.starts_with('{') && s.ends_with('}') {")
    lines.append("            ObjectQuery::Id(vec![serde_json::Value::String(s)])")
    lines.append("        } else if s.starts_with('\\\\') {")
    lines.append("            ObjectQuery::Path(vec![s])")
    lines.append("        } else if s.contains(':') {")
    lines.append("            ObjectQuery::Name(vec![s])")
    lines.append("        } else {")
    lines.append("            ObjectQuery::Path(vec![s])")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("impl From<&str> for ObjectQuery {")
    lines.append("    fn from(s: &str) -> Self { Self::from(s.to_string()) }")
    lines.append("}")
    lines.append("impl From<&String> for ObjectQuery {")
    lines.append("    fn from(s: &String) -> Self { Self::from(s.as_str()) }")
    lines.append("}")
    lines.append("impl From<Vec<String>> for ObjectQuery {")
    lines.append("    fn from(v: Vec<String>) -> Self {")
    lines.append("        ObjectQuery::Id(v.into_iter().map(serde_json::Value::String).collect())")
    lines.append("    }")
    lines.append("}")
    lines.append("impl From<ObjectType> for ObjectQuery {")
    lines.append("    fn from(object_type: ObjectType) -> Self { ObjectQuery::OfType(vec![object_type]) }")
    lines.append("}")
    lines.append("impl From<Vec<ObjectType>> for ObjectQuery {")
    lines.append("    fn from(object_types: Vec<ObjectType>) -> Self { ObjectQuery::OfType(object_types) }")
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


def gen_project_info_types(ctx):
    result = ctx.doc["resultSchema"]
    out = emit_struct(
        "response",
        "ProjectPlatformInfo",
        "Platform defined in the project, returned by `getProjectInfo`.",
        collect_fields(ctx, result["properties"]["platforms"]["items"], "response", "ProjectPlatformInfo"),
        ctx,
    )
    out += "\n\n"
    out += emit_struct(
        "response",
        "ProjectLanguageInfo",
        "Language defined in the project, returned by `getProjectInfo`.",
        collect_fields(ctx, result["properties"]["languages"]["items"], "response", "ProjectLanguageInfo"),
        ctx,
    )
    out += "\n\n"
    out += emit_struct(
        "response",
        "ProjectDirectoriesInfo",
        "Collection of project directories returned by `getProjectInfo`.",
        collect_fields(ctx, result["properties"]["directories"], "response", "ProjectDirectoriesInfo"),
        ctx,
    )
    return out


def gen_transport_types(exec_ctx, state_ctx, list_ctx):
    action_vals = exec_ctx.doc["argsSchema"]["properties"]["action"]["enum"]
    state_vals = state_ctx.doc["resultSchema"]["properties"]["state"]["enum"]
    list_item_sch = list_ctx.doc["resultSchema"]["properties"]["list"]["items"]

    parts = [
        emit_enum("TransportAction", "Action to execute on a Wwise transport object.", action_vals),
        emit_enum("TransportState", "Playback state of a Wwise transport object.", state_vals),
        emit_struct(
            "response",
            "TransportObject",
            "A transport object entry returned by `transport.getList`.",
            collect_fields(list_ctx, list_item_sch, "response", "TransportObject"),
            list_ctx,
        ),
    ]
    return "\n\n".join(parts)


def gen_attenuation_and_object_types(waapi_defs, atten_ctx, paste_ctx, switch_ctx, open_ctx):
    curve_shape_vals = waapi_defs["definitions"]["curveShape"]["enum"]
    curve_type_vals = atten_ctx.doc["argsSchema"]["properties"]["curveType"]["enum"]
    curve_use_vals = atten_ctx.doc["resultSchema"]["properties"]["use"]["enum"]
    paste_mode_vals = paste_ctx.doc["argsSchema"]["properties"]["pasteMode"]["enum"]
    migration_vals = open_ctx.doc["argsSchema"]["properties"]["onMigrationRequired"]["enum"]

    curve_point_rust = """/// A 2D point in an attenuation or RTPC curve (`pointsDefinition`).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CurvePoint2D {
    /// X coordinate of the curve point.
    pub x: f64,
    /// Y coordinate of the curve point.
    pub y: f64,
    /// Shape of the curve segment between this point and the next.
    pub shape: CurveShape,
}

impl CurvePoint2D {
    /// Creates a new [`CurvePoint2D`].
    pub const fn new(x: f64, y: f64, shape: CurveShape) -> Self {
        Self { x, y, shape }
    }
}"""

    switch_assignment_rust = """/// Assignment pairing a Switch Container child object with a State or Switch (`switchContainer.getAssignments`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SwitchContainerAssignment {
    /// GUID of the child object assigned to a State or Switch.
    pub child: String,
    /// GUID of the State or Switch to which the child is assigned.
    #[serde(rename = "stateOrSwitch")]
    pub state_or_switch: String,
}

impl SwitchContainerAssignment {
    /// Creates a new [`SwitchContainerAssignment`].
    pub fn new(child: impl Into<String>, state_or_switch: impl Into<String>) -> Self {
        Self { child: child.into(), state_or_switch: state_or_switch.into() }
    }
}"""

    parts = [
        emit_enum("CurveShape", "Shape of a curve segment between two points.", curve_shape_vals),
        curve_point_rust,
        emit_enum("AttenuationCurveType", "Type of an attenuation curve.", curve_type_vals),
        emit_enum("AttenuationCurveUse", "Usage mode of an attenuation curve.", curve_use_vals),
        emit_enum("PasteMode", "Paste mode for lists in `object.pasteProperties`.", paste_mode_vals),
        switch_assignment_rust,
        emit_enum("ProjectMigrationAction", "Action when opening a project that requires migration.", migration_vals),
    ]
    return "\n\n".join(parts)


def gen_soundengine_and_misc_types():
    """Shared types for the SoundEngine runtime surface, object introspection,
    blend containers, soundbanks, and audio conversion."""
    parts = [
        emit_int_enum(
            "ActionOnEventType",
            "Action to execute on an event via `soundengine.executeActionOnEvent`.",
            [("Stop", 0), ("Pause", 1), ("Resume", 2), ("Break", 3), ("ReleaseEnvelope", 4)],
        ),
        emit_int_enum(
            "AkCurveInterpolation",
            "Interpolation curve for fades (`AkCurveInterpolation`).",
            [("Log3", 0), ("Sine", 1), ("Log1", 2), ("InvSCurve", 3), ("Linear", 4),
             ("SCurve", 5), ("Exp1", 6), ("SineRecip", 7), ("Exp3", 8), ("Constant", 9)],
        ),
        emit_int_enum(
            "MultiPositionType",
            "How multiple positions are combined (`soundengine.setMultiplePositions`).",
            [("SingleSource", 0), ("MultiSources", 1), ("MultiDirections", 2)],
        ),
        """/// Auxiliary send value for a game object (`soundengine.setGameObjectAuxSendValues`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuxSendValue {
    /// Game object receiving the aux send.
    pub listener: GameObjectRef,
    /// Auxiliary bus to send to.
    #[serde(rename = "auxBus")]
    pub aux_bus: SoundEngineObjectRef,
    /// Gain control value in dB.
    #[serde(rename = "controlValue")]
    pub control_value: f64,
}

impl AuxSendValue {
    /// Creates a new [`AuxSendValue`].
    pub fn new(listener: impl Into<GameObjectRef>, aux_bus: impl Into<SoundEngineObjectRef>, control_value: f64) -> Self {
        Self { listener: listener.into(), aux_bus: aux_bus.into(), control_value }
    }
}""",
        """/// A single position entry for `soundengine.setMultiplePositions`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MultiPosition {
    /// The 3D position and orientation of the sound source.
    pub position: Position,
}

impl MultiPosition {
    /// Creates a new [`MultiPosition`].
    pub fn new(position: impl Into<Position>) -> Self {
        Self { position: position.into() }
    }
}

impl From<Position> for MultiPosition {
    fn from(position: Position) -> Self {
        Self { position }
    }
}

impl From<Vector3> for MultiPosition {
    fn from(position: Vector3) -> Self {
        Self { position: Position::new(position) }
    }
}""",
        """/// A Wwise object type entry returned by `object.getTypes`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WwiseObjectTypeInfo {
    /// The object type name (e.g. `"Sound"`, `"ActorMixer"`).
    #[serde(rename = "type")]
    pub object_type: String,
    /// The class ID of the object type.
    #[serde(rename = "classId")]
    pub class_id: u32,
    /// The display name of the object type.
    pub name: String,
}""",
        emit_enum("RtpcSupportMode", "How a property supports RTPC (`object.getPropertyInfo`).",
                  ["None", "Additive", "Exclusive", "Multiplicative"]),
        """/// RTPC/randomizer/unlink support flags for a property (`object.getPropertyInfo`).
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyInfoSupports {
    /// How the property supports RTPC curves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtpc: Option<RtpcSupportMode>,
    /// Whether the property supports a randomizer modifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub randomizer: Option<bool>,
    /// Whether the property can be unlinked per platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlink: Option<bool>,
}""",
        emit_enum("BlendFadeMode", "Fade mode of a blend track edge.", ["None", "Manual", "Automatic"]),
        """/// An edge of a blend track (`blendContainer.addAssignment` / `getAssignments`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlendEdge {
    /// Crossfade mode for this edge.
    #[serde(rename = "fadeMode")]
    pub fade_mode: BlendFadeMode,
    /// Shape of the crossfade curve.
    #[serde(rename = "fadeShape")]
    pub fade_shape: CurveShape,
    /// Position of the edge along the blend track X axis.
    #[serde(rename = "edgePosition")]
    pub edge_position: f64,
    /// Position where the fade starts or ends along the blend track X axis.
    #[serde(default, rename = "fadePosition", skip_serializing_if = "Option::is_none")]
    pub fade_position: Option<f64>,
}

impl BlendEdge {
    /// Creates a new [`BlendEdge`].
    pub fn new(fade_mode: BlendFadeMode, fade_shape: CurveShape, edge_position: f64) -> Self {
        Self { fade_mode, fade_shape, edge_position, fade_position: None }
    }

    /// Sets the position where the fade starts or ends along the blend track X axis.
    pub fn fade_position(mut self, fade_position: f64) -> Self {
        self.fade_position = Some(fade_position);
        self
    }
}""",
        """/// A blend track assignment (`blendContainer.getAssignments`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlendTrackAssignment {
    /// GUID of the assigned child.
    pub child: String,
    /// Zero-based index of the assignment within the blend track.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// Start and end edges of the assignment along the blend track.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<BlendEdge>>,
}""",
        emit_enum("OnCurveUpdate", "Action when a Game Parameter range is modified.", ["stretch", "preserveX"]),
        emit_enum("SoundBankInclusionType", "Inclusion type used by `soundbank.generate`.", ["event", "structure", "media"]),
        emit_enum("SoundBankInclusionFilter", "Inclusion relation used by `soundbank.get/setInclusions`.", ["events", "structures", "media"]),
        emit_enum("SoundBankInclusionOperation", "How `soundbank.setInclusions` modifies the inclusion list.", ["add", "remove", "replace"]),
        """/// A SoundBank inclusion (`soundbank.get/setInclusions`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundBankInclusion {
    /// Object included in the SoundBank.
    pub object: ObjectRef,
    /// Inclusion filter categories (`events`, `structures`, `media`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<SoundBankInclusionFilter>,
}

impl SoundBankInclusion {
    /// Creates a new [`SoundBankInclusion`] for `object` with an empty filter list.
    pub fn new(object: impl Into<ObjectRef>) -> Self {
        Self { object: object.into(), filter: Vec::new() }
    }

    /// Appends a [`SoundBankInclusionFilter`] to the inclusion filter list.
    pub fn filter(mut self, filter: SoundBankInclusionFilter) -> Self {
        self.filter.push(filter);
        self
    }
}""",
        """/// A user-defined SoundBank to generate (`soundbank.generate`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundBankInfo {
    /// Name of the SoundBank.
    pub name: String,
    /// Inclusion types to include in the generated SoundBank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inclusions: Option<Vec<SoundBankInclusionType>>,
    /// Events to include in the SoundBank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ObjectRef>>,
    /// Auxiliary busses to include in the SoundBank.
    #[serde(default, rename = "auxBusses", skip_serializing_if = "Option::is_none")]
    pub aux_busses: Option<Vec<ObjectRef>>,
    /// Whether to force a rebuild of the SoundBank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebuild: Option<bool>,
}

impl SoundBankInfo {
    /// Creates a new [`SoundBankInfo`] with the given SoundBank `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), inclusions: None, events: None, aux_busses: None, rebuild: None }
    }

    /// Sets the inclusion types for the SoundBank.
    pub fn inclusions(mut self, inclusions: impl Into<Vec<SoundBankInclusionType>>) -> Self {
        self.inclusions = Some(inclusions.into());
        self
    }

    /// Sets the events to include in the SoundBank.
    pub fn events<I, T>(mut self, events: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectRef>,
    {
        self.events = Some(events.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the auxiliary busses to include in the SoundBank.
    pub fn aux_busses<I, T>(mut self, aux_busses: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectRef>,
    {
        self.aux_busses = Some(aux_busses.into_iter().map(Into::into).collect());
        self
    }

    /// Sets whether to force a rebuild of the SoundBank.
    pub fn rebuild(mut self, rebuild: bool) -> Self {
        self.rebuild = Some(rebuild);
        self
    }
}""",
        """/// A central log entry (`logCentralItem`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogCentralItem {
    /// Severity level of the log item.
    pub severity: LogSeverity,
    /// Timestamp of the log entry in seconds.
    pub time: i64,
    /// Identifier of the log message type.
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Human-readable log message text.
    pub message: String,
    /// Platform associated with the log entry, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<WwiseObject>,
    /// Additional message parameters, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}""",
        emit_enum("LogChannel", "The Wwise log channel (`log.itemAdded`).",
                  ["soundbankGenerate", "conversion", "copyPlatformSettings", "waapi",
                   "projectLoad", "general", "sourceControl", "lua"]),
        """/// A source file to convert (`soundbank.convertExternalSources`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExternalSourceInfo {
    /// Target platform for conversion.
    pub platform: PlatformRef,
    /// Path to the `.wsources` input file.
    pub input: String,
    /// Optional output directory override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl ExternalSourceInfo {
    /// Creates a new [`ExternalSourceInfo`] for `platform` and `input`.
    pub fn new(platform: impl Into<PlatformRef>, input: impl Into<String>) -> Self {
        Self { platform: platform.into(), input: input.into(), output: None }
    }

    /// Sets the output directory override.
    pub fn output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(output.into());
        self
    }
}""",
        """/// A logged error from `audio.convert`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioConvertError {
    /// Severity level of the conversion log entry.
    pub severity: LogSeverity,
    /// Human-readable conversion error or warning message.
    pub message: String,
}""",
    ]
    return "\n\n".join(parts)


def gen_primitives(schemas, endpoints_docs):
    waapi_defs = schemas.load_definitions("waapi_definitions.json")
    get_ctx = endpoints_docs["ak.wwise.core.object.get"]
    info_ctx = endpoints_docs["ak.wwise.core.getInfo"]
    proj_info_ctx = endpoints_docs["ak.wwise.core.getProjectInfo"]
    import_ctx = endpoints_docs["ak.wwise.core.audio.import"]
    exec_ctx = endpoints_docs["ak.wwise.core.transport.executeAction"]
    state_ctx = endpoints_docs["ak.wwise.core.transport.getState"]
    list_ctx = endpoints_docs["ak.wwise.core.transport.getList"]
    atten_ctx = endpoints_docs["ak.wwise.core.object.getAttenuationCurve"]
    paste_ctx = endpoints_docs["ak.wwise.core.object.pasteProperties"]
    switch_ctx = endpoints_docs["ak.wwise.core.switchContainer.getAssignments"]
    open_ctx = endpoints_docs["ak.wwise.ui.project.open"]

    import_op_node = waapi_defs["definitions"]["importOperationType"]
    import_operation_vals = [b["enum"][0] for b in import_op_node["oneOf"]]
    import_operation_docs = extract_string_enum_docs(import_op_node)

    log_sev_node = waapi_defs["definitions"]["logCentralSeverity"]
    log_severity_vals = [b["enum"][0] for b in log_sev_node["oneOf"]]
    log_severity_docs = extract_string_enum_docs(log_sev_node)

    # Union of return-field enums across versions so 2024-only variants remain
    # usable (e.g. colorRGB, transitionObject, nextGain, prevGain).
    return_field_vals = union_definition_enum_values(
        SCHEMA_DIRS, "waapi_definitions.json", "returnExpression")
    return_field_docs = {
        wire: schema_description(get_ctx, sub)
        for wire, sub in load_object_return_props().items()
    }
    bus_return_vals = union_definition_enum_values(
        SCHEMA_DIRS, "waapi_definitions.json", "busReturnOptions")
    audio_return_vals = union_definition_enum_values(
        SCHEMA_DIRS, "waapi_definitions.json", "audioObjectReturnOptions")
    voice_return_vals = union_definition_enum_values(
        SCHEMA_DIRS, "waapi_definitions.json", "voiceReturnOptions")

    parts = [HEADER]
    parts.append("pub use super::object_types::ObjectType;")
    parts.append("pub use super::properties::Property;")
    parts.append("pub use crate::api::primitives::{")
    parts.append("    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, Position,")
    parts.append("    PropertyValue, Vector3,")
    parts.append("};")
    parts.append("")
    parts.append(STATIC_REFS)
    parts.append(gen_object_return_field(return_field_vals, return_field_docs))
    parts.append(STATIC_OBJECT_RETURN_OPTIONS)
    parts.append(emit_enum("BusReturnField", "Return fields for `ak.wwise.core.profiler.getBusses`.",
                           bus_return_vals))
    parts.append(emit_enum("AudioObjectReturnField", "Return fields for `ak.wwise.core.profiler.getAudioObjects`.",
                           audio_return_vals))
    parts.append(emit_enum("VoiceReturnField", "Return fields for `ak.wwise.core.profiler.getVoices`.",
                           voice_return_vals))
    parts.append(emit_enum("ImportOperation", "Determines how imported objects are created.",
                           import_operation_vals, variant_docs=import_operation_docs))
    parts.append(emit_enum("LogSeverity", "Severity level of an audio import log entry.",
                           log_severity_vals, variant_docs=log_severity_docs))
    parts.append(gen_wwise_object(get_ctx))
    parts.append(STATIC_WWISE_OBJECT_REF)
    parts.append(gen_object_query(get_ctx))
    parts.append(gen_transform_select(get_ctx))
    parts.append(STATIC_TRANSFORM)
    parts.append(gen_version_info(info_ctx))
    parts.append(gen_project_info_types(proj_info_ctx))
    parts.append(gen_import_types(import_ctx))
    parts.append(gen_transport_types(exec_ctx, state_ctx, list_ctx))
    parts.append(gen_attenuation_and_object_types(waapi_defs, atten_ctx, paste_ctx, switch_ctx, open_ctx))
    parts.append(gen_soundengine_and_misc_types())
    return "\n\n".join(parts) + "\n"


HEADER = """// Shared primitives referenced by generated typed bindings.
// This file is generated by tools/generate_typed_api.py; do not edit by hand.
"""

STATIC_REFS = """/// Object reference accepted by soundengine endpoints: name, GUID, or Short ID.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SoundEngineObjectRef {
    /// Object name.
    Name(String),
    /// Object GUID (`{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}`).
    Guid(String),
    /// Object 32-bit Short ID.
    ShortId(u32),
}

impl SoundEngineObjectRef {
    /// Reference an object by its Wwise name.
    pub fn name(s: impl Into<String>) -> Self {
        SoundEngineObjectRef::Name(s.into())
    }

    /// Reference an object by its GUID string (`{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}`).
    pub fn guid(s: impl Into<String>) -> Self {
        SoundEngineObjectRef::Guid(s.into())
    }

    /// Reference an object by its 32-bit Short ID.
    pub fn short_id(id: u32) -> Self {
        SoundEngineObjectRef::ShortId(id)
    }

    /// Alias for [`Self::short_id`].
    pub fn short(id: u32) -> Self {
        SoundEngineObjectRef::ShortId(id)
    }
}

impl From<&str> for SoundEngineObjectRef {
    fn from(s: &str) -> Self {
        if s.starts_with('{') && s.ends_with('}') {
            SoundEngineObjectRef::Guid(s.to_string())
        } else {
            SoundEngineObjectRef::Name(s.to_string())
        }
    }
}
impl From<String> for SoundEngineObjectRef {
    fn from(s: String) -> Self {
        if s.starts_with('{') && s.ends_with('}') {
            SoundEngineObjectRef::Guid(s)
        } else {
            SoundEngineObjectRef::Name(s)
        }
    }
}
impl From<&String> for SoundEngineObjectRef {
    fn from(s: &String) -> Self { Self::from(s.as_str()) }
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
    /// Numeric 64-bit game object ID.
    Id(u64),
}

impl GameObjectRef {
    /// Creates a [`GameObjectRef`] from a 64-bit game object ID.
    pub fn new(id: u64) -> Self {
        GameObjectRef::Id(id)
    }

    /// Creates a [`GameObjectRef`] from a 64-bit game object ID.
    pub fn id(id: u64) -> Self {
        GameObjectRef::Id(id)
    }

    /// Creates a [`GameObjectRef`] from a 64-bit game object ID.
    pub fn game_object(id: u64) -> Self {
        GameObjectRef::Id(id)
    }
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
    /// Language name.
    Name(String),
    /// Language GUID.
    Guid(String),
}

impl LanguageRef {
    /// Reference a language by name.
    pub fn name(s: impl Into<String>) -> Self {
        LanguageRef::Name(s.into())
    }

    /// Reference a language by GUID string (`{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}`).
    pub fn guid(s: impl Into<String>) -> Self {
        LanguageRef::Guid(s.into())
    }
}

impl From<&str> for LanguageRef {
    fn from(s: &str) -> Self {
        if s.starts_with('{') && s.ends_with('}') {
            LanguageRef::Guid(s.to_string())
        } else {
            LanguageRef::Name(s.to_string())
        }
    }
}
impl From<String> for LanguageRef {
    fn from(s: String) -> Self {
        if s.starts_with('{') && s.ends_with('}') {
            LanguageRef::Guid(s)
        } else {
            LanguageRef::Name(s)
        }
    }
}
impl From<&String> for LanguageRef {
    fn from(s: &String) -> Self { Self::from(s.as_str()) }
}"""

STATIC_OBJECT_RETURN_OPTIONS = """/// Options controlling which fields are returned for each object.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ObjectReturnOptions {
    /// List of return expressions/fields to include in each returned object.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// Platform to use when evaluating platform-dependent properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Language to use when evaluating language-dependent properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageRef>,
}

impl ObjectReturnOptions {
    /// Creates a new [`ObjectReturnOptions`] with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an [`ObjectReturnOptions`] requesting the four standard object fields
    /// (`Id`, `Name`, `Type`, and `Path`).
    pub fn basic() -> Self {
        Self::new().return_fields([
            ObjectReturnField::Id,
            ObjectReturnField::Name,
            ObjectReturnField::Type,
            ObjectReturnField::Path,
        ])
    }

    /// Specifies the fields or `@Property` expressions to return for each object.
    pub fn return_fields<I, T>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields = Some(fields.into_iter().map(Into::into).collect());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_<I, T>(self, fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields(fields)
    }

    /// Specifies the platform to use when querying properties.
    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    /// Specifies the language to use when querying language-specific properties.
    pub fn language(mut self, language: impl Into<LanguageRef>) -> Self {
        self.language = Some(language.into());
        self
    }
}

impl From<()> for ObjectReturnOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for ObjectReturnOptions {
    fn from(fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(fields)
    }
}

impl From<Vec<ObjectReturnField>> for ObjectReturnOptions {
    fn from(fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(fields)
    }
}

impl From<&[ObjectReturnField]> for ObjectReturnOptions {
    fn from(fields: &[ObjectReturnField]) -> Self {
        Self::new().return_fields(fields.iter().cloned())
    }
}

impl From<ObjectReturnField> for ObjectReturnOptions {
    fn from(field: ObjectReturnField) -> Self {
        Self::new().return_fields([field])
    }
}"""

STATIC_WWISE_OBJECT_REF = """/// A lightweight object reference returned inside other objects.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WwiseObjectRef {
    /// Object ID (GUID).
    #[serde(default, rename = "id")]
    pub id: Option<String>,
    /// Object name.
    #[serde(default)]
    pub name: Option<String>,
}

impl WwiseObjectRef {
    /// Returns the object GUID string (`id`), if present in the response.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Returns the object name (`name`), if present in the response.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}"""

STATIC_TRANSFORM = """/// A single transformation step in an `object.get` query.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Transform {
    /// Select related objects (`parent`, `children`, `descendants`, `ancestors`, or `referencesTo`).
    Select {
        /// Selector list (contains a single [`TransformSelect`] element).
        select: Vec<TransformSelect>,
    },
    /// Deduplicate object instances (`"distinct"`).
    Distinct(String),
    /// Slice results by `[from, count]` boundary indexes.
    Range {
        /// Two-element range `[from, count]`.
        range: Vec<usize>,
    },
    /// Filter objects using a `where` predicate and parameter.
    Where {
        /// Predicate name followed by its parameter(s).
        #[serde(rename = "where")]
        where_: Vec<serde_json::Value>,
    },
}

impl Transform {
    /// Select related objects (`parent`, `children`, `descendants`, `ancestors`, or `referencesTo`).
    pub fn select(selector: TransformSelect) -> Self {
        Transform::Select {
            select: vec![selector],
        }
    }

    /// Deduplicate the current object list (`"distinct"`).
    pub fn distinct() -> Self {
        Transform::Distinct("distinct".to_string())
    }

    /// Take a slice of results starting at 0-based `from` with length `count` (`[from, count]`).
    pub fn range(from: usize, count: usize) -> Self {
        Transform::Range {
            range: vec![from, count],
        }
    }

    /// Filter objects whose name contains `substring` (case-insensitive, `["name:contains", substring]`).
    pub fn where_name_contains(substring: impl Into<String>) -> Self {
        Transform::Where {
            where_: vec![
                serde_json::Value::String("name:contains".to_string()),
                serde_json::Value::String(substring.into()),
            ],
        }
    }

    /// Filter objects whose name matches regular expression `regex` (`["name:matches", regex]`).
    pub fn where_name_matches(regex: impl Into<String>) -> Self {
        Transform::Where {
            where_: vec![
                serde_json::Value::String("name:matches".to_string()),
                serde_json::Value::String(regex.into()),
            ],
        }
    }

    /// Filter objects whose type is in `types` (`["type:isIn", [...]]`).
    pub fn where_type_is_in(types: impl IntoIterator<Item = ObjectType>) -> Self {
        let list = types
            .into_iter()
            .map(|t| serde_json::Value::String(t.as_str().to_string()))
            .collect();
        Transform::Where {
            where_: vec![
                serde_json::Value::String("type:isIn".to_string()),
                serde_json::Value::Array(list),
            ],
        }
    }

    /// Filter objects whose category is in `categories` (`["category:isIn", [...]]`).
    pub fn where_category_is_in<I, S>(categories: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let list = categories
            .into_iter()
            .map(|c| serde_json::Value::String(c.into()))
            .collect();
        Transform::Where {
            where_: vec![
                serde_json::Value::String("category:isIn".to_string()),
                serde_json::Value::Array(list),
            ],
        }
    }
}

impl From<TransformSelect> for Transform {
    fn from(selector: TransformSelect) -> Self {
        Transform::select(selector)
    }
}"""


# ─────────────────────────────────────────────────────────────────────────────
# Endpoint files
# ─────────────────────────────────────────────────────────────────────────────

def endpoint_segments(uri):
    return uri.split(".")


def endpoint_base_name(uri):
    segs = endpoint_segments(uri)
    if segs[:2] == ["ak", "wwise"]:
        segs = segs[2:]
    else:
        segs = segs[1:]
    return "".join(pascal_case(s) for s in segs)


def endpoint_file_stem(uri):
    return snake_case(endpoint_segments(uri)[-1])


def endpoint_module_path(uri):
    return endpoint_segments(uri)[:-1]


def method_name(uri):
    return snake_case(endpoint_segments(uri)[-1])


def is_empty_schema(sch):
    return not sch.get("properties") and "$ref" not in sch


def _compose_struct_doc(header, desc):
    desc = clean_doc(desc)
    if desc:
        return f"{header}\n\n{desc}"
    return header


def gen_endpoint_file(ctx):
    base = endpoint_base_name(ctx.uri)
    leaf = pascal_case(endpoint_segments(ctx.uri)[-1])
    endpoint_desc = clean_doc(ctx.doc.get("description", ""))
    args_schema = ctx.doc.get("argsSchema", {})
    options_schema = ctx.doc.get("optionsSchema", {})
    result_schema = ctx.doc.get("resultSchema", {})
    if ctx.uri in RESULT_WRAPPED_OBJECT_RETURN:
        result_schema = {
            "type": "object",
            "properties": {"return": {"$ref": OBJECT_RETURN_REF}},
            "required": ["return"],
        }

    has_args = not is_empty_schema(args_schema)
    has_options = not is_empty_schema(options_schema)
    has_response = not is_empty_schema(result_schema)
    options_is_object_return = options_schema.get("$ref") == OBJECT_RETURN_OPTIONS_REF
    result_is_object_return = result_schema.get("$ref") == OBJECT_RETURN_REF

    sections = [f"// Generated bindings for {ctx.uri}", "", "#![allow(unused_imports)]",
                "#![allow(dead_code)]", "", "use crate::api::generated::primitives::*;", ""]
    exports = []
    aliases = []

    def add_alias(short_name, target_name, doc):
        if short_name != target_name:
            exports.append(short_name)
            aliases.append(f"/// Module-scoped alias for [`{target_name}`].\npub type {short_name} = {target_name};")

    if has_args:
        name = f"{base}Args"
        exports.append(name)
        add_alias(f"{leaf}Args", name, f"Arguments for `{ctx.uri}`.")
        if ctx.uri == "ak.wwise.core.object.get":
            sections.append(gen_object_get_args(ctx, name))
        elif ctx.uri == "ak.wwise.core.object.set":
            item_name = f"{base}Item"
            exports.append(item_name)
            add_alias(f"{leaf}Item", item_name, f"Item for `{ctx.uri}`.")
            sections.append(gen_object_set_args(ctx, name, item_name))
        else:
            ctx.local_root = args_schema
            fields = collect_fields(ctx, args_schema, "args", name)
            ctx.local_root = None
            extra_flatten = bool(args_schema.get("patternProperties"))
            args_doc = _compose_struct_doc(
                f"Arguments for `{ctx.uri}`.",
                schema_description(ctx, args_schema) or endpoint_desc,
            )
            sections.append(emit_struct("args", name, args_doc,
                                        fields, ctx, ctor_order=CTOR_ORDER.get(ctx.uri),
                                        required_order=schema_required(args_schema),
                                        extra_flatten=extra_flatten))
    if has_options:
        name = f"{base}Options"
        exports.append(name)
        add_alias(f"{leaf}Options", name, f"Options for `{ctx.uri}`.")
        opts_doc = _compose_struct_doc(
            f"Options for `{ctx.uri}`.",
            schema_description(ctx, options_schema) or endpoint_desc,
        )
        if options_is_object_return:
            sections.append("\n".join([*doc_lines(opts_doc), f"pub type {name} = ObjectReturnOptions;"]))
        else:
            ctx.local_root = options_schema
            fields = collect_fields(ctx, options_schema, "options", name)
            ctx.local_root = None
            sections.append(emit_struct("options", name, opts_doc, fields, ctx))
    if has_response:
        name = f"{base}Response"
        exports.append(name)
        add_alias(f"{leaf}Response", name, f"Response from `{ctx.uri}`.")
        resp_doc = _compose_struct_doc(
            f"Response from `{ctx.uri}`.",
            schema_description(ctx, result_schema) or endpoint_desc,
        )
        if result_is_object_return:
            sections.append("\n".join([*doc_lines(resp_doc), f"pub type {name} = WwiseObject;"]))
        elif ctx.uri == "ak.wwise.core.object.set":
            item_name = f"{base}Object"
            exports.append(item_name)
            add_alias(f"{leaf}Object", item_name, f"Object entry in `{ctx.uri}` response.")
            sections.append(gen_object_set_response(ctx, name, item_name))
        else:
            ctx.local_root = result_schema
            fields = collect_fields(ctx, result_schema, "response", name)
            ctx.local_root = None
            sections.append(emit_struct("response", name, resp_doc, fields, ctx))

    if aliases:
        sections.append("\n\n".join(aliases))

    return "\n\n".join(sections) + "\n", exports, has_args, has_options, has_response


def gen_topic_file(ctx):
    """Generate the payload struct and subscription options for a WAAPI topic."""
    base = endpoint_base_name(ctx.uri)
    leaf = pascal_case(endpoint_segments(ctx.uri)[-1])
    topic_desc = clean_doc(ctx.doc.get("description", ""))
    options_schema = ctx.doc.get("optionsSchema", {})
    publish_schema = ctx.doc.get("publishSchema", {})

    has_options = not is_empty_schema(options_schema)
    options_is_object_return = options_schema.get("$ref") == OBJECT_RETURN_OPTIONS_REF
    publish_is_empty = is_empty_schema(publish_schema)

    sections = [f"// Generated bindings for topic {ctx.uri}", "",
                "#![allow(unused_imports)]", "#![allow(dead_code)]", "",
                "use crate::api::generated::primitives::*;", ""]
    exports = []
    aliases = []

    def add_alias(short_name, target_name):
        if short_name != target_name:
            exports.append(short_name)
            aliases.append(f"/// Module-scoped alias for [`{target_name}`].\npub type {short_name} = {target_name};")

    payload_name = f"{base}Payload"
    exports.append(payload_name)
    add_alias(f"{leaf}Payload", payload_name)
    payload_doc = _compose_struct_doc(
        f"Payload for `{ctx.uri}`.",
        schema_description(ctx, publish_schema) or topic_desc,
    )
    if publish_is_empty:
        sections.append("\n".join([
            *doc_lines(payload_doc),
            "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
            f"pub struct {payload_name} {{}}",
        ]))
    else:
        ctx.local_root = publish_schema
        fields = collect_fields(ctx, publish_schema, "response", payload_name)
        ctx.local_root = None
        sections.append(emit_struct("response", payload_name, payload_doc, fields, ctx))

    if has_options:
        name = f"{base}Options"
        exports.append(name)
        add_alias(f"{leaf}Options", name)
        opts_doc = _compose_struct_doc(
            f"Subscription options for `{ctx.uri}`.",
            schema_description(ctx, options_schema) or topic_desc,
        )
        if options_is_object_return:
            sections.append("\n".join([*doc_lines(opts_doc), f"pub type {name} = ObjectReturnOptions;"]))
        else:
            ctx.local_root = options_schema
            fields = collect_fields(ctx, options_schema, "options", name)
            ctx.local_root = None
            sections.append(emit_struct("options", name, opts_doc, fields, ctx))

    if aliases:
        sections.append("\n\n".join(aliases))

    return "\n\n".join(sections) + "\n", exports, has_options


def gen_object_get_args(ctx, name):
    return f"""/// Arguments for `ak.wwise.core.object.get`. Specify either a WAQL query or
/// a `from` query; both are optional in the schema but one is required in practice.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct {name} {{
    /// Specifies a query in the WAQL language. See the WAQL introduction documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waql: Option<String>,
    /// The query starting point. Note, this is deprecated. The WAQL argument should be used instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<ObjectQuery>,
    /// Array of sequential transformations chained on the object list returned by `from`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
}}

impl {name} {{
    /// Creates a new [`{name}`] starting from a `from` [`ObjectQuery`].
    pub fn new(from: impl Into<ObjectQuery>) -> Self {{
        Self {{
            waql: None,
            from: Some(from.into()),
            transform: None,
        }}
    }}

    /// Creates a new [`{name}`] from a WAQL query string.
    pub fn from_waql(waql: impl Into<String>) -> Self {{
        Self {{
            waql: Some(waql.into()),
            from: None,
            transform: None,
        }}
    }}

    /// Specifies a query in the WAQL language.
    pub fn waql(mut self, waql: impl Into<String>) -> Self {{
        self.waql = Some(waql.into());
        self
    }}

    /// Sets the query starting point (`from`).
    pub fn from(mut self, from: impl Into<ObjectQuery>) -> Self {{
        self.from = Some(from.into());
        self
    }}

    /// Sets the sequential transformations applied to the object list returned by `from`.
    pub fn transform(mut self, transform: impl Into<Vec<Transform>>) -> Self {{
        self.transform = Some(transform.into());
        self
    }}
}}

impl From<ObjectQuery> for {name} {{
    fn from(from: ObjectQuery) -> Self {{
        Self::new(from)
    }}
}}

impl From<&str> for {name} {{
    fn from(s: &str) -> Self {{
        Self::new(s)
    }}
}}

impl From<String> for {name} {{
    fn from(s: String) -> Self {{
        Self::new(s)
    }}
}}

impl From<&String> for {name} {{
    fn from(s: &String) -> Self {{
        Self::new(s)
    }}
}}

impl From<ObjectType> for {name} {{
    fn from(object_type: ObjectType) -> Self {{
        Self::new(object_type)
    }}
}}

impl From<Vec<ObjectType>> for {name} {{
    fn from(object_types: Vec<ObjectType>) -> Self {{
        Self::new(object_types)
    }}
}}

impl From<Vec<String>> for {name} {{
    fn from(ids: Vec<String>) -> Self {{
        Self::new(ids)
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
                            schema_description(ctx, sub)))
    out = emit_struct("args", item_name,
                      "One object specification in the `objects` array of `object.set`.",
                      fields, ctx, required_order=item_schema.get("required"),
                      extra_flatten=bool(item_schema.get("patternProperties")))
    fields = collect_fields(ctx, args_schema, "args", args_name)
    ctx.local_root = None
    for f in fields:
        if f.wire == "objects":
            f.ty = f"Vec<{item_name}>"
    out += "\n\n"
    args_doc = _compose_struct_doc(
        f"Arguments for `{ctx.uri}`.",
        clean_doc(ctx.doc.get("description", "")),
    )
    out += emit_struct("args", args_name, args_doc, fields, ctx,
                       required_order=args_schema.get("required"))
    out += f"""

impl From<{item_name}> for {args_name} {{
    fn from(item: {item_name}) -> Self {{
        Self::new(vec![item])
    }}
}}"""
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
    resp_doc = _compose_struct_doc(
        f"Response from `{ctx.uri}`.",
        clean_doc(ctx.doc.get("description", "")),
    )
    out += emit_struct("response", resp_name, resp_doc, fields, ctx)
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


def emit_module_tree(tree, dir_path, prefix_parts=()):
    lines = ["// Auto-generated typed WAAPI bindings.",
             "// This file is generated by tools/generate_typed_api.py; do not edit by hand.", ""]
    for name in sorted(k for k in tree if k != "__endpoints__"):
        child_dir = dir_path / name
        child_dir.mkdir(parents=True, exist_ok=True)
        child_parts = (*prefix_parts, name)
        emit_module_tree(tree[name], child_dir, child_parts)
        lines.append(f"/// Typed bindings for the `{'/'.join(child_parts).replace('/', '.')}` WAAPI namespace.")
        if not name.islower():
            lines.append("#[allow(non_snake_case)]")
        lines.append(f"pub mod {name};")
    for uri, info in sorted(tree.get("__endpoints__", [])):
        stem = endpoint_file_stem(uri)
        (dir_path / f"{stem}.rs").write_text(info["content"])
        if stem == dir_path.name:
            lines.append("#[allow(clippy::module_inception)]")
        lines.append(f"mod {stem};")
        if info["exports"]:
            lines.append(f"pub use {stem}::{{")
            for e in info["exports"]:
                lines.append(f"    {e},")
            lines.append("};")
    (dir_path / "mod.rs").write_text("\n".join(lines) + "\n")
