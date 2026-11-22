// Generated bindings for ak.soundengine.setPosition



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.soundengine.setPosition`.
///
/// Sets the position of a game object. See `AK::SoundEngine::SetPosition`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetPositionArgs {
    /// The 3D position to set for the game object.
    pub position: Position,
    /// Game Object identifier.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundengineSetPositionArgs {
    /// Creates a new [`SoundengineSetPositionArgs`] with the required fields.
    pub fn new(game_object: GameObjectRef, position: Position) -> Self {
        Self {
            game_object,
            position,
        }
    }
}
