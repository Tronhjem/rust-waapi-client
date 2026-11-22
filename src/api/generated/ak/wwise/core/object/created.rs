// Generated bindings for topic ak.wwise.core.object.created

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.created`.
///
/// Sent when an object is created. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectCreatedPayload {
    /// The newly created object.
    pub object: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.created`.
///
/// Sent when an object is created. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectCreatedOptions {
    /// Data to be returned for every object being created.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectCreatedOptions {
    /// Creates a new [`CoreObjectCreatedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every object being created.
    pub fn return_(mut self, return_: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }

    /// The ID (GUID) of the platform.
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}

/// Module-scoped alias for [`CoreObjectCreatedPayload`].
pub type CreatedPayload = CoreObjectCreatedPayload;

/// Module-scoped alias for [`CoreObjectCreatedOptions`].
pub type CreatedOptions = CoreObjectCreatedOptions;
