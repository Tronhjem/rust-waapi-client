// Generated bindings for ak.soundengine.registerGameObj

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.registerGameObj`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineRegisterGameObjArgs {
    /// Name of the game object (for monitoring purpose).
    pub name: String,
    /// ID of the game object to be registered. Valid range is [0, 0xFFFFFFFFFFFFFFDF].
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundengineRegisterGameObjArgs {
    pub fn new(game_object: GameObjectRef, name: String) -> Self {
        Self { game_object, name }
    }
}
