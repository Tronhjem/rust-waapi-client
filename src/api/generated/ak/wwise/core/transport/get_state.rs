// Generated bindings for ak.wwise.core.transport.getState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.getState`.
///
/// Gets the state of the given transport object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportGetStateArgs {
    /// The transport object ID to be used with all other ak.wwise.core.transport functions.
    pub transport: u32,
}

impl CoreTransportGetStateArgs {
    /// Creates a new [`CoreTransportGetStateArgs`] with the required fields.
    pub fn new(transport: u32) -> Self {
        Self { transport }
    }
}

impl From<u32> for CoreTransportGetStateArgs {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.transport.getState`.
///
/// Gets the state of the given transport object.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreTransportGetStateResponse {
    /// The state of the transport object.
    pub state: TransportState,
}

/// Module-scoped alias for [`CoreTransportGetStateArgs`].
pub type GetStateArgs = CoreTransportGetStateArgs;

/// Module-scoped alias for [`CoreTransportGetStateResponse`].
pub type GetStateResponse = CoreTransportGetStateResponse;
