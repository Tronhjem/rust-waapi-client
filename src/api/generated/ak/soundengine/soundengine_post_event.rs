// Generated bindings for ak.soundengine.postEvent

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.soundengine.postEvent`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundenginePosteventArgs {
    /// The event GUID, name, or short ID.
    pub event: EventRef,
    /// The game object identifier.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundenginePosteventArgs {
    pub fn new(event: EventRef, game_object: GameObjectRef) -> Self {
        Self {
            event,
            game_object,
        }
    }
}

/// Options for `ak.soundengine.postEvent`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct SoundenginePosteventOptions {
}

impl SoundenginePosteventOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Response from `ak.soundengine.postEvent`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub struct SoundenginePosteventResponse {
    /// The playing ID.
    #[serde(rename = "playingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playing_id: Option<u32>,
}
