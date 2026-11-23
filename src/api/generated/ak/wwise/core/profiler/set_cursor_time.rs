// Generated bindings for ak.wwise.core.profiler.setCursorTime

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.setCursorTime`.
///
/// Sets the user time cursor of the profiler to the specified time, in milliseconds, and returns the new time. Use the time obtained from the other profiler functions. Note that the specified time will be snapped to a buffer interval (~10ms). This sets the cursor position in the profiler views. To synchronize the user time cursor to the latest capture time, use `ak.wwise.ui.commands.execute` with `ToggleFollowCapture`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerSetCursorTimeArgs {
    /// The time to set the user cursor to, in milliseconds.
    pub time: i64,
}

impl CoreProfilerSetCursorTimeArgs {
    /// Creates a new [`CoreProfilerSetCursorTimeArgs`] with the required fields.
    pub fn new(time: i64) -> Self {
        Self { time }
    }
}

/// Response from `ak.wwise.core.profiler.setCursorTime`.
///
/// Sets the user time cursor of the profiler to the specified time, in milliseconds, and returns the new time. Use the time obtained from the other profiler functions. Note that the specified time will be snapped to a buffer interval (~10ms). This sets the cursor position in the profiler views. To synchronize the user time cursor to the latest capture time, use `ak.wwise.ui.commands.execute` with `ToggleFollowCapture`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerSetCursorTimeResponse {
    /// The new position of the user Time Cursor, in ms. This could differ from the specified time due to snapping.
    #[serde(rename = "return")]
    pub return_: i64,
}
