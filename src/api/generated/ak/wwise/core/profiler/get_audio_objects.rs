// Generated bindings for ak.wwise.core.profiler.getAudioObjects

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getAudioObjects`.
///
/// Retrieves the Audio Objects at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetAudioObjectsArgs {
    /// Time in milliseconds to query for Audio Objects, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
    /// The pipeline ID of a Bus instance for which to get Audio Objects.
    #[serde(rename = "busPipelineID", skip_serializing_if = "Option::is_none")]
    pub bus_pipeline_id: Option<u32>,
}

impl CoreProfilerGetAudioObjectsArgs {
    /// Creates a new [`CoreProfilerGetAudioObjectsArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self {
            time,
            bus_pipeline_id: None,
        }
    }

    /// The pipeline ID of a Bus instance for which to get Audio Objects.
    pub fn bus_pipeline_id(mut self, bus_pipeline_id: u32) -> Self {
        self.bus_pipeline_id = Some(bus_pipeline_id);
        self
    }
}

/// Options for `ak.wwise.core.profiler.getAudioObjects`.
///
/// Retrieves the Audio Objects at a specific profiler capture time.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreProfilerGetAudioObjectsOptions {
    /// Members to return for each Audio Object. Defaults to Audio Object ID, Bus Pipeline ID, Instigator Pipeline ID and Effect Class ID.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<AudioObjectReturnField>>,
}

impl CoreProfilerGetAudioObjectsOptions {
    /// Creates a new [`CoreProfilerGetAudioObjectsOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Members to return for each Audio Object. Defaults to Audio Object ID, Bus Pipeline ID, Instigator Pipeline ID and Effect Class ID.
    pub fn return_fields(mut self, return_fields: impl Into<Vec<AudioObjectReturnField>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_(self, return_fields: impl Into<Vec<AudioObjectReturnField>>) -> Self {
        self.return_fields(return_fields)
    }
}

/// Response from `ak.wwise.core.profiler.getAudioObjects`.
///
/// Retrieves the Audio Objects at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetAudioObjectsResponse {
    /// Array of Audio Objects.
    #[serde(rename = "return")]
    pub return_: Vec<serde_json::Value>,
}
