// Generated bindings for ak.soundengine.setMultiplePositions

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setMultiplePositions`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetMultiplePositionsArgs {
    /// Game Object identifier.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// Array of positions to apply.
    pub positions: Vec<MultiPosition>,
    /// Uses values from <tt>AkMultiPositionType</tt>.
    #[serde(rename = "multiPositionType")]
    pub multi_position_type: MultiPositionType,
}

impl SoundengineSetMultiplePositionsArgs {
    pub fn new(
        game_object: GameObjectRef,
        positions: Vec<MultiPosition>,
        multi_position_type: MultiPositionType,
    ) -> Self {
        Self {
            game_object,
            positions,
            multi_position_type,
        }
    }
}
