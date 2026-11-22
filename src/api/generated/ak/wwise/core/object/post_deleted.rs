// Generated bindings for topic ak.wwise.core.object.postDeleted



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Payload for `ak.wwise.core.object.postDeleted`.
///
/// Sent following an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectPostDeletedPayload {
    /// The deleted object.
    pub object: WwiseObject,
}


/// Subscription options for `ak.wwise.core.object.postDeleted`.
///
/// Sent following an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectPostDeletedOptions {
    /// Data to be returned for every object following its deletion.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectPostDeletedOptions {
    /// Creates a new [`CoreObjectPostDeletedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every object following its deletion.
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
