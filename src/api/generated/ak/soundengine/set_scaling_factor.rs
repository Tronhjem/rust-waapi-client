// Generated bindings for ak.soundengine.setScalingFactor



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.soundengine.setScalingFactor`.
///
/// Sets the scaling factor of a game object. You can modify the attenuation computations on this game object to simulate sounds with a larger or smaller affected areas. See `AK::SoundEngine::SetScalingFactor`.
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
    /// Creates a new [`SoundengineSetScalingFactorArgs`] with the required fields.
    pub fn new(game_object: GameObjectRef, attenuation_scaling_factor: f64) -> Self {
        Self {
            game_object,
            attenuation_scaling_factor,
        }
    }
}
