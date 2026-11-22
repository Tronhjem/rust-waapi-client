// Generated bindings for topic ak.wwise.core.transport.stateChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.transport.stateChanged`.
///
/// Sent when the transport's state has changed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreTransportStateChangedPayload {
    /// The ID (GUID) of the object controlled by the transport object.
    pub object: String,
    /// The game object used by the transport object.
    #[serde(rename = "gameObject", default)]
    pub game_object: Option<GameObjectRef>,
    /// The ID of the watched transport.
    pub transport: u32,
    /// The state of the transport object.
    pub state: TransportState,
}

/// Subscription options for `ak.wwise.core.transport.stateChanged`.
///
/// Sent when the transport's state has changed.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportStateChangedOptions {
    /// The ID of the watched transport.
    pub transport: u32,
}

impl CoreTransportStateChangedOptions {
    /// Creates a new [`CoreTransportStateChangedOptions`] with the required fields.
    pub fn new(transport: u32) -> Self {
        Self { transport }
    }
}

/// Module-scoped alias for [`CoreTransportStateChangedPayload`].
pub type StateChangedPayload = CoreTransportStateChangedPayload;

/// Module-scoped alias for [`CoreTransportStateChangedOptions`].
pub type StateChangedOptions = CoreTransportStateChangedOptions;
