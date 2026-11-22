// Generated bindings for ak.wwise.core.transport.create

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.create`.
///
/// Creates a transport object for the given Wwise object. The return transport object can be used to play, stop, pause and resume the Wwise object via the other transport functions.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportCreateArgs {
    /// The ID (GUID), name, or path of the object to control via the transport object.
    pub object: ObjectRef,
    /// The game object to use for playback.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
}

impl CoreTransportCreateArgs {
    /// Creates a new [`CoreTransportCreateArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>) -> Self {
        Self {
            object: object.into(),
            game_object: None,
        }
    }

    /// The game object to use for playback.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

impl From<ObjectRef> for CoreTransportCreateArgs {
    fn from(value: ObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreTransportCreateArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CoreTransportCreateArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreTransportCreateArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.transport.create`.
///
/// Creates a transport object for the given Wwise object. The return transport object can be used to play, stop, pause and resume the Wwise object via the other transport functions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreTransportCreateResponse {
    /// Transport object ID to be used with all other ak.wwise.core.transport functions.
    pub transport: u32,
}

/// Module-scoped alias for [`CoreTransportCreateArgs`].
pub type CreateArgs = CoreTransportCreateArgs;

/// Module-scoped alias for [`CoreTransportCreateResponse`].
pub type CreateResponse = CoreTransportCreateResponse;
