// Generated bindings for ak.wwise.core.profiler.getGameObjects

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getGameObjects`.
///
/// Retrieves the game objects at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetGameObjectsArgs {
    /// The time in milliseconds to query for game objects.
    pub time: serde_json::Value,
}

impl CoreProfilerGetGameObjectsArgs {
    /// Creates a new [`CoreProfilerGetGameObjectsArgs`] with the required fields.
    pub fn new(time: serde_json::Value) -> Self {
        Self { time }
    }
}

/// Response from `ak.wwise.core.profiler.getGameObjects`.
///
/// Retrieves the game objects at a specific profiler capture time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetGameObjectsResponse {
    /// An array of objects containing game object data.
    #[serde(rename = "return")]
    pub return_: Vec<WwiseObjectRef>,
}
