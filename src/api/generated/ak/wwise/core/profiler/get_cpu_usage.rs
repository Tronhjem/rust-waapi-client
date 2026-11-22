// Generated bindings for ak.wwise.core.profiler.getCpuUsage

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getCpuUsage`.
///
/// Retrieves CPU usage statistics at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the CPU tab. To ensure the CPU data is received, see `ak.wwise.core.profiler.enableProfilerData`. The returned data includes "Inclusive" and "Exclusive" values, where "Inclusive" refers to the time spent in the element plus the time spent in any called elements, and "Exclusive" values pertain to execution only within the element itself.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetCpuUsageArgs {
    /// Time in milliseconds to query for cpu data, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
}

impl CoreProfilerGetCpuUsageArgs {
    /// Creates a new [`CoreProfilerGetCpuUsageArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self { time }
    }
}

impl From<serde_json::Value> for CoreProfilerGetCpuUsageArgs {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.profiler.getCpuUsage`.
///
/// Retrieves CPU usage statistics at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the CPU tab. To ensure the CPU data is received, see `ak.wwise.core.profiler.enableProfilerData`. The returned data includes "Inclusive" and "Exclusive" values, where "Inclusive" refers to the time spent in the element plus the time spent in any called elements, and "Exclusive" values pertain to execution only within the element itself.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetCpuUsageResponse {
    /// Array of CPU statistics for each element.
    #[serde(rename = "return")]
    pub return_: Vec<serde_json::Value>,
}

/// Module-scoped alias for [`CoreProfilerGetCpuUsageArgs`].
pub type GetCpuUsageArgs = CoreProfilerGetCpuUsageArgs;

/// Module-scoped alias for [`CoreProfilerGetCpuUsageResponse`].
pub type GetCpuUsageResponse = CoreProfilerGetCpuUsageResponse;
