// Generated bindings for ak.wwise.core.profiler.getRTPCs

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getRTPCs`.
///
/// Retrieves active RTPCs at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetRtpCsArgs {
    /// Time in milliseconds to query for RTPCs, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
}

impl CoreProfilerGetRtpCsArgs {
    /// Creates a new [`CoreProfilerGetRtpCsArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self { time }
    }
}

impl From<serde_json::Value> for CoreProfilerGetRtpCsArgs {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.profiler.getRTPCs`.
///
/// Retrieves active RTPCs at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetRtpCsResponse {
    /// Array of RTPCs.
    #[serde(rename = "return")]
    pub return_: Vec<WwiseObjectRef>,
}

/// Module-scoped alias for [`CoreProfilerGetRtpCsArgs`].
pub type GetRtpCsArgs = CoreProfilerGetRtpCsArgs;

/// Module-scoped alias for [`CoreProfilerGetRtpCsResponse`].
pub type GetRtpCsResponse = CoreProfilerGetRtpCsResponse;
