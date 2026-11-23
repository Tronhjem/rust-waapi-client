// Generated bindings for ak.wwise.core.profiler.enableProfilerData

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.enableProfilerData`.
///
/// Specifies the type of data you want to capture. Overrides the user's profiler settings.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerEnableProfilerDataArgs {
    /// An array of data types.
    #[serde(rename = "dataTypes")]
    pub data_types: Vec<serde_json::Value>,
}

impl CoreProfilerEnableProfilerDataArgs {
    /// Creates a new [`CoreProfilerEnableProfilerDataArgs`] with the required fields.
    pub fn new(data_types: impl Into<Vec<serde_json::Value>>) -> Self {
        Self {
            data_types: data_types.into(),
        }
    }
}
