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
    pub fn new(game_object: impl Into<GameObjectRef>, name: impl Into<String>) -> Self {
        Self {
            game_object: game_object.into(),
            name: name.into(),
        }
    }
}

impl<T0: Into<GameObjectRef>, T1: Into<String>> From<(T0, T1)> for SoundengineRegisterGameObjArgs {
    fn from((game_object, name): (T0, T1)) -> Self {
        Self::new(game_object, name)
    }
}

/// Module-scoped alias for [`SoundengineRegisterGameObjArgs`].
pub type RegisterGameObjArgs = SoundengineRegisterGameObjArgs;
