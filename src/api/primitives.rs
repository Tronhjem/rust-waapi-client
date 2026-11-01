//! Shared primitive types used across multiple WAAPI endpoints.

use serde::{Deserialize, Serialize, Serializer};

// ─────────────────────────────────────────────────────────────────────────────
// Object and platform references
// ─────────────────────────────────────────────────────────────────────────────

/// Identifies an existing Wwise object.
#[derive(Debug, Clone)]
pub enum ObjectRef {
    /// A GUID, e.g. `"{FF59687C-48CF-4385-B1C5-CE84B0A63880}"`.
    Guid(String),
    /// A backslash-separated hierarchy path.
    Path(String),
    /// A unique qualified object name.
    Name(String),
}

impl ObjectRef {
    pub fn guid(s: impl Into<String>) -> Self {
        ObjectRef::Guid(s.into())
    }
    pub fn path(s: impl Into<String>) -> Self {
        ObjectRef::Path(s.into())
    }
    pub fn name(s: impl Into<String>) -> Self {
        ObjectRef::Name(s.into())
    }
}

impl Serialize for ObjectRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = match self {
            ObjectRef::Guid(s) | ObjectRef::Path(s) | ObjectRef::Name(s) => s,
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for ObjectRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.starts_with('{') && s.ends_with('}') {
            Ok(ObjectRef::Guid(s))
        } else if s.starts_with('\\') {
            Ok(ObjectRef::Path(s))
        } else {
            Ok(ObjectRef::Name(s))
        }
    }
}

/// Identifies a Wwise platform for platform-specific property overrides.
#[derive(Debug, Clone)]
pub enum PlatformRef {
    Guid(String),
    Name(String),
}

impl PlatformRef {
    pub fn guid(s: impl Into<String>) -> Self {
        PlatformRef::Guid(s.into())
    }
    pub fn name(s: impl Into<String>) -> Self {
        PlatformRef::Name(s.into())
    }
}

impl Serialize for PlatformRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = match self {
            PlatformRef::Guid(s) | PlatformRef::Name(s) => s,
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for PlatformRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.starts_with('{') && s.ends_with('}') {
            Ok(PlatformRef::Guid(s))
        } else {
            Ok(PlatformRef::Name(s))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared enums
// ─────────────────────────────────────────────────────────────────────────────

/// Action to take when a child with the same name already exists.
#[derive(Debug, Clone)]
pub enum OnNameConflict {
    Rename,
    Replace,
    Fail,
    Merge,
}

impl OnNameConflict {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            OnNameConflict::Rename => "rename",
            OnNameConflict::Replace => "replace",
            OnNameConflict::Fail => "fail",
            OnNameConflict::Merge => "merge",
        }
    }
}

impl Serialize for OnNameConflict {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OnNameConflict {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "rename" => Ok(OnNameConflict::Rename),
            "replace" => Ok(OnNameConflict::Replace),
            "fail" => Ok(OnNameConflict::Fail),
            "merge" => Ok(OnNameConflict::Merge),
            _ => Err(serde::de::Error::custom(format!(
                "unknown OnNameConflict: {s}"
            ))),
        }
    }
}

/// Action to take when an object already has items in a named list.
#[derive(Debug, Clone)]
pub enum ListMode {
    ReplaceAll,
    Append,
}

impl ListMode {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            ListMode::ReplaceAll => "replaceAll",
            ListMode::Append => "append",
        }
    }
}

impl Serialize for ListMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ListMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "replaceAll" => Ok(ListMode::ReplaceAll),
            "append" => Ok(ListMode::Append),
            _ => Err(serde::de::Error::custom(format!("unknown ListMode: {s}"))),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Property values and child object definitions
// ─────────────────────────────────────────────────────────────────────────────

/// A new Wwise object to create, nested under a parent via `children` or a
/// named list via an `@`-prefixed field.
#[derive(Debug, Clone)]
pub struct ChildObject {
    /// The Wwise object type, e.g. `"Sound"`, `"Event"`, `"RandomSequenceContainer"`.
    pub object_type: String,
    pub name: String,
    pub notes: Option<String>,
    pub platform: Option<PlatformRef>,
    /// Nested children to create under this object (recursive).
    pub children: Vec<ChildObject>,
    /// `@PropertyName` / `@ReferenceName` / `@ListName` fields.
    /// Keys are bare names without the `@` prefix — it is added during serialization.
    pub properties: Vec<(String, PropertyValue)>,
}

impl Serialize for ChildObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let len = 2
            + usize::from(self.notes.is_some())
            + usize::from(self.platform.is_some())
            + usize::from(!self.children.is_empty())
            + self.properties.len();
        let mut ser = serializer.serialize_map(Some(len))?;
        ser.serialize_entry("type", &self.object_type)?;
        ser.serialize_entry("name", &self.name)?;
        if let Some(notes) = &self.notes {
            ser.serialize_entry("notes", notes)?;
        }
        if let Some(platform) = &self.platform {
            ser.serialize_entry("platform", platform)?;
        }
        if !self.children.is_empty() {
            ser.serialize_entry("children", &self.children)?;
        }
        for (key, value) in &self.properties {
            ser.serialize_entry(&format!("@{key}"), value)?;
        }
        ser.end()
    }
}

impl ChildObject {
    pub fn new(object_type: impl Into<String>, name: impl Into<String>) -> Self {
        ChildObject {
            object_type: object_type.into(),
            name: name.into(),
            notes: None,
            platform: None,
            children: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Add a property, reference, or list field.
    /// Pass the bare name without the `@` prefix, e.g. `"Volume"` not `"@Volume"`.
    pub fn property(mut self, name: impl Into<String>, value: PropertyValue) -> Self {
        self.properties.push((name.into(), value));
        self
    }

    pub fn child(mut self, child: ChildObject) -> Self {
        self.children.push(child);
        self
    }
}

/// The value of a Wwise property or reference, set via an `@`-prefixed field.
#[derive(Debug, Clone)]
pub enum PropertyValue {
    /// A numeric property (WAAPI accepts both float and integer).
    Number(f64),
    /// A boolean property.
    Boolean(bool),
    /// A string value or object reference path/GUID/name.
    String(String),
    /// An array of string references, e.g. a list of GUIDs.
    StringList(Vec<String>),
    /// Create a new object and assign it to this reference slot.
    Object(Box<ChildObject>),
    /// Create new objects and insert them into this named list.
    ObjectList(Vec<ChildObject>),
}

impl Serialize for PropertyValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PropertyValue::Number(n) => serializer.serialize_f64(*n),
            PropertyValue::Boolean(b) => serializer.serialize_bool(*b),
            PropertyValue::String(s) => serializer.serialize_str(s),
            PropertyValue::StringList(v) => v.serialize(serializer),
            PropertyValue::Object(obj) => obj.serialize(serializer),
            PropertyValue::ObjectList(v) => v.serialize(serializer),
        }
    }
}
