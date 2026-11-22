// Generated bindings for ak.wwise.core.profiler.setCursorTime

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.setCursorTime`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerSetCursorTimeArgs {
    /// The time to set the user cursor to, in milliseconds.
    pub time: i64,
}

impl CoreProfilerSetCursorTimeArgs {
    pub fn new(time: i64) -> Self {
        Self { time }
    }
}

/// Response from `ak.wwise.core.profiler.setCursorTime`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerSetCursorTimeResponse {
    /// The new position of the user Time Cursor, in ms. This could differ from the specified time due to snapping.
    #[serde(rename = "return")]
    pub return_: i64,
}
