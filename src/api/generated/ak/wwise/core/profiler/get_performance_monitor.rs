// Generated bindings for ak.wwise.core.profiler.getPerformanceMonitor

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getPerformanceMonitor`.
///
/// Retrieves the Performance Monitor statistics at a specific profiler capture time. See the Wwise global counter IDs reference for the available counters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetPerformanceMonitorArgs {
    /// Time in milliseconds to query for Performance Monitor data, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: CursorTime,
}

impl CoreProfilerGetPerformanceMonitorArgs {
    /// Creates a new [`CoreProfilerGetPerformanceMonitorArgs`] with the required fields.
    pub fn new(time: CursorTime) -> Self {
        Self { time }
    }
}

/// Response from `ak.wwise.core.profiler.getPerformanceMonitor`.
///
/// Retrieves the Performance Monitor statistics at a specific profiler capture time. See the Wwise global counter IDs reference for the available counters.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetPerformanceMonitorResponse {
    /// Array of Performance Monitor counters.
    #[serde(rename = "return", default)]
    pub counters: Vec<ProfilerPerformanceCounter>,
}
