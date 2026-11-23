// Generated bindings for ak.soundengine.setMultiplePositions

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setMultiplePositions`.
///
/// Sets multiple positions for a single game object. Setting multiple positions for a single game object is a way to simulate multiple emission sources while using the resources of only one voice. This can be used to simulate wall openings, area sounds, or multiple objects emitting the same sound in the same area. See `AK::SoundEngine::SetMultiplePositions`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetMultiplePositionsArgs {
    /// Game Object identifier.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// Array of positions to apply.
    pub positions: Vec<MultiPosition>,
    /// Uses values from `AkMultiPositionType`.
    #[serde(rename = "multiPositionType")]
    pub multi_position_type: MultiPositionType,
}

impl SoundengineSetMultiplePositionsArgs {
    /// Creates a new [`SoundengineSetMultiplePositionsArgs`] with the required fields.
    pub fn new(
        game_object: GameObjectRef,
        positions: impl Into<Vec<MultiPosition>>,
        multi_position_type: MultiPositionType,
    ) -> Self {
        Self {
            game_object,
            positions: positions.into(),
            multi_position_type,
        }
    }
}
