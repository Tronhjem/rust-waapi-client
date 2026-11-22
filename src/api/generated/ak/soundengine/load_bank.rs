// Generated bindings for ak.soundengine.loadBank

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.loadBank`.
///
/// Load a SoundBank. See `AK::SoundEngine::LoadBank`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineLoadBankArgs {
    /// Either the ID (GUID), name, or Short ID of the SoundBank.
    #[serde(rename = "soundBank")]
    pub sound_bank: SoundEngineObjectRef,
}

impl SoundengineLoadBankArgs {
    /// Creates a new [`SoundengineLoadBankArgs`] with the required fields.
    pub fn new(sound_bank: impl Into<SoundEngineObjectRef>) -> Self {
        Self {
            sound_bank: sound_bank.into(),
        }
    }
}

impl From<SoundEngineObjectRef> for SoundengineLoadBankArgs {
    fn from(value: SoundEngineObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SoundengineLoadBankArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SoundengineLoadBankArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for SoundengineLoadBankArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

impl From<u32> for SoundengineLoadBankArgs {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`SoundengineLoadBankArgs`].
pub type LoadBankArgs = SoundengineLoadBankArgs;
