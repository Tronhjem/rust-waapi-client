// Generated bindings for ak.wwise.core.transport.getState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.getState`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportGetStateArgs {
    /// The transport object ID to be used with all other ak.wwise.core.transport functions.
    pub transport: u32,
}

impl CoreTransportGetStateArgs {
    pub fn new(transport: u32) -> Self {
        Self { transport }
    }
}

/// Response from `ak.wwise.core.transport.getState`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreTransportGetStateResponse {
    /// The state of the transport object.
    pub state: TransportState,
}
