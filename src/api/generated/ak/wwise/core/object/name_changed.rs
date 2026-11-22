// Generated bindings for topic ak.wwise.core.object.nameChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.nameChanged`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectNameChangedPayload {
    /// The object that was renamed.
    pub object: WwiseObject,
    /// The new name of the object.
    #[serde(rename = "newName")]
    pub new_name: String,
    /// The previous name of the object.
    #[serde(rename = "oldName")]
    pub old_name: String,
}

/// Subscription options for `ak.wwise.core.object.nameChanged`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectNameChangedOptions {
    /// Data to be returned for every name change.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectNameChangedOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_(mut self, return_: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
