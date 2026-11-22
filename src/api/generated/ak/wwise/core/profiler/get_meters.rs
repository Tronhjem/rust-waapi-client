// Generated bindings for ak.wwise.core.profiler.getMeters

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getMeters`.
///
/// Retrieves the Meter data for all registered busses, aux busses and devices. Only the Main Audio Bus is registered by default. Use `ak.wwise.core.profiler.registerMeter` for other busses, before retrieval of the meter data.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetMetersArgs {
    /// Time in milliseconds to query for Performance Monitor data, or a Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub time: serde_json::Value,
}

impl CoreProfilerGetMetersArgs {
    /// Creates a new [`CoreProfilerGetMetersArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self { time }
    }
}

impl From<serde_json::Value> for CoreProfilerGetMetersArgs {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

/// Options for `ak.wwise.core.profiler.getMeters`.
///
/// Specifies the return options for the meter object field. The defaults are name and ID.
pub type CoreProfilerGetMetersOptions = ObjectReturnOptions;

/// Response from `ak.wwise.core.profiler.getMeters`.
///
/// Retrieves the Meter data for all registered busses, aux busses and devices. Only the Main Audio Bus is registered by default. Use `ak.wwise.core.profiler.registerMeter` for other busses, before retrieval of the meter data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetMetersResponse {
    /// Array of registered meters.
    pub meters: Vec<serde_json::Value>,
}

/// Module-scoped alias for [`CoreProfilerGetMetersArgs`].
pub type GetMetersArgs = CoreProfilerGetMetersArgs;

/// Module-scoped alias for [`CoreProfilerGetMetersOptions`].
pub type GetMetersOptions = CoreProfilerGetMetersOptions;

/// Module-scoped alias for [`CoreProfilerGetMetersResponse`].
pub type GetMetersResponse = CoreProfilerGetMetersResponse;
