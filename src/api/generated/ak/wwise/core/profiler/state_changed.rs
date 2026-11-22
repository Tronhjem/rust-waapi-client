// Generated bindings for topic ak.wwise.core.profiler.stateChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.profiler.stateChanged`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerStateChangedPayload {
    /// The state object that changed. Default return is id and name.
    pub state: WwiseObject,
    /// The state group object that changed. Default return is id and name.
    #[serde(rename = "stateGroup")]
    pub state_group: WwiseObject,
}

/// Subscription options for `ak.wwise.core.profiler.stateChanged`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreProfilerStateChangedOptions {
    /// Data to be returned for every object.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreProfilerStateChangedOptions {
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
