// Generated bindings for ak.soundengine.stopAll

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.stopAll`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineStopAllArgs {
    /// Specify a game object to stop only playback associated to the provided game object ID.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundengineStopAllArgs {
    pub fn new(game_object: GameObjectRef) -> Self {
        Self { game_object }
    }
}
