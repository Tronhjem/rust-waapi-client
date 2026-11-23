// Generated bindings for ak.wwise.core.profiler.stopCapture

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.profiler.stopCapture`.
///
/// Stops the profiler capture and returns the time at the end of the capture, in milliseconds.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerStopCaptureResponse {
    /// The value of the Capture Time Cursor in ms.
    #[serde(rename = "return", default)]
    pub return_: Option<i64>,
}
