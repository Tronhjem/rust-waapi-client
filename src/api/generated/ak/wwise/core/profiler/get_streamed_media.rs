// Generated bindings for ak.wwise.core.profiler.getStreamedMedia

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getStreamedMedia`.
///
/// Retrieves the streaming media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Streams tab. To ensure the Streams data is received, see `ak.wwise.core.profiler.enableProfilerData`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetStreamedMediaArgs {
    /// Time in milliseconds to query for media, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
}

impl CoreProfilerGetStreamedMediaArgs {
    /// Creates a new [`CoreProfilerGetStreamedMediaArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self { time }
    }
}

impl From<serde_json::Value> for CoreProfilerGetStreamedMediaArgs {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.profiler.getStreamedMedia`.
///
/// Retrieves the streaming media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Streams tab. To ensure the Streams data is received, see `ak.wwise.core.profiler.enableProfilerData`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetStreamedMediaResponse {
    /// Array of Streams.
    #[serde(rename = "return")]
    pub return_: Vec<serde_json::Value>,
}

/// Module-scoped alias for [`CoreProfilerGetStreamedMediaArgs`].
pub type GetStreamedMediaArgs = CoreProfilerGetStreamedMediaArgs;

/// Module-scoped alias for [`CoreProfilerGetStreamedMediaResponse`].
pub type GetStreamedMediaResponse = CoreProfilerGetStreamedMediaResponse;
