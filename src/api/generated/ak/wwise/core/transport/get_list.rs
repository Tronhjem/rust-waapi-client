// Generated bindings for ak.wwise.core.transport.getList

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.transport.getList`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreTransportGetListResponse {
    /// An array of transport object.
    #[serde(default)]
    pub list: Vec<TransportObject>,
}
