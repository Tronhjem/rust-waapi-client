// Generated bindings for topic ak.wwise.core.profiler.stateChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.profiler.stateChanged`.
///
/// Sent when a state group state has been changed. This subscription does not require the profiler capture log to be started.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerStateChangedPayload {
    /// The state object that changed. Default return is id and name.
    pub state: WwiseObject,
    /// The state group object that changed. Default return is id and name.
    #[serde(rename = "stateGroup")]
    pub state_group: WwiseObject,
}

/// Subscription options for `ak.wwise.core.profiler.stateChanged`.
///
/// Sent when a state group state has been changed. This subscription does not require the profiler capture log to be started.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreProfilerStateChangedOptions {
    /// Data to be returned for every object.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreProfilerStateChangedOptions {
    /// Creates a new [`CoreProfilerStateChangedOptions`] with all fields unset.
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

impl From<()> for CoreProfilerStateChangedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreProfilerStateChangedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreProfilerStateChangedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreProfilerStateChangedPayload`].
pub type StateChangedPayload = CoreProfilerStateChangedPayload;

/// Module-scoped alias for [`CoreProfilerStateChangedOptions`].
pub type StateChangedOptions = CoreProfilerStateChangedOptions;
