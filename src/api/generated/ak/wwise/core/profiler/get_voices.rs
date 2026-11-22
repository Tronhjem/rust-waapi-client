// Generated bindings for ak.wwise.core.profiler.getVoices

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getVoices`.
///
/// Retrieves the voices at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetVoicesArgs {
    /// Time in milliseconds to query for voices, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
    /// The pipeline ID of a single voice to get.
    #[serde(rename = "voicePipelineID", skip_serializing_if = "Option::is_none")]
    pub voice_pipeline_id: Option<u32>,
}

impl CoreProfilerGetVoicesArgs {
    /// Creates a new [`CoreProfilerGetVoicesArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self {
            time,
            voice_pipeline_id: None,
        }
    }

    /// The pipeline ID of a single voice to get.
    pub fn voice_pipeline_id(mut self, voice_pipeline_id: impl Into<u32>) -> Self {
        self.voice_pipeline_id = Some(voice_pipeline_id.into());
        self
    }
}

/// Options for `ak.wwise.core.profiler.getVoices`.
///
/// Retrieves the voices at a specific profiler capture time.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreProfilerGetVoicesOptions {
    /// Members to return for each voice. Defaults to Pipeline ID, Game Object ID and Object GUID.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<VoiceReturnField>>,
}

impl CoreProfilerGetVoicesOptions {
    /// Creates a new [`CoreProfilerGetVoicesOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Members to return for each voice. Defaults to Pipeline ID, Game Object ID and Object GUID.
    pub fn return_(mut self, return_: impl Into<Vec<VoiceReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }
}

/// Response from `ak.wwise.core.profiler.getVoices`.
///
/// Retrieves the voices at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetVoicesResponse {
    /// Array of voices.
    #[serde(rename = "return")]
    pub return_: Vec<serde_json::Value>,
}
