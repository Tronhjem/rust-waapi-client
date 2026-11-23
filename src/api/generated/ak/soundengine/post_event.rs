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
    pub fn new(event: SoundEngineObjectRef) -> Self {
        Self {
            event,
            game_object: None,
        }
    }

    /// The unique ID of the game object. When not specified, `AK_INVALID_GAME_OBJECT` is used.
    pub fn game_object(mut self, game_object: GameObjectRef) -> Self {
        self.game_object = Some(game_object);
        self
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
