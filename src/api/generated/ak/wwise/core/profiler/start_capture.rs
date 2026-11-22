// Generated bindings for ak.wwise.core.profiler.startCapture

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.profiler.startCapture`.
///
/// Starts the profiler capture and returns the time at the beginning of the capture, in milliseconds.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerStartCaptureResponse {
    /// The value of the Capture Time Cursor in ms.
    #[serde(rename = "return", default)]
    pub return_: Option<i64>,
}

/// Module-scoped alias for [`CoreProfilerStartCaptureResponse`].
pub type StartCaptureResponse = CoreProfilerStartCaptureResponse;
