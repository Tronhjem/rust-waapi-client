// Generated bindings for ak.soundengine.setGameObjectAuxSendValues

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setGameObjectAuxSendValues`.
///
/// Sets the Auxiliary Busses to route the specified game object. See `AK::SoundEngine::SetGameObjectAuxSendValues`.
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
    /// Creates a new [`SoundengineSetGameObjectAuxSendValuesArgs`] with the required fields.
    pub fn new(game_object: impl Into<GameObjectRef>, aux_send_values: Vec<AuxSendValue>) -> Self {
        Self {
            game_object: game_object.into(),
            aux_send_values,
        }
    }
}

impl<T0: Into<GameObjectRef>, T1: Into<Vec<AuxSendValue>>> From<(T0, T1)>
    for SoundengineSetGameObjectAuxSendValuesArgs
{
    fn from((game_object, aux_send_values): (T0, T1)) -> Self {
        Self::new(game_object, aux_send_values.into())
    }
}

/// Module-scoped alias for [`SoundengineSetGameObjectAuxSendValuesArgs`].
pub type SetGameObjectAuxSendValuesArgs = SoundengineSetGameObjectAuxSendValuesArgs;
