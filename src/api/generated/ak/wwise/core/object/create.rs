// Generated bindings for ak.wwise.core.object.create

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.create`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreObjectCreateArgs {
    /// For internal use only! The ID (GUID) to assign to the newly created Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the new object
    /// Refer to for possible object types.
    #[serde(rename = "type")]
    pub object_type: String,
    /// The name of the new object.
    pub name: String,
    /// The notes or comments of the new object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The ID (GUID), name, or path of the parent of the new object.
    pub parent: ObjectRef,
    /// The name of the list in which to insert the object
    /// If this argument is set, the object gets inserted in a list owned by the parent and not as a child.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<WwiseObjectRef>>,
    /// The ID (GUID) or unique name of the platform used when setting properties via this command
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
    /// The action to take if "parent" already has a child with the same name
    /// Default value is "fail"
    /// Refer to for more information.
    #[serde(rename = "onNameConflict", skip_serializing_if = "Option::is_none")]
    pub on_name_conflict: Option<OnNameConflict>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl WwiseCoreObjectCreateArgs {
    pub fn new(parent: ObjectRef, object_type: String, name: String) -> Self {
        Self {
            parent,
            object_type,
            name,
            id: None,
            notes: None,
            list: None,
            children: None,
            platform: None,
            auto_add_to_source_control: None,
            on_name_conflict: None,
            extra: std::collections::HashMap::new(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn list(mut self, list: impl Into<String>) -> Self {
        self.list = Some(list.into());
        self
    }

    pub fn children(mut self, children: impl Into<Vec<WwiseObjectRef>>) -> Self {
        self.children = Some(children.into());
        self
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

    /// Set an arbitrary `@`-prefixed property, reference, or list field.
    /// The key must include the `@` prefix, e.g. `"@Volume"`.
    pub fn extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

/// Response from `ak.wwise.core.object.create`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreObjectCreateResponse {
    /// The ID (GUID) of the newly created Object.
    #[serde(default)]
    pub id: Option<String>,
    /// The name of the newly created Object.
    #[serde(default)]
    pub name: Option<String>,
    /// The children of objects created.
    #[serde(default)]
    pub children: Vec<serde_json::Value>,
}
