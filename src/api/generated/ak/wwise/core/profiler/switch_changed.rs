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
    pub return_fields: Option<Vec<ObjectReturnField>>,
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
    pub fn return_fields<I, T>(mut self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields = Some(return_fields.into_iter().map(Into::into).collect());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_<I, T>(self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields(return_fields)
    }

    /// The ID (GUID) of the platform.
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}

impl From<()> for CoreProfilerSwitchChangedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreProfilerSwitchChangedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreProfilerSwitchChangedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreProfilerSwitchChangedPayload`].
pub type SwitchChangedPayload = CoreProfilerSwitchChangedPayload;

/// Module-scoped alias for [`CoreProfilerSwitchChangedOptions`].
pub type SwitchChangedOptions = CoreProfilerSwitchChangedOptions;
