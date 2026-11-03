// Generated bindings for ak.soundengine.setRTPCValue

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setRTPCValue`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetRtpcValueArgs {
    /// Associated game object ID
    /// The default value is transport game object.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Value to set.
    pub value: f64,
    /// Either the ID (GUID), name, or Short ID of the real-time parameter control.
    pub rtpc: SoundEngineObjectRef,
}

impl SoundengineSetRtpcValueArgs {
    pub fn new(rtpc: SoundEngineObjectRef, value: f64) -> Self {
        Self {
            rtpc,
            value,
            game_object: None,
        }
    }

    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

pub type SoundengineSetrtpcvalueArgs = SoundengineSetRtpcValueArgs;
