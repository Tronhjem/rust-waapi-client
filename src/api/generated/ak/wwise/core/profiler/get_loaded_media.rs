// Generated bindings for ak.wwise.core.profiler.getLoadedMedia

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getLoadedMedia`.
///
/// Retrieves the loaded media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Loaded Media tab. To ensure the Loaded Media data is received, see `ak.wwise.core.profiler.enableProfilerData`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetLoadedMediaArgs {
    /// Time in milliseconds to query for media, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
}

impl CoreProfilerGetLoadedMediaArgs {
    /// Creates a new [`CoreProfilerGetLoadedMediaArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self { time }
    }
}

/// Response from `ak.wwise.core.profiler.getLoadedMedia`.
///
/// Retrieves the loaded media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Loaded Media tab. To ensure the Loaded Media data is received, see `ak.wwise.core.profiler.enableProfilerData`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetLoadedMediaResponse {
    /// Array of Loaded Media.
    #[serde(rename = "return")]
    pub return_: Vec<serde_json::Value>,
}
