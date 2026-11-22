// Generated bindings for ak.wwise.core.blendContainer.addTrack

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.blendContainer.addTrack`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreBlendContainerAddTrackArgs {
    /// For internal use only! The ID (GUID) to assign to the newly created Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Blend Track.
    pub name: String,
    /// The ID (GUID), name, or path of the Blend Container.
    pub object: ObjectRef,
}

impl CoreBlendContainerAddTrackArgs {
    pub fn new(object: ObjectRef, name: String) -> Self {
        Self {
            object,
            name,
            id: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// Response from `ak.wwise.core.blendContainer.addTrack`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreBlendContainerAddTrackResponse {
    /// The ID (GUID) of the Blend Track.
    #[serde(default)]
    pub id: Option<String>,
    /// The name of the Blend Track.
    #[serde(default)]
    pub name: Option<String>,
    /// The type of the object. See for more information on the types available.
    #[serde(rename = "type", default)]
    pub object_type: Option<ObjectType>,
    /// The Short ID of the object.
    #[serde(rename = "shortId", default)]
    pub short_id: Option<i64>,
}
