// Generated bindings for ak.soundengine.unloadBank

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.unloadBank`.
///
/// Unload a SoundBank. See `AK::SoundEngine::UnloadBank`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineUnloadBankArgs {
    /// Either the ID (GUID), name, or Short ID of the SoundBank.
    #[serde(rename = "soundBank")]
    pub sound_bank: SoundEngineObjectRef,
}

impl SoundengineUnloadBankArgs {
    /// Creates a new [`SoundengineUnloadBankArgs`] with the required fields.
    pub fn new(sound_bank: impl Into<SoundEngineObjectRef>) -> Self {
        Self {
            sound_bank: sound_bank.into(),
        }
    }
}

impl From<SoundEngineObjectRef> for SoundengineUnloadBankArgs {
    fn from(value: SoundEngineObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SoundengineUnloadBankArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SoundengineUnloadBankArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for SoundengineUnloadBankArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

impl From<u32> for SoundengineUnloadBankArgs {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`SoundengineUnloadBankArgs`].
pub type UnloadBankArgs = SoundengineUnloadBankArgs;
