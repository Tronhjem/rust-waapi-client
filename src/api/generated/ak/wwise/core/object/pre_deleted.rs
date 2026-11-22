// Generated bindings for topic ak.wwise.core.object.preDeleted

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.preDeleted`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectPreDeletedPayload {
    /// The object to be deleted.
    pub object: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.preDeleted`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectPreDeletedOptions {
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Data to be returned for every object prior to its deletion.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
}

impl CoreObjectPreDeletedOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn return_(mut self, return_: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }
}
