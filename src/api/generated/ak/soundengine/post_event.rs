// Generated bindings for ak.soundengine.postEvent

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.postEvent`.
///
/// Asynchronously post an Event to the sound engine (by event ID). See `AK::SoundEngine::PostEvent`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundenginePostEventArgs {
    /// The unique ID of the game object. When not specified, `AK_INVALID_GAME_OBJECT` is used.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), name, or Short ID of the Event.
    pub event: SoundEngineObjectRef,
}

impl SoundenginePostEventArgs {
    /// Creates a new [`SoundenginePostEventArgs`] with the required fields.
    pub fn new(event: impl Into<SoundEngineObjectRef>) -> Self {
        Self {
            event: event.into(),
            game_object: None,
        }
    }

    /// The unique ID of the game object. When not specified, `AK_INVALID_GAME_OBJECT` is used.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

impl From<SoundEngineObjectRef> for SoundenginePostEventArgs {
    fn from(value: SoundEngineObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SoundenginePostEventArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SoundenginePostEventArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for SoundenginePostEventArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

impl From<u32> for SoundenginePostEventArgs {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.soundengine.postEvent`.
///
/// Asynchronously post an Event to the sound engine (by event ID). See `AK::SoundEngine::PostEvent`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundenginePostEventResponse {
    /// The playing ID of the Event launched, or `AK_INVALID_PLAYING_ID` if posting the event failed.
    #[serde(rename = "return")]
    pub playing_id: u32,
}

/// Module-scoped alias for [`SoundenginePostEventArgs`].
pub type PostEventArgs = SoundenginePostEventArgs;

/// Module-scoped alias for [`SoundenginePostEventResponse`].
pub type PostEventResponse = SoundenginePostEventResponse;
