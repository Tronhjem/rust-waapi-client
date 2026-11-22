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
    pub fn new(sound_bank: SoundEngineObjectRef) -> Self {
        Self {
            sound_bank,
        }
    }
}
