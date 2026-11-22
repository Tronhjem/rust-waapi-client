// Generated bindings for ak.soundengine.setGameObjectAuxSendValues

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setGameObjectAuxSendValues`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetGameObjectAuxSendValuesArgs {
    /// Associated game object ID.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// Array of AkAuxSendValue structures.
    #[serde(rename = "auxSendValues")]
    pub aux_send_values: Vec<AuxSendValue>,
}

impl SoundengineSetGameObjectAuxSendValuesArgs {
    pub fn new(game_object: GameObjectRef, aux_send_values: Vec<AuxSendValue>) -> Self {
        Self {
            game_object,
            aux_send_values,
        }
    }
}
