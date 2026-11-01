// Generated bindings for ak.soundengine.setRTPCValue

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.soundengine.setRTPCValue`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetrtpcvalueArgs {
    /// The RTPC GUID, name, or short ID.
    pub rtpc: RtpcRef,
    /// The new value.
    pub value: f64,
    /// The game object identifier.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
}

impl SoundengineSetrtpcvalueArgs {
    pub fn new(rtpc: RtpcRef, value: f64, game_object: GameObjectRef) -> Self {
        Self {
            rtpc,
            value,
            game_object,
        }
    }
}

/// Options for `ak.soundengine.setRTPCValue`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct SoundengineSetrtpcvalueOptions {
}

impl SoundengineSetrtpcvalueOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
