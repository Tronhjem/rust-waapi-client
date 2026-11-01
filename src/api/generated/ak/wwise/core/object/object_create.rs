// Generated bindings for ak.wwise.core.object.create

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.wwise.core.object.create`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreObjectCreateArgs {
    /// The parent object.
    pub parent: ObjectRef,
    /// The Wwise object type to create.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The name of the new object.
    pub name: String,
    /// Action to take on a name conflict.
    #[serde(rename = "onNameConflict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_name_conflict: Option<OnNameConflict>,
    /// Platform for the new object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Notes for the new object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Child objects to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChildObject>>,
    /// How to modify named lists.
    #[serde(rename = "listMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_mode: Option<ListMode>,
}

impl WwiseCoreObjectCreateArgs {
    pub fn new(parent: ObjectRef, r#type: String, name: String) -> Self {
        Self {
            parent,
            r#type,
            name,
            on_name_conflict: None,
            platform: None,
            notes: None,
            children: None,
            list_mode: None,
        }
    }

    pub fn on_name_conflict(mut self, on_name_conflict: impl Into<OnNameConflict>) -> Self {
        self.on_name_conflict = Some(on_name_conflict.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
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

    pub fn list_mode(mut self, list_mode: impl Into<ListMode>) -> Self {
        self.list_mode = Some(list_mode.into());
        self
    }
}

/// Options for `ak.wwise.core.object.create`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreObjectCreateOptions {
    /// Fields to return.
    #[serde(rename = "return")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#return: Option<Vec<String>>,
    /// Platform for return values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Language for return values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageRef>,
}

impl WwiseCoreObjectCreateOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn r#return(mut self, r#return: impl Into<Vec<String>>) -> Self {
        self.r#return = Some(r#return.into());
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
}

/// Response from `ak.wwise.core.object.create`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub struct WwiseCoreObjectCreateResponse {
    /// Objects that were created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<WwiseObject>>,
}
