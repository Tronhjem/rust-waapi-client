// Generated bindings for ak.wwise.core.transport.getList

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.transport.getList`.
///
/// Returns the list of transport objects.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreTransportGetListResponse {
    /// An array of transport object.
    #[serde(default)]
    pub list: Vec<TransportObject>,
}

/// Module-scoped alias for [`CoreTransportGetListResponse`].
pub type GetListResponse = CoreTransportGetListResponse;
