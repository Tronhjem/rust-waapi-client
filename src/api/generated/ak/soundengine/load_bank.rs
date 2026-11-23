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
    pub fn new(sound_bank: SoundEngineObjectRef) -> Self {
        Self { sound_bank }
    }
}
