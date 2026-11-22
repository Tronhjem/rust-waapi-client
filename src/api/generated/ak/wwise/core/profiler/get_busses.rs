// Generated bindings for ak.wwise.core.profiler.getBusses

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getBusses`.
///
/// Retrieves the busses at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetBussesArgs {
    /// Time in milliseconds to query for busses, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
    /// The pipeline ID of a single bus instance to get.
    #[serde(rename = "busPipelineID", skip_serializing_if = "Option::is_none")]
    pub bus_pipeline_id: Option<u32>,
}

impl CoreProfilerGetBussesArgs {
    /// Creates a new [`CoreProfilerGetBussesArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self {
            time,
            bus_pipeline_id: None,
        }
    }

    /// The pipeline ID of a single bus instance to get.
    pub fn bus_pipeline_id(mut self, bus_pipeline_id: impl Into<u32>) -> Self {
        self.bus_pipeline_id = Some(bus_pipeline_id.into());
        self
    }
}

impl From<serde_json::Value> for CoreProfilerGetBussesArgs {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

/// Options for `ak.wwise.core.profiler.getBusses`.
///
/// Retrieves the busses at a specific profiler capture time.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreProfilerGetBussesOptions {
    /// Members to return for each bus. Defaults to Pipeline ID, Game Object ID, and Object GUID.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<BusReturnField>>,
}

impl CoreProfilerGetBussesOptions {
    /// Creates a new [`CoreProfilerGetBussesOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Members to return for each bus. Defaults to Pipeline ID, Game Object ID, and Object GUID.
    pub fn return_(mut self, return_: impl Into<Vec<BusReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }
}

/// Response from `ak.wwise.core.profiler.getBusses`.
///
/// Retrieves the busses at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetBussesResponse {
    /// Array of busses.
    #[serde(rename = "return")]
    pub return_: Vec<serde_json::Value>,
}

/// Module-scoped alias for [`CoreProfilerGetBussesArgs`].
pub type GetBussesArgs = CoreProfilerGetBussesArgs;

/// Module-scoped alias for [`CoreProfilerGetBussesOptions`].
pub type GetBussesOptions = CoreProfilerGetBussesOptions;

/// Module-scoped alias for [`CoreProfilerGetBussesResponse`].
pub type GetBussesResponse = CoreProfilerGetBussesResponse;
