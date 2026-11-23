// Generated bindings for ak.soundengine.registerGameObj

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.registerGameObj`.
///
/// Register a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. See `AK::SoundEngine::RegisterGameObj`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineRegisterGameObjArgs {
    /// Name of the game object (for monitoring purpose).
    pub name: String,
    /// ID of the game object to be registered. Valid range is `[0, 0xFFFFFFFFFFFFFFDF]`.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundengineRegisterGameObjArgs {
    /// Creates a new [`SoundengineRegisterGameObjArgs`] with the required fields.
    pub fn new(game_object: GameObjectRef, name: impl Into<String>) -> Self {
        Self {
            game_object,
            name: name.into(),
        }
    }
}
