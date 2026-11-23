// Generated bindings for topic ak.wwise.core.object.childAdded

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.childAdded`.
///
/// Sent when an object is added as a child to another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectChildAddedPayload {
    /// The parent object to which the object is added.
    pub parent: WwiseObject,
    /// The newly added child object.
    pub child: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.childAdded`.
///
/// Sent when an object is added as a child to another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectChildAddedOptions {
    /// Data to be returned for every child being added.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectChildAddedOptions {
    /// Creates a new [`CoreObjectChildAddedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every child being added.
    pub fn return_fields(mut self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_(self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields(return_fields)
    }

    /// The ID (GUID) of the platform.
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
