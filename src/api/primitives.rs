//! Shared primitive types used across multiple WAAPI endpoints.
//!
//! These map to the shared definitions in `waapi_definitions.json` and
//! `common_definitions.json` that are referenced by many endpoint schemas.

use std::collections::HashMap;

use crate::types::WaapiValue;

// ─────────────────────────────────────────────────────────────────────────────
// Object and platform references
// ─────────────────────────────────────────────────────────────────────────────

/// Identifies an existing Wwise object.
///
/// Maps to `waapi_definitions.json#/definitions/objectArg`.
/// All three variants serialize to a plain string — WAAPI disambiguates by
/// content.
#[derive(Debug, Clone)]
pub enum ObjectRef {
    /// A GUID, e.g. `"{FF59687C-48CF-4385-B1C5-CE84B0A63880}"`.
    Guid(String),
    /// A backslash-separated hierarchy path, e.g.
    /// `"\Actor-Mixer Hierarchy\Default Work Unit\MySound"`.
    Path(String),
    /// A unique qualified object name.
    Name(String),
}

impl ObjectRef {
    pub fn guid(s: impl Into<String>) -> Self { ObjectRef::Guid(s.into()) }
    pub fn path(s: impl Into<String>) -> Self { ObjectRef::Path(s.into()) }
    pub fn name(s: impl Into<String>) -> Self { ObjectRef::Name(s.into()) }

    pub(crate) fn to_waapi_value(&self) -> WaapiValue {
        let s = match self {
            ObjectRef::Guid(s) | ObjectRef::Path(s) | ObjectRef::Name(s) => s,
        };
        WaapiValue::String(s.clone())
    }
}

/// Identifies a Wwise platform for platform-specific property overrides.
///
/// Maps to `waapi_definitions.json#/definitions/platformArg`.
#[derive(Debug, Clone)]
pub enum PlatformRef {
    Guid(String),
    Name(String),
}

impl PlatformRef {
    pub fn guid(s: impl Into<String>) -> Self { PlatformRef::Guid(s.into()) }
    pub fn name(s: impl Into<String>) -> Self { PlatformRef::Name(s.into()) }

    pub(crate) fn to_waapi_value(&self) -> WaapiValue {
        let s = match self {
            PlatformRef::Guid(s) | PlatformRef::Name(s) => s,
        };
        WaapiValue::String(s.clone())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared enums (appear in multiple endpoint schemas)
// ─────────────────────────────────────────────────────────────────────────────

/// Action to take when a child with the same name already exists.
///
/// Used by `object.set`, `object.create`, and `audio.import`.
/// Maps to the `onNameConflict` enum in those schemas.
#[derive(Debug, Clone)]
pub enum OnNameConflict {
    /// Assign a new unique name to the incoming object.
    Rename,
    /// Delete the existing object and replace it with the incoming one.
    Replace,
    /// Return an error without making any change. (WAAPI default)
    Fail,
    /// Merge the incoming object into the existing one.
    Merge,
}

impl OnNameConflict {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            OnNameConflict::Rename  => "rename",
            OnNameConflict::Replace => "replace",
            OnNameConflict::Fail    => "fail",
            OnNameConflict::Merge   => "merge",
        }
    }
}

/// Action to take when an object already has items in a named list.
///
/// Used by `object.set` and `object.create`.
/// Maps to the `listMode` enum in those schemas.
#[derive(Debug, Clone)]
pub enum ListMode {
    /// Remove all existing items in the list and insert the new ones.
    ReplaceAll,
    /// Append the new items after any existing ones. (WAAPI default)
    Append,
}

impl ListMode {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            ListMode::ReplaceAll => "replaceAll",
            ListMode::Append     => "append",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Property values and child object definitions
// ─────────────────────────────────────────────────────────────────────────────

/// A new Wwise object to create, nested under a parent via `children` or a
/// named list via an `@`-prefixed field.
///
/// Used by `object.set` and `object.create`.
/// Maps to `argsSchema.definitions.object` in those schemas.
/// `object_type` and `name` are required; everything else is optional.
#[derive(Debug, Clone)]
pub struct ChildObject {
    /// The Wwise object type, e.g. `"Sound"`, `"Event"`, `"RandomSequenceContainer"`.
    pub object_type: String,
    pub name:        String,
    pub notes:       Option<String>,
    pub platform:    Option<PlatformRef>,
    /// Nested children to create under this object (recursive).
    pub children:    Vec<ChildObject>,
    /// `@PropertyName` / `@ReferenceName` / `@ListName` fields.
    /// Keys are bare names without the `@` prefix — it is added during serialization.
    pub properties:  Vec<(String, PropertyValue)>,
}

impl ChildObject {
    pub fn new(object_type: impl Into<String>, name: impl Into<String>) -> Self {
        ChildObject {
            object_type: object_type.into(),
            name:        name.into(),
            notes:       None,
            platform:    None,
            children:    Vec::new(),
            properties:  Vec::new(),
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

    pub(crate) fn to_waapi_value(&self) -> WaapiValue {
        WaapiValue::Dict(self.to_map())
    }

    pub(crate) fn to_map(&self) -> HashMap<String, WaapiValue> {
        let mut map = HashMap::new();

        map.insert("type".into(), WaapiValue::String(self.object_type.clone()));
        map.insert("name".into(), WaapiValue::String(self.name.clone()));

        if let Some(notes) = &self.notes {
            map.insert("notes".into(), WaapiValue::String(notes.clone()));
        }
        if let Some(platform) = &self.platform {
            map.insert("platform".into(), platform.to_waapi_value());
        }
        if !self.children.is_empty() {
            map.insert("children".into(), WaapiValue::List(
                self.children.iter().map(|c| c.to_waapi_value()).collect(),
            ));
        }
        for (key, value) in &self.properties {
            map.insert(format!("@{key}"), value.to_waapi_value());
        }

        map
    }
}

/// The value of a Wwise property or reference, set via an `@`-prefixed field.
///
/// Maps to the `anyOf` inside `patternProperties "^@[:_a-zA-Z0-9]+$"` in
/// `object.set` and `object.create`.
///
/// Which variant is valid depends on the specific property:
/// - `@Volume` → `Number`
/// - `@OutputBus` → `String` (GUID, path, or name of the target object)
/// - `@RTPC` → `ObjectList`
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

impl PropertyValue {
    pub(crate) fn to_waapi_value(&self) -> WaapiValue {
        match self {
            PropertyValue::Number(n)     => WaapiValue::Float(*n),
            PropertyValue::Boolean(b)    => WaapiValue::Boolean(*b),
            PropertyValue::String(s)     => WaapiValue::String(s.clone()),
            PropertyValue::StringList(v) => WaapiValue::List(
                v.iter().map(|s| WaapiValue::String(s.clone())).collect(),
            ),
            PropertyValue::Object(obj)   => obj.to_waapi_value(),
            PropertyValue::ObjectList(v) => WaapiValue::List(
                v.iter().map(|o| o.to_waapi_value()).collect(),
            ),
        }
    }
}
