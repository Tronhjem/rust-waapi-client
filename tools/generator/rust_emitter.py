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
    "PropertyValue",
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
            f"{f.rust_name}: impl Into<{f.ty}>"
            if (f.ty in INTO_CTOR_TYPES or f.ty.startswith("Vec<"))
            else f"{f.rust_name}: {f.ty}"
            for f in ctor_fields
        )
        lines.append(f"    /// Creates a new [`{struct_name}`] with the required fields.")
        lines.append(f"    pub fn new({params}) -> Self {{")
        lines.append("        Self {")
        for f in ctor_fields:
            if f.ty in INTO_CTOR_TYPES or f.ty.startswith("Vec<"):
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
                lines.append(f"    pub fn {f.rust_name}(mut self, {f.rust_name}: impl Into<{f.ty}>) -> Self {{")
                lines.append(f"        self.{f.rust_name} = Some({f.rust_name}.into());")
                lines.append("        self")
                lines.append("    }")
                lines.append("")
                lines.append("    /// Alias for [`Self::return_fields`].")
                lines.append(f"    pub fn return_(self, return_fields: impl Into<{f.ty}>) -> Self {{")
                lines.append("        self.return_fields(return_fields)")
                lines.append("    }")
            elif f.ty in INTO_CTOR_TYPES or f.ty.startswith("Vec<"):
                lines.append(f"    pub fn {f.rust_name}(mut self, {f.rust_name}: impl Into<{f.ty}>) -> Self {{")
                lines.append(f"        self.{f.rust_name} = Some({f.rust_name}.into());")
                lines.append("        self")
                lines.append("    }")
            else:
                lines.append(f"    pub fn {f.rust_name}(mut self, {f.rust_name}: {f.ty}) -> Self {{")
                lines.append(f"        self.{f.rust_name} = Some({f.rust_name});")
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


def load_definition_props(def_key: str):
    props = {}
    for sdir in SCHEMA_DIRS:
        defs_path = sdir / "Definitions" / "waapi_definitions.json"
        if not defs_path.exists():
            continue
        waapi_defs = json.loads(defs_path.read_text())
        defn = waapi_defs.get("definitions", {}).get(def_key, {})
        target = defn.get("properties") or defn.get("items", {}).get("properties") or {}
        for wire, sub in target.items():
            props[wire] = sub
    return props


def load_object_return_props():
    return load_definition_props("objectReturn")


def _resolve_local_ref(sdir, node):
    while isinstance(node, dict) and "$ref" in node:
        ref = node["$ref"]
        if "#" not in ref:
            break
        file_part, ptr = ref.split("#", 1)
        defs_path = sdir / "Definitions" / file_part
        if not defs_path.exists():
            break
        target = json.loads(defs_path.read_text())
        for seg in ptr.strip("/").split("/"):
            if isinstance(target, dict) and seg in target:
                target = target[seg]
            else:
                target = {}
                break
        node = target
    return node


def _expand_node_candidates(sdir, node):
    node = _resolve_local_ref(sdir, node)
    if not isinstance(node, dict):
        return []
    out = [node]
    if isinstance(node.get("items"), dict):
        out.extend(_expand_node_candidates(sdir, node["items"]))
    for key in ("oneOf", "anyOf"):
        for branch in node.get(key, []):
            out.extend(_expand_node_candidates(sdir, branch))
    return out


