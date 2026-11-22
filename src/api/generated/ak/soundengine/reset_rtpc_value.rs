// Generated bindings for ak.soundengine.resetRTPCValue

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.resetRTPCValue`.
///
/// Resets the value of a real-time parameter control to its default value, as specified in the Wwise project. See `AK::SoundEngine::ResetRTPCValue`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineResetRtpcValueArgs {
    /// Associated game object ID. The default value is transport game object.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), name, or Short ID of the real-time parameter control.
    pub rtpc: SoundEngineObjectRef,
}

impl SoundengineResetRtpcValueArgs {
    /// Creates a new [`SoundengineResetRtpcValueArgs`] with the required fields.
    pub fn new(rtpc: impl Into<SoundEngineObjectRef>) -> Self {
        Self {
            rtpc: rtpc.into(),
            game_object: None,
        }
    }

    /// Associated game object ID. The default value is transport game object.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

impl From<SoundEngineObjectRef> for SoundengineResetRtpcValueArgs {
    fn from(value: SoundEngineObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SoundengineResetRtpcValueArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SoundengineResetRtpcValueArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for SoundengineResetRtpcValueArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

impl From<u32> for SoundengineResetRtpcValueArgs {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`SoundengineResetRtpcValueArgs`].
pub type ResetRtpcValueArgs = SoundengineResetRtpcValueArgs;
