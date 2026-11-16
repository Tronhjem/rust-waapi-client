// Generated bindings for ak.soundengine.unregisterGameObj

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.unregisterGameObj`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineUnregisterGameObjArgs {
    /// ID of the game object to be unregistered
    /// Valid range is [0, 0xFFFFFFFFFFFFFFDF]
    /// Use \c AK_INVALID_GAME_OBJECT to unregister all game objects.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundengineUnregisterGameObjArgs {
    pub fn new(game_object: GameObjectRef) -> Self {
        Self { game_object }
    }
}
