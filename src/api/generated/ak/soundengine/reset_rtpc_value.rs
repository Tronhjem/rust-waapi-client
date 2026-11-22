// Generated bindings for ak.soundengine.resetRTPCValue

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.resetRTPCValue`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineResetRtpcValueArgs {
    /// Associated game object ID. The default value is transport game object.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), name, or Short ID of the real-time parameter control.
    pub rtpc: SoundEngineObjectRef,
}

impl SoundengineResetRtpcValueArgs {
    pub fn new(rtpc: SoundEngineObjectRef) -> Self {
        Self {
            rtpc,
            game_object: None,
        }
    }

    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}
