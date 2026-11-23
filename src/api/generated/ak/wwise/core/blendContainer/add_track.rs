// Generated bindings for ak.wwise.core.blendContainer.addTrack

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.blendContainer.addTrack`.
///
/// Adds a new Blend Track to a Blend Container. Equivalent to clicking New Blend Track in the Blend Track Editor. To get the list of Blend Tracks, use the function ak.wwise.core.object.get with {return = "blendTracks"}.
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
    /// Creates a new [`CoreBlendContainerAddTrackArgs`] with the required fields.
    pub fn new(object: ObjectRef, name: impl Into<String>) -> Self {
        Self {
            object,
            name: name.into(),
            id: None,
        }
    }

    /// For internal use only! The ID (GUID) to assign to the newly created Object.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// Response from `ak.wwise.core.blendContainer.addTrack`.
///
/// Adds a new Blend Track to a Blend Container. Equivalent to clicking New Blend Track in the Blend Track Editor. To get the list of Blend Tracks, use the function ak.wwise.core.object.get with {return = "blendTracks"}.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreBlendContainerAddTrackResponse {
    /// The ID (GUID) of the Blend Track.
    #[serde(default)]
    pub id: Option<String>,
    /// The name of the Blend Track.
    #[serde(default)]
    pub name: Option<String>,
    /// The type of the object. See the Wwise Objects Reference for more information on the types available.
    #[serde(rename = "type", default)]
    pub object_type: Option<ObjectType>,
    /// The Short ID of the object.
    #[serde(rename = "shortId", default)]
    pub short_id: Option<i64>,
}

impl CoreBlendContainerAddTrackResponse {
    /// Returns the object GUID string (`id`), if present in the response.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Returns the object name (`name`), if present in the response.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
