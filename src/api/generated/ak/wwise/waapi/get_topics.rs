// Generated bindings for ak.wwise.waapi.getTopics

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.waapi.getTopics`.
///
/// Retrieves the list of topics to which a client can subscribe.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WaapiGetTopicsResponse {
    /// Array of topic URIs.
    #[serde(default)]
    pub topics: Vec<String>,
}

/// Module-scoped alias for [`WaapiGetTopicsResponse`].
pub type GetTopicsResponse = WaapiGetTopicsResponse;
