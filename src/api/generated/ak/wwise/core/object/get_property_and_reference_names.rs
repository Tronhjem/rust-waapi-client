// Generated bindings for ak.wwise.core.object.getPropertyAndReferenceNames

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.getPropertyAndReferenceNames`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectGetPropertyAndReferenceNamesArgs {
    /// The ID (class ID) of the object to retrieve the property and reference names from.
    #[serde(rename = "classId", skip_serializing_if = "Option::is_none")]
    pub class_id: Option<u32>,
    /// The ID (GUID), name, or path of the object to watch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectRef>,
}

impl CoreObjectGetPropertyAndReferenceNamesArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn class_id(mut self, class_id: impl Into<u32>) -> Self {
        self.class_id = Some(class_id.into());
        self
    }

    pub fn object(mut self, object: impl Into<ObjectRef>) -> Self {
        self.object = Some(object.into());
        self
    }
}

/// Response from `ak.wwise.core.object.getPropertyAndReferenceNames`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectGetPropertyAndReferenceNamesResponse {
    /// An array of all properties and references for the specified object.
    #[serde(rename = "return", default)]
    pub names: Vec<String>,
}
