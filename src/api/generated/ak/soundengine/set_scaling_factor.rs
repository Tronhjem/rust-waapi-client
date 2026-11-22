// Generated bindings for ak.soundengine.setScalingFactor

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setScalingFactor`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetScalingFactorArgs {
    /// The game object identifier.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// The scaling factor, where 1 means 100%, 0.5 means 50%, 2 means 200%, and so on.
    #[serde(rename = "attenuationScalingFactor")]
    pub attenuation_scaling_factor: f64,
}

impl SoundengineSetScalingFactorArgs {
    pub fn new(game_object: GameObjectRef, attenuation_scaling_factor: f64) -> Self {
        Self {
            game_object,
            attenuation_scaling_factor,
        }
    }
}
