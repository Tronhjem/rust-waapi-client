// Generated bindings for ak.soundengine.setRTPCValue

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setRTPCValue`.
///
/// Sets the value of a Game Parameter for real-time parameter control. See `AK::SoundEngine::SetRTPCValue`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetRtpcValueArgs {
    /// Value to set for the specified Game Parameter.
    pub value: f64,
    /// Associated game object ID. The default value is transport game object.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), name, or Short ID of the Game Parameter.
    pub rtpc: SoundEngineObjectRef,
}

impl SoundengineSetRtpcValueArgs {
    /// Creates a new [`SoundengineSetRtpcValueArgs`] with the required fields.
    pub fn new(rtpc: impl Into<SoundEngineObjectRef>, value: f64) -> Self {
        Self {
            rtpc: rtpc.into(),
            value,
            game_object: None,
        }
    }

    /// Associated game object ID. The default value is transport game object.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

impl<T0: Into<SoundEngineObjectRef>, T1: Into<f64>> From<(T0, T1)> for SoundengineSetRtpcValueArgs {
    fn from((rtpc, value): (T0, T1)) -> Self {
        Self::new(rtpc, value.into())
    }
}

/// Module-scoped alias for [`SoundengineSetRtpcValueArgs`].
pub type SetRtpcValueArgs = SoundengineSetRtpcValueArgs;
