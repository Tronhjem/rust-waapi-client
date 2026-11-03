// Generated bindings for ak.soundengine.postEvent

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.postEvent`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundenginePostEventArgs {
    /// The unique ID of the game object.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), name, or Short ID of the Event.
    pub event: SoundEngineObjectRef,
}

impl SoundenginePostEventArgs {
    pub fn new(event: SoundEngineObjectRef) -> Self {
        Self {
            event,
            game_object: None,
        }
    }

    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

/// Response from `ak.soundengine.postEvent`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundenginePostEventResponse {
    /// The playing ID of the Event launched, or \c AK_INVALID_PLAYING_ID if posting the event failed.
    #[serde(rename = "return")]
    pub playing_id: u32,
}

pub type SoundenginePosteventArgs = SoundenginePostEventArgs;

pub type SoundenginePosteventResponse = SoundenginePostEventResponse;
