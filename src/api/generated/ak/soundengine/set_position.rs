// Generated bindings for ak.soundengine.setPosition

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setPosition`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetPositionArgs {
    /// Game Object identifier.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// The 3D position to set for the game object.
    pub position: Position,
}

impl SoundengineSetPositionArgs {
    pub fn new(game_object: GameObjectRef, position: Position) -> Self {
        Self {
            game_object,
            position,
        }
    }
}
