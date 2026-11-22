// Generated bindings for ak.wwise.core.profiler.getVoiceContributions

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getVoiceContributions`.
///
/// Retrieves all parameters affecting voice volume, highpass and lowpass for a voice path, resolved from pipeline IDs.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetVoiceContributionsArgs {
    /// Time in milliseconds to query for voices, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
    /// The pipeline ID of the voice to get contributions from.
    #[serde(rename = "voicePipelineID")]
    pub voice_pipeline_id: f64,
    /// An array of bus pipeline IDs.
    #[serde(rename = "bussesPipelineID", skip_serializing_if = "Option::is_none")]
    pub busses_pipeline_id: Option<Vec<f64>>,
}

impl CoreProfilerGetVoiceContributionsArgs {
    /// Creates a new [`CoreProfilerGetVoiceContributionsArgs`] with the required fields.
    pub fn new(voice_pipeline_id: f64, time: serde_json::Value) -> Self {
        Self {
            voice_pipeline_id,
            time,
            busses_pipeline_id: None,
        }
    }

    /// An array of bus pipeline IDs.
    pub fn busses_pipeline_id(mut self, busses_pipeline_id: impl Into<Vec<f64>>) -> Self {
        self.busses_pipeline_id = Some(busses_pipeline_id.into());
        self
    }
}

impl<T0: Into<f64>, T1: Into<serde_json::Value>> From<(T0, T1)>
    for CoreProfilerGetVoiceContributionsArgs
{
    fn from((voice_pipeline_id, time): (T0, T1)) -> Self {
        Self::new(voice_pipeline_id.into(), time.into())
    }
}

/// Response from `ak.wwise.core.profiler.getVoiceContributions`.
///
/// Retrieves all parameters affecting voice volume, highpass and lowpass for a voice path, resolved from pipeline IDs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetVoiceContributionsResponse {
    /// The hierarchy of objects with parameters contributing to the voice, ordered from top-level parent to the voice object.
    #[serde(rename = "return", default)]
    pub return_: Option<serde_json::Value>,
}

/// Module-scoped alias for [`CoreProfilerGetVoiceContributionsArgs`].
pub type GetVoiceContributionsArgs = CoreProfilerGetVoiceContributionsArgs;

/// Module-scoped alias for [`CoreProfilerGetVoiceContributionsResponse`].
pub type GetVoiceContributionsResponse = CoreProfilerGetVoiceContributionsResponse;
