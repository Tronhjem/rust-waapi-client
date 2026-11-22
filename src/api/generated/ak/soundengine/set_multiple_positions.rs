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
        game_object: impl Into<GameObjectRef>,
        positions: Vec<MultiPosition>,
        multi_position_type: MultiPositionType,
    ) -> Self {
        Self {
            game_object: game_object.into(),
            positions,
            multi_position_type,
        }
    }
}

impl<T0: Into<GameObjectRef>, T1: Into<Vec<MultiPosition>>, T2: Into<MultiPositionType>>
    From<(T0, T1, T2)> for SoundengineSetMultiplePositionsArgs
{
    fn from((game_object, positions, multi_position_type): (T0, T1, T2)) -> Self {
        Self::new(game_object, positions.into(), multi_position_type.into())
    }
}

/// Module-scoped alias for [`SoundengineSetMultiplePositionsArgs`].
pub type SetMultiplePositionsArgs = SoundengineSetMultiplePositionsArgs;
