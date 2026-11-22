// Generated bindings for ak.soundengine.unregisterGameObj

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.unregisterGameObj`.
///
/// Unregisters a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. Unregistering a game object while it is in use is allowed, but the control over the parameters of this game object is lost. For example, say a sound associated with this game object is a 3D moving sound. It stops moving when the game object is unregistered, and there is no way to regain control over the game object. See `AK::SoundEngine::UnregisterGameObj`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineUnregisterGameObjArgs {
    /// ID of the game object to be unregistered. Valid range is `[0, 0xFFFFFFFFFFFFFFDF]`. Use `AK_INVALID_GAME_OBJECT` to unregister all game objects.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundengineUnregisterGameObjArgs {
    /// Creates a new [`SoundengineUnregisterGameObjArgs`] with the required fields.
    pub fn new(game_object: impl Into<GameObjectRef>) -> Self {
        Self {
            game_object: game_object.into(),
        }
    }
}

impl From<GameObjectRef> for SoundengineUnregisterGameObjArgs {
    fn from(value: GameObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<u64> for SoundengineUnregisterGameObjArgs {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<u32> for SoundengineUnregisterGameObjArgs {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`SoundengineUnregisterGameObjArgs`].
pub type UnregisterGameObjArgs = SoundengineUnregisterGameObjArgs;
