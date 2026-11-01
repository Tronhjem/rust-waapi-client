//! Type-safe wrapper for `ak.wwise.core.object.set`.

use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

use super::primitives::{
    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, PropertyValue,
};
use crate::waapi_client::{WaapiClient, WaapiError, WaapiResult};
use crate::waapi_function_api::ak;

// ─────────────────────────────────────────────────────────────────────────────
// Per-object specification (one entry in the top-level `objects` array)
// ─────────────────────────────────────────────────────────────────────────────

/// Describes what to set on a single existing Wwise object.
#[derive(Debug)]
pub struct ObjectSetItem {
    /// The target object to modify.
    pub object: ObjectRef,
    /// Rename the object.
    pub name: Option<String>,
    /// Set or replace the object's notes/comments.
    pub notes: Option<String>,
    /// Per-item platform override (takes precedence over the top-level default).
    pub platform: Option<PlatformRef>,
    /// Per-item name-conflict override.
    pub on_name_conflict: Option<OnNameConflict>,
    /// Per-item list-mode override.
    pub list_mode: Option<ListMode>,
    /// Child objects to create under this object.
    pub children: Vec<ChildObject>,
    /// `@PropertyName` fields to set. Keys are bare names without the `@` prefix.
    pub properties: Vec<(String, PropertyValue)>,
}

impl ObjectSetItem {
    pub fn new(object: ObjectRef) -> Self {
        ObjectSetItem {
            object,
            name: None,
            notes: None,
            platform: None,
            on_name_conflict: None,
            list_mode: None,
            children: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn rename(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn on_name_conflict(mut self, conflict: OnNameConflict) -> Self {
        self.on_name_conflict = Some(conflict);
        self
    }

    pub fn list_mode(mut self, mode: ListMode) -> Self {
        self.list_mode = Some(mode);
        self
    }

    /// Set a property, reference, or list field.
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

impl Serialize for ObjectSetItem {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let len = 1
            + usize::from(self.name.is_some())
            + usize::from(self.notes.is_some())
            + usize::from(self.platform.is_some())
            + usize::from(self.on_name_conflict.is_some())
            + usize::from(self.list_mode.is_some())
            + usize::from(!self.children.is_empty())
            + self.properties.len();
        let mut ser = serializer.serialize_map(Some(len))?;
        ser.serialize_entry("object", &self.object)?;
        if let Some(name) = &self.name {
            ser.serialize_entry("name", name)?;
        }
        if let Some(notes) = &self.notes {
            ser.serialize_entry("notes", notes)?;
        }
        if let Some(platform) = &self.platform {
            ser.serialize_entry("platform", platform)?;
        }
        if let Some(conflict) = &self.on_name_conflict {
            ser.serialize_entry("onNameConflict", conflict)?;
        }
        if let Some(mode) = &self.list_mode {
            ser.serialize_entry("listMode", mode)?;
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

// ─────────────────────────────────────────────────────────────────────────────
// Top-level args
// ─────────────────────────────────────────────────────────────────────────────

/// Arguments for `ak.wwise.core.object.set`.
#[derive(Debug)]
pub struct ObjectSetArgs {
    /// The objects to modify. At least one entry required.
    pub objects: Vec<ObjectSetItem>,
    /// Default platform for all items.
    pub platform: Option<PlatformRef>,
    /// Default name-conflict resolution for all items.
    pub on_name_conflict: Option<OnNameConflict>,
    /// Default list mode for all items.
    pub list_mode: Option<ListMode>,
    /// Whether Wwise auto-adds affected work units to source control.
    pub auto_add_to_source_control: Option<bool>,
}

impl ObjectSetArgs {
    pub fn new(objects: Vec<ObjectSetItem>) -> Self {
        ObjectSetArgs {
            objects,
            platform: None,
            on_name_conflict: None,
            list_mode: None,
            auto_add_to_source_control: None,
        }
    }

    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn on_name_conflict(mut self, conflict: OnNameConflict) -> Self {
        self.on_name_conflict = Some(conflict);
        self
    }

    pub fn list_mode(mut self, mode: ListMode) -> Self {
        self.list_mode = Some(mode);
        self
    }

    pub fn auto_add_to_source_control(mut self, value: bool) -> Self {
        self.auto_add_to_source_control = Some(value);
        self
    }
}

impl Serialize for ObjectSetArgs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let len = 1
            + usize::from(self.platform.is_some())
            + usize::from(self.on_name_conflict.is_some())
            + usize::from(self.list_mode.is_some())
            + usize::from(self.auto_add_to_source_control.is_some());
        let mut ser = serializer.serialize_map(Some(len))?;
        ser.serialize_entry("objects", &self.objects)?;
        if let Some(platform) = &self.platform {
            ser.serialize_entry("platform", platform)?;
        }
        if let Some(conflict) = &self.on_name_conflict {
            ser.serialize_entry("onNameConflict", conflict)?;
        }
        if let Some(mode) = &self.list_mode {
            ser.serialize_entry("listMode", mode)?;
        }
        if let Some(add) = &self.auto_add_to_source_control {
            ser.serialize_entry("autoAddToSourceControl", add)?;
        }
        ser.end()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Return options
// ─────────────────────────────────────────────────────────────────────────────

/// Controls which fields are returned for objects affected by the call.
#[derive(Debug, Default)]
pub struct ObjectReturnOptions {
    pub fields: Vec<String>,
    pub platform: Option<PlatformRef>,
    pub language: Option<String>,
}

impl ObjectReturnOptions {
    pub fn new(fields: Vec<String>) -> Self {
        ObjectReturnOptions {
            fields,
            platform: None,
            language: None,
        }
    }

    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

impl Serialize for ObjectReturnOptions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let len =
            1 + usize::from(self.platform.is_some()) + usize::from(self.language.is_some());
        let mut ser = serializer.serialize_map(Some(len))?;
        ser.serialize_entry("return", &self.fields)?;
        if let Some(platform) = &self.platform {
            ser.serialize_entry("platform", platform)?;
        }
        if let Some(language) = &self.language {
            ser.serialize_entry("language", language)?;
        }
        ser.end()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Response types
// ─────────────────────────────────────────────────────────────────────────────

/// An object entry in the response.
#[derive(Debug, Deserialize)]
pub struct ReturnedObject {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub children: Vec<ReturnedObject>,
}

/// Response from `ak.wwise.core.object.set`.
#[derive(Debug, Deserialize)]
pub struct ObjectSetResponse {
    #[serde(default)]
    pub objects: Vec<ReturnedObject>,
}

impl TryFrom<Value> for ObjectSetResponse {
    type Error = WaapiError;

    fn try_from(value: Value) -> Result<Self, WaapiError> {
        serde_json::from_value(value).map_err(WaapiError::from)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Typed method on WaapiClient
// ─────────────────────────────────────────────────────────────────────────────

impl WaapiClient {
    /// Set properties, rename, or create children on one or more existing
    /// Wwise objects in a single call.
    pub async fn object_set(
        &mut self,
        args: ObjectSetArgs,
        options: ObjectReturnOptions,
    ) -> WaapiResult<ObjectSetResponse> {
        let args_value = serde_json::to_value(args)?;
        let options_value = serde_json::to_value(options)?;
        let response = self
            .call(ak::wwise::core::object::set, Some(args_value), Some(options_value))
            .await?;
        ObjectSetResponse::try_from(response)
    }
}
