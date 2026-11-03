// Generated bindings for ak.wwise.core.object.set

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// One object specification in the `objects` array of `object.set`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreObjectSetItem {
    /// The new name of "object".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The new notes or comments of "object".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The ID (GUID), name, or path of the existing object on which to set the name, notes, properties or references and under which to create children or create objects in a list.
    pub object: ObjectRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChildObject>>,
    /// The ID (GUID) or unique name of the platform used when setting properties and references via this operation
    /// Not specifying a platform sets the value for all linked platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// The action to take if "object" already has a child with the same name
    /// Default value is "fail"
    /// Refer to for more information.
    #[serde(rename = "onNameConflict", skip_serializing_if = "Option::is_none")]
    pub on_name_conflict: Option<OnNameConflict>,
    /// The action to take if "object" already has objects in a specified list
    /// Default value is "append"
    /// Refer to for more information.
    #[serde(rename = "listMode", skip_serializing_if = "Option::is_none")]
    pub list_mode: Option<ListMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl WwiseCoreObjectSetItem {
    pub fn new(object: ObjectRef) -> Self {
        Self {
            object,
            name: None,
            notes: None,
            children: None,
            platform: None,
            on_name_conflict: None,
            list_mode: None,
            import: None,
            extra: std::collections::HashMap::new(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn children(mut self, children: impl Into<Vec<ChildObject>>) -> Self {
        self.children = Some(children.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn on_name_conflict(mut self, on_name_conflict: impl Into<OnNameConflict>) -> Self {
        self.on_name_conflict = Some(on_name_conflict.into());
        self
    }

    pub fn list_mode(mut self, list_mode: impl Into<ListMode>) -> Self {
        self.list_mode = Some(list_mode.into());
        self
    }

    pub fn import(mut self, import: impl Into<serde_json::Value>) -> Self {
        self.import = Some(import.into());
        self
    }

    /// Set an arbitrary `@`-prefixed property, reference, or list field.
    /// The key must include the `@` prefix, e.g. `"@Volume"`.
    pub fn extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

impl WwiseCoreObjectSetItem {
    /// Set a `@`-prefixed property, reference, or list field.
    /// Pass the bare name without the `@` prefix, e.g. `"Volume"`.
    pub fn property(mut self, name: impl Into<String>, value: impl serde::Serialize) -> Self {
        self.extra.insert(
            format!("@{}", name.into()),
            serde_json::to_value(value).expect("value must serialize to JSON"),
        );
        self
    }
}

/// Arguments for `ak.wwise.core.object.set`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreObjectSetArgs {
    /// Unless overriden by an individual "object", the ID (GUID) or unique name of the platform used when setting properties and references via this operation
    /// Not specifying a platform sets the value for all linked platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Determines if Wwise automatically performs an Add source control operation for affected work units
    /// Defaults to true.
    #[serde(
        rename = "autoAddToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_add_to_source_control: Option<bool>,
    /// Unless overriden by an individual "object", the action to take if "object" already has a child with the same name
    /// Default value is "fail"
    /// Refer to for more information.
    #[serde(rename = "onNameConflict", skip_serializing_if = "Option::is_none")]
    pub on_name_conflict: Option<OnNameConflict>,
    /// Unless overriden by an individual "object", the action to take if "object" already has objects in a specified list
    /// Default value is "append"
    /// Refer to for more information.
    #[serde(rename = "listMode", skip_serializing_if = "Option::is_none")]
    pub list_mode: Option<ListMode>,
    /// An array of objects on which to set values and create child or list objects.
    pub objects: Vec<WwiseCoreObjectSetItem>,
}

impl WwiseCoreObjectSetArgs {
    pub fn new(objects: Vec<WwiseCoreObjectSetItem>) -> Self {
        Self {
            objects,
            platform: None,
            auto_add_to_source_control: None,
            on_name_conflict: None,
            list_mode: None,
        }
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn auto_add_to_source_control(
        mut self,
        auto_add_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_add_to_source_control = Some(auto_add_to_source_control.into());
        self
    }

    pub fn on_name_conflict(mut self, on_name_conflict: impl Into<OnNameConflict>) -> Self {
        self.on_name_conflict = Some(on_name_conflict.into());
        self
    }

    pub fn list_mode(mut self, list_mode: impl Into<ListMode>) -> Self {
        self.list_mode = Some(list_mode.into());
        self
    }
}

/// Options for `ak.wwise.core.object.set`.

pub type WwiseCoreObjectSetOptions = ObjectReturnOptions;

/// One object entry in the `object.set` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreObjectSetObject {
    /// The ID (GUID) of the parent.
    #[serde(default)]
    pub id: Option<String>,
    /// The name of the parent.
    #[serde(default)]
    pub name: Option<String>,
    /// The child objects created.
    #[serde(default)]
    pub children: Vec<WwiseObject>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Response from `ak.wwise.core.object.set`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreObjectSetResponse {
    /// Array of {object, created objects} associations for each parent object.
    #[serde(default)]
    pub objects: Vec<WwiseCoreObjectSetObject>,
}
