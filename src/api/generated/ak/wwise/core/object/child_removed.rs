// Generated bindings for topic ak.wwise.core.object.childRemoved

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.childRemoved`.
///
/// Sent when an object is removed from the children of another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectChildRemovedPayload {
    /// The parent object from which the child object is removed.
    pub parent: WwiseObject,
    /// The child object that is removed.
    pub child: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.childRemoved`.
///
/// Sent when an object is removed from the children of another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectChildRemovedOptions {
    /// Data to be returned for every child being removed.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectChildRemovedOptions {
    /// Creates a new [`CoreObjectChildRemovedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every child being removed.
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
