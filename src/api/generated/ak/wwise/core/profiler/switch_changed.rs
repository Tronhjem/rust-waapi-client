// Generated bindings for topic ak.wwise.core.profiler.switchChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.profiler.switchChanged`.
///
/// Sent when a switch group state has been changed. This function does not require the profiler capture log to be started.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerSwitchChangedPayload {
    /// Game Object ID associated with the switch.
    #[serde(rename = "gameObjectID")]
    pub game_object_id: u64,
    /// The switch group object that changed. Default return is id and name.
    #[serde(rename = "switchGroup")]
    pub switch_group: WwiseObject,
    /// The new switch used. Default return is id and name.
    pub switch: WwiseObject,
}

/// Subscription options for `ak.wwise.core.profiler.switchChanged`.
///
/// Sent when a switch group state has been changed. This function does not require the profiler capture log to be started.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreProfilerSwitchChangedOptions {
    /// Data to be returned for every object.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreProfilerSwitchChangedOptions {
    /// Creates a new [`CoreProfilerSwitchChangedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every object.
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