def union_endpoint_enum_values(uris, section: str, prop_path):
    """Union `enum` string values at `section -> prop_path` across all `SCHEMA_DIRS` and `uris`."""
    if isinstance(uris, str):
        uris = [uris]
    seen = set()
    ordered = []
    for sdir in SCHEMA_DIRS:
        for uri in uris:
            p = sdir / f"{uri}.json"
            if not p.exists():
                continue
            doc = json.loads(p.read_text())
            nodes = [doc.get(section, {})]
            steps = [s for s in prop_path if s not in ("properties", "items")]
            for step in steps:
                next_nodes = []
                for raw in nodes:
                    for cand in _expand_node_candidates(sdir, raw):
                        if isinstance(cand.get("properties"), dict) and step in cand["properties"]:
                            next_nodes.append(cand["properties"][step])
                nodes = next_nodes
            for raw in nodes:
                for cand in _expand_node_candidates(sdir, raw):
                    for v in extract_enum_values(cand):
                        if v not in seen:
                            seen.add(v)
                            ordered.append(v)
    if not ordered:
        raise ValueError(f"No enum values found for {uris} {section} {prop_path}")
    return ordered


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
    lines.append("    ///")
    lines.append("    /// A leading `\\` is prepended automatically if not already present, as required by WAAPI.")
    lines.append("    pub fn path(path: impl Into<String>) -> Self {")
    lines.append("        let s = path.into();")
    lines.append("        let s = if s.starts_with('\\\\') { s } else { format!(r\"\\{s}\") };")
    lines.append("        ObjectQuery::Path(vec![s])")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Query multiple objects by backslash-separated hierarchy paths.")
    lines.append("    ///")
    lines.append("    /// A leading `\\` is prepended automatically to each path if not already present.")
    lines.append("    pub fn paths<I, S>(paths: I) -> Self")
    lines.append("    where")
    lines.append("        I: IntoIterator<Item = S>,")
    lines.append("        S: Into<String>,")
    lines.append("    {")
    lines.append("        ObjectQuery::Path(paths.into_iter().map(|p| {")
    lines.append("            let s = p.into();")
    lines.append("            if s.starts_with('\\\\') { s } else { format!(r\"\\{s}\") }")
    lines.append("        }).collect())")
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
    curve_shape_vals = union_definition_enum_values(SCHEMA_DIRS, "waapi_definitions.json", "curveShape")
    curve_type_vals = union_endpoint_enum_values(
        ["ak.wwise.core.object.getAttenuationCurve", "ak.wwise.core.object.setAttenuationCurve"],
        "argsSchema",
        ["properties", "curveType"],
    )
    curve_use_vals = union_endpoint_enum_values(
        "ak.wwise.core.object.getAttenuationCurve",
        "resultSchema",
        ["properties", "use"],
    )
    paste_mode_vals = union_endpoint_enum_values(
        "ak.wwise.core.object.pasteProperties",
        "argsSchema",
        ["properties", "pasteMode"],
    )
    migration_vals = union_endpoint_enum_values(
        "ak.wwise.ui.project.open",
        "argsSchema",
        ["properties", "onMigrationRequired"],
    )

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
    pub fn new(listener: GameObjectRef, aux_bus: SoundEngineObjectRef, control_value: f64) -> Self {
        Self { listener, aux_bus, control_value }
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
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl From<Position> for MultiPosition {
    fn from(position: Position) -> Self {
        Self { position }
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
                  union_endpoint_enum_values(["ak.wwise.core.object.getPropertyInfo"], "resultSchema", ("supports", "rtpc"))),
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
        emit_enum("BlendFadeMode", "Fade mode of a blend track edge.",
                  extract_enum_values(load_definition_props("blendTrackEdge")["fadeMode"])),
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
        emit_enum("OnCurveUpdate", "Action when a Game Parameter range is modified.",
                  union_endpoint_enum_values(["ak.wwise.core.gameParameter.setRange"], "argsSchema", ("onCurveUpdate",))),
        emit_enum("SoundBankInclusionType", "Inclusion type used by `soundbank.generate`.",
                  union_endpoint_enum_values(["ak.wwise.core.soundbank.generate"], "argsSchema", ("soundbanks", "inclusions"))),
        emit_enum("SoundBankInclusionFilter", "Inclusion relation used by `soundbank.get/setInclusions`.",
                  union_endpoint_enum_values(["ak.wwise.core.soundbank.getInclusions", "ak.wwise.core.soundbank.setInclusions"], "resultSchema", ("inclusions", "filter"))),
        emit_enum("SoundBankInclusionOperation", "How `soundbank.setInclusions` modifies the inclusion list.",
                  union_endpoint_enum_values(["ak.wwise.core.soundbank.setInclusions"], "argsSchema", ("operation",))),
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
    pub fn new(object: ObjectRef) -> Self {
        Self { object, filter: Vec::new() }
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
    pub fn events(mut self, events: impl Into<Vec<ObjectRef>>) -> Self {
        self.events = Some(events.into());
        self
    }

    /// Sets the auxiliary busses to include in the SoundBank.
    pub fn aux_busses(mut self, aux_busses: impl Into<Vec<ObjectRef>>) -> Self {
        self.aux_busses = Some(aux_busses.into());
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
                  union_definition_enum_values(SCHEMA_DIRS, "waapi_definitions.json", "logCentralChannel")),
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
    pub fn new(platform: PlatformRef, input: impl Into<String>) -> Self {
        Self { platform, input: input.into(), output: None }
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


def gen_profiler_and_domain_types(schemas, endpoints_docs):
    waapi_defs = schemas.load_definitions("waapi_definitions.json")
    get_ctx = endpoints_docs["ak.wwise.core.object.get"]
    loaded_media_ctx = endpoints_docs["ak.wwise.core.profiler.getLoadedMedia"]
    streamed_media_ctx = endpoints_docs["ak.wwise.core.profiler.getStreamedMedia"]

    voice_fields = collect_fields(get_ctx, {"properties": load_definition_props("voiceReturn")}, "response", "ProfilerVoice")
    bus_fields = collect_fields(get_ctx, {"properties": load_definition_props("busReturn")}, "response", "ProfilerBus")
    audio_obj_fields = collect_fields(get_ctx, {"properties": load_definition_props("audioObjectReturn")}, "response", "ProfilerAudioObject")
    for f in audio_obj_fields:
        if f.wire == "metadata":
            f.ty = "Vec<AudioObjectMetadata>"

    loaded_media_sch = loaded_media_ctx.doc["resultSchema"]["properties"]["return"]["items"]
    loaded_media_fields = collect_fields(loaded_media_ctx, loaded_media_sch, "response", "ProfilerLoadedMedia")
    streamed_media_sch = streamed_media_ctx.doc["resultSchema"]["properties"]["return"]["items"]
    streamed_media_fields = collect_fields(streamed_media_ctx, streamed_media_sch, "response", "ProfilerStreamedMedia")

    parts = [
        emit_enum(
            "ProfilerCursor",
            "A profiler cursor selector (`profilerCursor`).",
            union_definition_enum_values(SCHEMA_DIRS, "waapi_definitions.json", "profilerCursor"),
        ),
        """/// Profiler cursor time (`cursorTime`): either an explicit timestamp in milliseconds or a named cursor (`"user"` / `"capture"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CursorTime {
    /// Timestamp in milliseconds (`>= 0`).
    TimeMs(i64),
    /// Named profiler cursor (`"user"` or `"capture"`).
    Cursor(ProfilerCursor),
}

impl CursorTime {
    /// Specifies an explicit capture timestamp in milliseconds.
    pub const fn ms(ms: i64) -> Self {
        Self::TimeMs(ms)
    }

    /// Selects the user profiler cursor (`"user"`).
    pub const fn user() -> Self {
        Self::Cursor(ProfilerCursor::User)
    }

    /// Selects the capture profiler cursor (`"capture"`).
    pub const fn capture() -> Self {
        Self::Cursor(ProfilerCursor::Capture)
    }
}

impl From<i64> for CursorTime {
    fn from(ms: i64) -> Self {
        Self::TimeMs(ms)
    }
}

impl From<ProfilerCursor> for CursorTime {
    fn from(cursor: ProfilerCursor) -> Self {
        Self::Cursor(cursor)
    }
}""",
        emit_enum(
            "CursorMovePosition",
            "Direction or boundary for `ak.wwise.core.profiler.moveCursor`.",
            union_endpoint_enum_values("ak.wwise.core.profiler.moveCursor", "argsSchema", ["properties", "position"]),
        ),
        emit_enum(
            "ProfilerDataType",
            "Profiler data category for `ak.wwise.core.profiler.enableProfilerData`.",
            union_endpoint_enum_values("ak.wwise.core.profiler.enableProfilerData", "argsSchema", ("dataTypes", "dataType")),
        ),
        """/// Entry configuring a profiler data type in `ak.wwise.core.profiler.enableProfilerData`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProfilerDataTypeEntry {
    /// Profiler data category to configure.
    #[serde(rename = "dataType")]
    pub data_type: ProfilerDataType,
    /// Whether capture of this data category is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl ProfilerDataTypeEntry {
    /// Creates a new [`ProfilerDataTypeEntry`] for `data_type`.
    pub fn new(data_type: ProfilerDataType) -> Self {
        Self { data_type, enable: None }
    }

    /// Sets whether capture of this data category is enabled.
    pub fn enable(mut self, enable: bool) -> Self {
        self.enable = Some(enable);
        self
    }
}""",
        """/// A registered game object returned by `ak.wwise.core.profiler.getGameObjects`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProfilerGameObject {
    /// Unsigned 64-bit game object ID.
    pub id: u64,
    /// Name of the game object.
    pub name: String,
    /// Capture timestamp (ms) when the game object was registered.
    #[serde(rename = "registerTime")]
    pub register_time: i32,
    /// Capture timestamp (ms) when the game object was unregistered (`0` if still active).
    #[serde(rename = "unregisterTime")]
    pub unregister_time: i32,
}""",
        """/// An active RTPC value returned by `ak.wwise.core.profiler.getRTPCs`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProfilerRtpc {
    /// GUID or identifier of the Game Parameter.
    pub id: String,
    /// Name of the Game Parameter.
    pub name: String,
    /// Current value of the Game Parameter at the queried cursor time.
    pub value: f64,
    /// Game object ID if the RTPC is game-object-scoped.
    #[serde(default, rename = "gameObjectId", skip_serializing_if = "Option::is_none")]
    pub game_object_id: Option<GameObjectRef>,
}""",
        """/// A CPU usage entry returned by `ak.wwise.core.profiler.getCpuUsage`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProfilerCpuUsageItem {
    /// Identifier of the profiled element.
    pub id: i64,
    /// Category or type of the profiled element.
    #[serde(rename = "type")]
    pub item_type: String,
    /// Display name of the profiled element.
    #[serde(rename = "elementName")]
    pub element_name: String,
    /// Number of active instances of the element.
    pub instances: i64,
    /// Inclusive CPU usage percentage.
    #[serde(rename = "percentInclusive")]
    pub percent_inclusive: f64,
    /// Exclusive CPU usage percentage.
    #[serde(rename = "percentExclusive")]
    pub percent_exclusive: f64,
    /// Inclusive CPU time in milliseconds.
    #[serde(rename = "millisecondsInclusive")]
    pub milliseconds_inclusive: f64,
    /// Exclusive CPU time in milliseconds.
    #[serde(rename = "millisecondsExclusive")]
    pub milliseconds_exclusive: f64,
}""",
        """/// A performance monitor counter value returned by `ak.wwise.core.profiler.getPerformanceMonitor`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProfilerPerformanceCounter {
    /// Identifier of the performance counter.
    pub id: String,
    /// Display name of the performance counter.
    pub name: String,
    /// Value of the performance counter at the queried cursor time.
    pub value: f64,
}""",
        emit_struct("response", "ProfilerVoice", "A profiler voice entry (`voiceReturn`).", voice_fields, get_ctx),
        emit_struct("response", "ProfilerBus", "A profiler bus entry (`busReturn`).", bus_fields, get_ctx),
        """/// Metadata item attached to a [`ProfilerAudioObject`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AudioObjectMetadata {
    /// Class ID of the metadata plug-in.
    #[serde(rename = "metadataClassID")]
    pub metadata_class_id: u32,
    /// Name of the metadata plug-in.
    #[serde(default, rename = "metadataName", skip_serializing_if = "Option::is_none")]
    pub metadata_name: Option<String>,
    /// Authoring GUID of the metadata source object.
    #[serde(default, rename = "sourceID", skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// 32-bit Short ID of the metadata source object.
    #[serde(rename = "sourceShortID")]
    pub source_short_id: u32,
    /// Name of the metadata source object.
    #[serde(default, rename = "sourceName", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}""",
        emit_struct("response", "ProfilerAudioObject", "A profiler audio object entry (`audioObjectReturn`).",
                    audio_obj_fields, get_ctx),
        emit_struct("response", "ProfilerLoadedMedia", "A loaded media entry returned by `ak.wwise.core.profiler.getLoadedMedia`.",
                    loaded_media_fields, loaded_media_ctx),
        emit_struct("response", "ProfilerStreamedMedia", "A streamed media entry returned by `ak.wwise.core.profiler.getStreamedMedia`.",
                    streamed_media_fields, streamed_media_ctx),
        emit_enum(
            "MeterSource",
            "Audio signal source for a profiler meter (`getMeters`).",
            union_endpoint_enum_values("ak.wwise.core.profiler.getMeters", "resultSchema", ["properties", "meters", "items", "properties", "source"]),
        ),
        emit_enum(
            "MeterConfigurationType",
            "Channel configuration type for a profiler meter (`getMeters`).",
            union_endpoint_enum_values("ak.wwise.core.profiler.getMeters", "resultSchema", ["properties", "meters", "items", "properties", "configurationType"]),
        ),
        """/// Per-channel meter reading inside a [`ProfilerMeter`].
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MeterChannel {
    /// Channel name (e.g. `"FL"`, `"FR"`, `"Object 0"`, `"H1V1"`).
    pub name: String,
    /// Meter value for this channel in decibels.
    pub value: f64,
}""",
        """/// Meter data for a registered bus or device returned by `ak.wwise.core.profiler.getMeters`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfilerMeter {
    /// Bus or device object being metered.
    pub object: WwiseObject,
    /// Audio signal source of the meter (`Bus`, `DeviceMainMix`, `DevicePassthrough`, `DeviceSystemAudioObjects`).
    pub source: MeterSource,
    /// Channel configuration name (e.g. `"2.0"`, `"7.1.4"`).
    #[serde(rename = "configurationName")]
    pub configuration_name: String,
    /// Bitmask of the speaker configuration.
    #[serde(rename = "configurationMask")]
    pub configuration_mask: i64,
    /// Channel configuration type (`Anonymous`, `Standard`, `Ambisonic`, `Objects`).
    #[serde(rename = "configurationType")]
    pub configuration_type: MeterConfigurationType,
    /// Meter readings for each channel.
    #[serde(default)]
    pub channels: Vec<MeterChannel>,
}""",
        """/// A single parameter contribution (e.g. Volume, LPF, HPF) on a [`VoiceContributionObject`].
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VoiceContributionParameter {
    /// Property or parameter type name.
    #[serde(default, rename = "propertyType", skip_serializing_if = "Option::is_none")]
    pub property_type: Option<String>,
    /// Reason or driver category for this parameter contribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Driver identifier (such as an RTPC or Attenuation name/GUID) responsible for the contribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Driver input value (number or string), if applicable.
    #[serde(default, rename = "driverValue", skip_serializing_if = "Option::is_none")]
    pub driver_value: Option<serde_json::Value>,
    /// Numeric contribution value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}""",
        """/// An object node in the voice contribution hierarchy (`ak.wwise.core.profiler.getVoiceContributions`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VoiceContributionObject {
    /// Name of the contributing Wwise object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Resulting volume contribution in dB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    /// Resulting low-pass filter contribution.
    #[serde(default, rename = "LPF", skip_serializing_if = "Option::is_none")]
    pub lpf: Option<f64>,
    /// Resulting high-pass filter contribution.
    #[serde(default, rename = "HPF", skip_serializing_if = "Option::is_none")]
    pub hpf: Option<f64>,
    /// Resulting dual-shelf filter contribution (Wwise 2025+).
    #[serde(default, rename = "DSF", skip_serializing_if = "Option::is_none")]
    pub dsf: Option<f64>,
    /// Optional index of the contributing object or slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    /// Parameter contributions originating from this object.
    #[serde(default)]
    pub parameters: Vec<VoiceContributionParameter>,
    /// Child contributing objects in the hierarchy.
    #[serde(default)]
    pub children: Vec<VoiceContributionObject>,
}""",
        """/// Full voice contribution breakdown returned by `ak.wwise.core.profiler.getVoiceContributions`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VoiceContributions {
    /// Total resulting volume in dB.
    pub volume: f64,
    /// Total resulting low-pass filter value.
    #[serde(rename = "LPF")]
    pub lpf: f64,
    /// Total resulting high-pass filter value.
    #[serde(rename = "HPF")]
    pub hpf: f64,
    /// Total resulting dual-shelf filter value (Wwise 2025+).
    #[serde(default, rename = "DSF", skip_serializing_if = "Option::is_none")]
    pub dsf: Option<f64>,
    /// Root contributing objects in the voice hierarchy.
    #[serde(default)]
    pub objects: Vec<VoiceContributionObject>,
}""",
        emit_enum(
            "CaptureLogType",
            "Capture log item type filter for `ak.wwise.core.profiler.captureLog.itemAdded`.",
            union_endpoint_enum_values("ak.wwise.core.profiler.captureLog.itemAdded", "optionsSchema", ["properties", "types"]),
        ),
        emit_enum(
            "CaptureLogSeverity",
            "Severity of a capture log item (`ak.wwise.core.profiler.captureLog.itemAdded`).",
            union_endpoint_enum_values("ak.wwise.core.profiler.captureLog.itemAdded", "publishSchema", ["properties", "severity"]),
        ),
        emit_enum(
            "StructureChangeType",
            "Kind of hierarchy change reported in `ak.wwise.core.object.structureChanged`.",
            union_endpoint_enum_values("ak.wwise.core.object.structureChanged", "publishSchema", ["properties", "objects", "items", "properties", "changes", "items", "properties", "type"]),
        ),
        """/// Old and new parent references for a `parentChange` entry in [`StructureChange`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructureParentChange {
    /// Previous parent object.
    #[serde(default, rename = "oldParent", skip_serializing_if = "Option::is_none")]
    pub old_parent: Option<WwiseObject>,
    /// New parent object.
    #[serde(default, rename = "newParent", skip_serializing_if = "Option::is_none")]
    pub new_parent: Option<WwiseObject>,
}""",
        """/// Old and new owner references for an `ownerChange` entry in [`StructureChange`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructureOwnerChange {
    /// Name of the reference or property that changed owner.
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    /// Previous owner object.
    #[serde(default, rename = "oldOwner", skip_serializing_if = "Option::is_none")]
    pub old_owner: Option<WwiseObject>,
    /// New owner object.
    #[serde(default, rename = "newOwner", skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<WwiseObject>,
}""",
        """/// Old and new names for a `nameChange` entry in [`StructureChange`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StructureNameChange {
    /// Previous object name.
    #[serde(rename = "oldName")]
    pub old_name: String,
    /// New object name.
    #[serde(rename = "newName")]
    pub new_name: String,
}""",
        """/// Old and new notes for a `notesChange` entry in [`StructureChange`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StructureNotesChange {
    /// Previous notes string (Wwise schema wire key `"oldName"`).
    #[serde(rename = "oldName")]
    pub old_name: String,
    /// New notes string (Wwise schema wire key `"newName"`).
    #[serde(rename = "newName")]
    pub new_name: String,
}""",
        """/// A single structural change reported on a [`StructureChangedObject`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructureChange {
    /// Kind of structural change.
    #[serde(rename = "type")]
    pub change_type: StructureChangeType,
    /// Parent change details when `change_type` is [`StructureChangeType::ParentChange`].
    #[serde(default, rename = "parentChange", skip_serializing_if = "Option::is_none")]
    pub parent_change: Option<StructureParentChange>,
    /// Owner change details when `change_type` is [`StructureChangeType::OwnerChange`].
    #[serde(default, rename = "ownerChange", skip_serializing_if = "Option::is_none")]
    pub owner_change: Option<StructureOwnerChange>,
    /// Name change details when `change_type` is [`StructureChangeType::NameChange`].
    #[serde(default, rename = "nameChange", skip_serializing_if = "Option::is_none")]
    pub name_change: Option<StructureNameChange>,
    /// Notes change details when `change_type` is [`StructureChangeType::NotesChange`].
    #[serde(default, rename = "notesChange", skip_serializing_if = "Option::is_none")]
    pub notes_change: Option<StructureNotesChange>,
}""",
        """/// An affected object and its changes in `ak.wwise.core.object.structureChanged`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructureChangedObject {
    /// The affected Wwise object.
    pub object: WwiseObject,
    /// Structural changes applied to this object.
    #[serde(default)]
    pub changes: Vec<StructureChange>,
}""",
        """/// Display metadata for a property returned by `ak.wwise.core.object.getPropertyInfo`.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyDisplayInfo {
    /// Human-readable display name of the property.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Display ordering index of the property.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// UI property group name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}""",
        """/// Display formatting flags for a property in Wwise Authoring (`object.getPropertyInfo`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyUiDisplayAs {
    /// Whether the property is displayed as a Left/Right balance mix slider.
    #[serde(default, rename = "lrMix", skip_serializing_if = "Option::is_none")]
    pub lr_mix: Option<bool>,
    /// Whether the property is displayed as a musical MIDI note name.
    #[serde(default, rename = "musicNote", skip_serializing_if = "Option::is_none")]
    pub music_note: Option<bool>,
    /// Whether the property is displayed as a bitfield.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitfield: Option<bool>,
    /// Whether the property is displayed as a percentage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<bool>,
}""",
        emit_enum("PropertyDataMeaning", "Semantic meaning of a property's numeric value (`object.getPropertyInfo`).",
                  union_endpoint_enum_values(["ak.wwise.core.object.getPropertyInfo"], "resultSchema", ("ui", "dataMeaning"))),
        """/// Numeric value constraints and formatting metadata for a property UI (`object.getPropertyInfo`).
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyUiValueInfo {
    /// Minimum UI value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum UI value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Number of decimal places displayed in the UI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimals: Option<f64>,
    /// Standard UI increment step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    /// Fine UI increment step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fine: Option<f64>,
    /// Numeric threshold below which the UI displays negative infinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infinity: Option<f64>,
}""",
        """/// UI metadata for a property returned by `ak.wwise.core.object.getPropertyInfo`.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyUiInfo {
    /// Numeric value range and step details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<PropertyUiValueInfo>,
    /// Special display formatting flags (`lrMix`, `musicNote`, `bitfield`, `percentage`).
    #[serde(default, rename = "displayAs", skip_serializing_if = "Option::is_none")]
    pub display_as: Option<PropertyUiDisplayAs>,
    /// Semantic unit or meaning of the property value (`None`, `Frequency`, `Decibels`, `PitchCents`).
    #[serde(default, rename = "dataMeaning", skip_serializing_if = "Option::is_none")]
    pub data_meaning: Option<PropertyDataMeaning>,
    /// Whether the property updates automatically while dragging in the UI.
    #[serde(default, rename = "autoUpdate", skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,
}""",
        emit_enum("PropertyDependencyType", "Type of property dependency (`object.getPropertyInfo`).",
                  union_endpoint_enum_values(["ak.wwise.core.object.getPropertyInfo"], "resultSchema", ("dependencies", "type"))),
        """/// Value or reference restriction metadata for a property returned by `ak.wwise.core.object.getPropertyInfo`.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyRestriction {
    /// Kind of value restriction (`"range"`, `"reference"`, `"enum"`), or `None` when unrestricted (`{}`).
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub restriction_type: Option<String>,
    /// Minimum allowed value when `restriction_type` is `"range"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum allowed value when `restriction_type` is `"range"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Reference restrictions when `restriction_type` is `"reference"`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<serde_json::Value>,
    /// Enumerated allowed values (`{value, displayName}`) when `restriction_type` is `"enum"`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<serde_json::Value>,
}""",
        """/// Condition activating a [`PropertyDependency`] (`object.getPropertyInfo`).
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyDependencyCondition {
    /// For reference dependencies, defines whether the target reference must be set.
    #[serde(default, rename = "referenceIsSet", skip_serializing_if = "Option::is_none")]
    pub reference_is_set: Option<bool>,
    /// For property dependencies, defines the restriction on which the target property must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restriction: Option<PropertyRestriction>,
}""",
        """/// Dependency rule between properties returned by `ak.wwise.core.object.getPropertyInfo`.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyDependency {
    /// Type of dependency (`override`, `property`, `reference`, `objectType`, `isMasterBus`).
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub dependency_type: Option<PropertyDependencyType>,
    /// Boolean value used by `isMasterBus` dependencies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
    /// Name of the action that is performed when the conditions are met (e.g. `"enable"`, `"show"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Name of the target property or reference to which the dependency is applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Name of the context in which the dependency is evaluated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Conditions related to the dependency.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PropertyDependencyCondition>,
}""",
        """/// Default work unit reference returned by `ak.wwise.core.getProjectInfo` (Wwise 2025+).
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct DefaultWorkUnitInfo {
    /// Work unit GUID.
    pub id: String,
    /// Work unit name in the project.
    pub name: String,
    /// Work unit project hierarchy path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Absolute file system path of the work unit `.wwu` file.
    #[serde(default, rename = "filePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}""",
        """/// Available remote console entry returned by `ak.wwise.core.remote.getAvailableConsoles`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RemoteConsoleInfo {
    /// Name of the remote console or executable.
    pub name: String,
    /// Target platform name.
    pub platform: String,
    /// Custom platform name configured in the project.
    #[serde(rename = "customPlatform")]
    pub custom_platform: String,
    /// IP address or hostname of the remote target.
    pub host: String,
    /// Command port number.
    #[serde(rename = "commandPort")]
    pub command_port: u16,
    /// Connected application name.
    #[serde(rename = "appName")]
    pub app_name: String,
}""",
        """/// Connected console details returned by `ak.wwise.core.remote.getConnectionStatus`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConnectedConsoleInfo {
    /// Name of the remote console or executable.
    pub name: String,
    /// Target platform name.
    pub platform: String,
    /// Custom platform name configured in the project.
    #[serde(rename = "customPlatform")]
    pub custom_platform: String,
    /// IP address or hostname of the remote target.
    pub host: String,
    /// Connected application name.
    #[serde(rename = "appName")]
    pub app_name: String,
}""",
        emit_enum("SourceControlFileFilter", "Filter for `ak.wwise.core.sourceControl.getSourceFiles`.",
                  union_endpoint_enum_values(["ak.wwise.core.sourceControl.getSourceFiles"], "argsSchema", ("filter",))),
        emit_enum("SourceControlReturnField", "Return field selector for `ak.wwise.core.sourceControl.getSourceFiles`.",
                  union_endpoint_enum_values(["ak.wwise.core.sourceControl.getSourceFiles"], "optionsSchema", ("return",))),
        """/// A source file entry returned by `ak.wwise.core.sourceControl.getSourceFiles`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceControlSourceFile {
    /// Directory path containing the source file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// Full file path or file name on disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Whether the file is used in the project.
    #[serde(default, rename = "isUsed", skip_serializing_if = "Option::is_none")]
    pub is_used: Option<bool>,
    /// Wwise objects referencing this source file.
    #[serde(default)]
    pub usage: Vec<WwiseObject>,
    /// Whether the file is missing on disk.
    #[serde(default, rename = "isMissing", skip_serializing_if = "Option::is_none")]
    pub is_missing: Option<bool>,
}""",
        """/// File status entry returned by `ak.wwise.core.sourceControl.getStatus`.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceControlStatus {
    /// Source control owner or lock holder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Source control status text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}""",
        emit_enum("MediaPoolFilterType", "Filter category for `ak.wwise.core.mediaPool.get`.",
                  union_endpoint_enum_values(["ak.wwise.core.mediaPool.get"], "argsSchema", ("filters", "type"))),
        emit_enum("MediaPoolFilterOperator", "Comparison operator for a `field` filter in `ak.wwise.core.mediaPool.get`.",
                  union_endpoint_enum_values(["ak.wwise.core.mediaPool.get"], "argsSchema", ("filters", "operator"))),
        """/// A filter predicate entry for `ak.wwise.core.mediaPool.get`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MediaPoolFilterEntry {
    /// Type of filter (`field`, `audioDescription`, or `audioSimilarity`).
    #[serde(rename = "type")]
    pub filter_type: MediaPoolFilterType,
    /// Media Pool field name to compare when `filter_type` is [`MediaPoolFilterType::Field`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Comparison operator when `filter_type` is [`MediaPoolFilterType::Field`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MediaPoolFilterOperator>,
    /// Value to compare against (string, number, or similarity reference object).
    pub value: serde_json::Value,
    /// Optional weight for similarity/description matching (`0.0..=1.0`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

impl MediaPoolFilterEntry {
    /// Creates a new [`MediaPoolFilterEntry`] with `filter_type` and `value`.
    pub fn new(filter_type: MediaPoolFilterType, value: impl serde::Serialize) -> Self {
        Self {
            filter_type,
            field: None,
            operator: None,
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            weight: None,
        }
    }

    /// Creates a `field` filter entry comparing `field` with `operator` against `value`.
    pub fn field_filter(
        field: impl Into<String>,
        operator: MediaPoolFilterOperator,
        value: impl serde::Serialize,
    ) -> Self {
        Self {
            filter_type: MediaPoolFilterType::Field,
            field: Some(field.into()),
            operator: Some(operator),
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            weight: None,
        }
    }

    /// Sets the Media Pool field name.
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Sets the field comparison operator.
    pub fn operator(mut self, operator: MediaPoolFilterOperator) -> Self {
        self.operator = Some(operator);
        self
    }

    /// Sets the similarity/description weight (`0.0..=1.0`).
    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = Some(weight);
        self
    }
}""",
        """/// A log entry returned by `ak.wwise.core.audio.import`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AudioImportLogItem {
    /// Severity level of the import log entry.
    pub severity: LogSeverity,
    /// Human-readable log message.
    pub message: String,
    /// Index of the import command to which the log entry applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
}""",
        emit_enum("AudioImportedOperation", "Operation reported by `ak.wwise.core.audio.imported`.",
                  union_endpoint_enum_values(["ak.wwise.core.audio.imported"], "publishSchema", ("operation",))),
        """/// Base64-encoded SoundBank data payload in `ak.wwise.core.soundbank.generated`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SoundBankGeneratedBankData {
    /// Size of the generated SoundBank in bytes.
    pub size: i64,
    /// Base64-encoded SoundBank binary data.
    pub data: String,
}""",
        """/// A plug-in library entry inside [`SoundBankPluginInfo`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SoundBankPluginLib {
    /// Plug-in category or type.
    #[serde(rename = "Type")]
    pub plugin_type: String,
    /// Library display name.
    #[serde(rename = "LibName")]
    pub lib_name: String,
    /// Numeric or hex library identifier.
    #[serde(rename = "LibId")]
    pub lib_id: String,
    /// Dynamic library file name (`.dll` / `.so` / `.dylib`), if applicable.
    #[serde(default, rename = "DLL", skip_serializing_if = "Option::is_none")]
    pub dll: Option<String>,
    /// Static library file name (`.lib` / `.a`), if applicable.
    #[serde(default, rename = "StaticLib", skip_serializing_if = "Option::is_none")]
    pub static_lib: Option<String>,
}""",
        """/// Plug-in usage metadata emitted in `ak.wwise.core.soundbank.generated`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SoundBankPluginInfo {
    /// Target platform name.
    #[serde(rename = "Platform")]
    pub platform: String,
    /// Base platform name.
    #[serde(rename = "BasePlatform")]
    pub base_platform: String,
    /// Plug-in libraries referenced by the generated SoundBank.
    #[serde(default, rename = "PluginLibs")]
    pub plugin_libs: Vec<SoundBankPluginLib>,
}""",
        emit_enum("ToneChannelConfig", "Channel configuration for `ak.wwise.debug.generateToneWAV`.",
                  union_endpoint_enum_values(["ak.wwise.debug.generateToneWAV"], "argsSchema", ("channelConfig",))),
        emit_enum("ToneBitDepth", "Sample bit depth for `ak.wwise.debug.generateToneWAV`.",
                  union_endpoint_enum_values(["ak.wwise.debug.generateToneWAV"], "argsSchema", ("bitDepth",))),
        emit_enum("ToneWaveform", "Waveform type for `ak.wwise.debug.generateToneWAV`.",
                  union_endpoint_enum_values(["ak.wwise.debug.generateToneWAV"], "argsSchema", ("waveform",))),
        """/// A WAV cue marker definition for `ak.wwise.debug.generateToneWAV`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WavMarker {
    /// Position of the marker in seconds.
    pub position: f64,
    /// Optional marker text label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl WavMarker {
    /// Creates a new [`WavMarker`] at `position` (in seconds).
    pub fn new(position: f64) -> Self {
        Self { position, label: None }
    }

    /// Sets the marker text label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}""",
        """/// Screen rectangle region for `ak.wwise.ui.captureScreen`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ScreenRect {
    /// Left X coordinate in pixels.
    pub x: i64,
    /// Top Y coordinate in pixels.
    pub y: i64,
    /// Width in pixels.
    pub width: i64,
    /// Height in pixels.
    pub height: i64,
}

impl ScreenRect {
    /// Creates a new [`ScreenRect`].
    pub const fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Self { x, y, width, height }
    }
}""",
        """/// Window dimensions for `ak.wwise.ui.window.create`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WindowSize {
    /// Width in pixels.
    pub width: i64,
    /// Height in pixels.
    pub height: i64,
}

impl WindowSize {
    /// Creates a new [`WindowSize`].
    pub const fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }
}""",
        """/// An active view instance returned by `ak.wwise.ui.layout.getViewInstances`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ViewInstance {
    /// Internal view type name.
    #[serde(rename = "viewName")]
    pub view_name: String,
    /// Unique identifier (GUID) of the view instance.
    #[serde(rename = "viewID")]
    pub view_id: String,
    /// Whether the view instance is docked inside a layout.
    #[serde(rename = "viewIsDocked")]
    pub view_is_docked: bool,
    /// Localized display name of the view.
    #[serde(default, rename = "viewDisplayName", skip_serializing_if = "Option::is_none")]
    pub view_display_name: Option<String>,
}""",
        """/// A view type definition returned by `ak.wwise.ui.layout.getViewTypes`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ViewTypeInfo {
    /// Internal view type identifier.
    #[serde(rename = "viewName")]
    pub view_name: String,
    /// Localized display name of the view type.
    #[serde(rename = "viewDisplayName")]
    pub view_display_name: String,
}""",
        """/// A platform entry for `ak.wwise.ui.project.create` (`platformToCreateArg`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlatformTemplateEntry {
    /// Optional custom platform name in the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Base deployment platform name (e.g. `"Windows"`, `"Mac"`, `"PS4"`).
    #[serde(rename = "basePlatform")]
    pub base_platform: String,
}

impl PlatformTemplateEntry {
    /// Creates a new [`PlatformTemplateEntry`] for `base_platform`.
    pub fn new(base_platform: impl Into<String>) -> Self {
        Self { name: None, base_platform: base_platform.into() }
    }

    /// Sets a custom platform name for this entry.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}""",
        """/// Platforms to create in `ak.wwise.ui.project.create` (`platformsToCreateArg`): either a single platform entry or a list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PlatformsToCreate {
    /// A single platform definition.
    Single(PlatformTemplateEntry),
    /// Multiple platform definitions.
    Multiple(Vec<PlatformTemplateEntry>),
}

impl PlatformsToCreate {
    /// Creates a [`PlatformsToCreate::Single`] entry.
    pub fn single(entry: PlatformTemplateEntry) -> Self {
        Self::Single(entry)
    }

    /// Creates a [`PlatformsToCreate::Multiple`] list of platform entries.
    pub fn multiple(entries: impl Into<Vec<PlatformTemplateEntry>>) -> Self {
        Self::Multiple(entries.into())
    }
}""",
        """/// A widget attribute key/value pair (`widgetAttributeEntry`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WidgetAttributeEntry {
    /// Widget attribute name.
    pub attribute: String,
    /// Widget attribute value.
    pub value: serde_json::Value,
}

impl WidgetAttributeEntry {
    /// Creates a new [`WidgetAttributeEntry`].
    pub fn new(attribute: impl Into<String>, value: impl serde::Serialize) -> Self {
        Self {
            attribute: attribute.into(),
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        }
    }
}""",
        """/// A widget state query entry for `ak.wwise.ui.model.getState`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WidgetStateQuery {
    /// Identifier of the widget to query.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    /// List of attribute names to retrieve for the widget.
    pub state: Vec<String>,
}

impl WidgetStateQuery {
    /// Creates a new [`WidgetStateQuery`] for `widget_id`.
    pub fn new(widget_id: impl Into<String>, state: impl Into<Vec<String>>) -> Self {
        Self { widget_id: widget_id.into(), state: state.into() }
    }
}""",
        """/// A widget state result entry returned by `ak.wwise.ui.model.getState`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WidgetStateResult {
    /// Identifier of the queried widget.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    /// Retrieved attribute values for the widget.
    #[serde(default)]
    pub state: Vec<WidgetAttributeEntry>,
}""",
        """/// A widget state update entry for `ak.wwise.ui.model.setState`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WidgetStateUpdate {
    /// Identifier of the widget to update.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    /// Attribute values to apply to the widget.
    pub state: Vec<WidgetAttributeEntry>,
}

impl WidgetStateUpdate {
    /// Creates a new [`WidgetStateUpdate`] for `widget_id`.
    pub fn new(widget_id: impl Into<String>, state: impl Into<Vec<WidgetAttributeEntry>>) -> Self {
        Self { widget_id: widget_id.into(), state: state.into() }
    }
}""",
        """/// Either a single string or a list of strings (`stringOrArrayOfString`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum StringOrVec {
    /// A single string value.
    Single(String),
    /// Multiple string values.
    Multiple(Vec<String>),
}

impl StringOrVec {
    /// Creates a [`StringOrVec::Single`] value.
    pub fn single(value: impl Into<String>) -> Self {
        Self::Single(value.into())
    }

    /// Creates a [`StringOrVec::Multiple`] list of strings.
    pub fn multiple(values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Multiple(values.into_iter().map(Into::into).collect())
    }
}""",
        emit_enum("CommandStartMode", "Process launch mode for a custom add-on command (`addOnsCommands`).",
                  extract_enum_values(load_definition_props("addOnsCommands")["startMode"])),
        """/// Context-menu placement configuration for an [`AddOnCommand`].
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct CommandContextMenu {
    /// Slash-separated menu path inside the context menu.
    #[serde(default, rename = "basePath", skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// Object types or categories for which the menu item is visible.
    #[serde(default, rename = "visibleFor", skip_serializing_if = "Option::is_none")]
    pub visible_for: Option<StringOrVec>,
    /// Object types or categories for which the menu item is enabled.
    #[serde(default, rename = "enabledFor", skip_serializing_if = "Option::is_none")]
    pub enabled_for: Option<StringOrVec>,
}

impl CommandContextMenu {
    /// Creates a new [`CommandContextMenu`] with `base_path`.
    pub fn new(base_path: impl Into<String>) -> Self {
        Self {
            base_path: Some(base_path.into()),
            visible_for: None,
            enabled_for: None,
        }
    }

    /// Sets the object types for which this context menu item is visible.
    pub fn visible_for(mut self, visible_for: StringOrVec) -> Self {
        self.visible_for = Some(visible_for);
        self
    }

    /// Sets the object types for which this context menu item is enabled.
    pub fn enabled_for(mut self, enabled_for: StringOrVec) -> Self {
        self.enabled_for = Some(enabled_for);
        self
    }
}""",
        """/// Main-menu placement configuration for an [`AddOnCommand`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommandMainMenu {
    /// Slash-separated menu path inside the Wwise Authoring main menu bar.
    #[serde(rename = "basePath")]
    pub base_path: String,
}

impl CommandMainMenu {
    /// Creates a new [`CommandMainMenu`] with `base_path`.
    pub fn new(base_path: impl Into<String>) -> Self {
        Self { base_path: base_path.into() }
    }
}""",
        """/// A custom add-on command definition (`addOnsCommands`) for `ak.wwise.ui.commands.register`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AddOnCommand {
    /// Unique identifier for the command.
    pub id: String,
    /// Display name shown in menus and shortcuts.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Executable or script to invoke when the command is triggered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    /// Lua script file path to run inside the Wwise process when the command is executed (Wwise 2025+).
    #[serde(default, rename = "luaScript", skip_serializing_if = "Option::is_none")]
    pub lua_script: Option<String>,
    /// Additional Lua search paths used when running `lua_script` (Wwise 2025+).
    #[serde(default, rename = "luaPaths", skip_serializing_if = "Option::is_none")]
    pub lua_paths: Option<Vec<String>>,
    /// Return expressions for selected objects passed to `lua_script` in `wa_args.selected` (Wwise 2025+).
    #[serde(default, rename = "luaSelectedReturn", skip_serializing_if = "Option::is_none")]
    pub lua_selected_return: Option<Vec<String>>,
    /// Process launch mode for multi-selection commands.
    #[serde(default, rename = "startMode", skip_serializing_if = "Option::is_none")]
    pub start_mode: Option<CommandStartMode>,
    /// Command-line arguments template passed to `program`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    /// Working directory when launching `program`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    /// Default keyboard shortcut (e.g. `"Ctrl+Shift+K"`).
    #[serde(default, rename = "defaultShortcut", skip_serializing_if = "Option::is_none")]
    pub default_shortcut: Option<String>,
    /// Whether standard output and standard error of `program` are redirected and logged to Wwise on termination.
    #[serde(default, rename = "redirectOutputs", skip_serializing_if = "Option::is_none")]
    pub redirect_outputs: Option<bool>,
    /// Context menu placement configuration.
    #[serde(default, rename = "contextMenu", skip_serializing_if = "Option::is_none")]
    pub context_menu: Option<CommandContextMenu>,
    /// Main menu placement configuration.
    #[serde(default, rename = "mainMenu", skip_serializing_if = "Option::is_none")]
    pub main_menu: Option<CommandMainMenu>,
}

impl AddOnCommand {
    /// Creates a new [`AddOnCommand`] with `id` and `display_name`.
    pub fn new(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            program: None,
            lua_script: None,
            lua_paths: None,
            lua_selected_return: None,
            start_mode: None,
            args: None,
            cwd: None,
            default_shortcut: None,
            redirect_outputs: None,
            context_menu: None,
            main_menu: None,
        }
    }

    /// Sets the executable or script path.
    pub fn program(mut self, program: impl Into<String>) -> Self {
        self.program = Some(program.into());
        self
    }

    /// Sets the Lua script file path to run inside the Wwise process (Wwise 2025+).
    pub fn lua_script(mut self, lua_script: impl Into<String>) -> Self {
        self.lua_script = Some(lua_script.into());
        self
    }

    /// Sets the additional Lua search paths (Wwise 2025+).
    pub fn lua_paths(mut self, lua_paths: impl Into<Vec<String>>) -> Self {
        self.lua_paths = Some(lua_paths.into());
        self
    }

    /// Sets the return expressions for selected objects passed to `lua_script` (Wwise 2025+).
    pub fn lua_selected_return(mut self, lua_selected_return: impl Into<Vec<String>>) -> Self {
        self.lua_selected_return = Some(lua_selected_return.into());
        self
    }

    /// Sets the multi-selection process start mode.
    pub fn start_mode(mut self, start_mode: CommandStartMode) -> Self {
        self.start_mode = Some(start_mode);
        self
    }

    /// Sets the command-line arguments template.
    pub fn args(mut self, args: impl Into<String>) -> Self {
        self.args = Some(args.into());
        self
    }

    /// Sets the working directory for the command process.
    pub fn cwd(mut self, cwd: impl Into<String>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// Sets the default keyboard shortcut.
    pub fn default_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.default_shortcut = Some(shortcut.into());
        self
    }

    /// Sets whether standard output and error streams are redirected and logged to Wwise.
    pub fn redirect_outputs(mut self, redirect_outputs: bool) -> Self {
        self.redirect_outputs = Some(redirect_outputs);
        self
    }

    /// Sets the context menu configuration.
    pub fn context_menu(mut self, context_menu: CommandContextMenu) -> Self {
        self.context_menu = Some(context_menu);
        self
    }

    /// Sets the main menu configuration.
    pub fn main_menu(mut self, main_menu: CommandMainMenu) -> Self {
        self.main_menu = Some(main_menu);
        self
    }
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
    parts.append(gen_profiler_and_domain_types(schemas, endpoints_docs))
    return "\n\n".join(parts) + "\n"


HEADER = """// Shared primitives referenced by generated typed bindings.
// This file is generated by tools/generate_typed_api.py; do not edit by hand.
"""

STATIC_REFS = """/// Object reference accepted by soundengine endpoints: name, GUID, or Short ID.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

/// Event reference accepted by `soundengine.postEvent`.
pub type EventRef = SoundEngineObjectRef;

/// RTPC reference accepted by `soundengine.setRTPCValue`.
pub type RtpcRef = SoundEngineObjectRef;

/// Game object reference accepted by soundengine calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

/// Language reference used in return options.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub fn return_fields(mut self, fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields = Some(fields.into());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_(self, fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields(fields)
    }

    /// Specifies the platform to use when querying properties.
    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Specifies the language to use when querying language-specific properties.
    pub fn language(mut self, language: LanguageRef) -> Self {
        self.language = Some(language);
        self
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
        resp_doc = _compose_struct_doc(
            f"Response from `{ctx.uri}`.",
            schema_description(ctx, result_schema) or endpoint_desc,
        )
        if result_is_object_return:
            sections.append("\n".join([*doc_lines(resp_doc), f"pub type {name} = WwiseObject;"]))
        elif ctx.uri == "ak.wwise.core.object.set":
            item_name = f"{base}Object"
            exports.append(item_name)
            sections.append(gen_object_set_response(ctx, name, item_name))
        else:
            ctx.local_root = result_schema
            fields = collect_fields(ctx, result_schema, "response", name)
            ctx.local_root = None
            sections.append(emit_struct("response", name, resp_doc, fields, ctx))

    return "\n\n".join(sections) + "\n", exports, has_args, has_options, has_response


def gen_topic_file(ctx):
    """Generate the payload struct and subscription options for a WAAPI topic."""
    base = endpoint_base_name(ctx.uri)
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

    payload_name = f"{base}Payload"
    exports.append(payload_name)
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
    pub fn new(from: ObjectQuery) -> Self {{
        Self {{
            waql: None,
            from: Some(from),
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
    pub fn from(mut self, from: ObjectQuery) -> Self {{
        self.from = Some(from);
        self
    }}

    /// Sets the sequential transformations applied to the object list returned by `from`.
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

impl {args_name} {{
    /// Creates a new [`{args_name}`] containing a single [`{item_name}`].
    pub fn from_item(item: {item_name}) -> Self {{
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
