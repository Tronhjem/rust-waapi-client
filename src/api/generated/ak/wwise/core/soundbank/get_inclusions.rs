// Generated bindings for ak.wwise.core.soundbank.getInclusions

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.soundbank.getInclusions`.
///
/// Retrieves a SoundBank's inclusion list.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSoundbankGetInclusionsArgs {
    /// The ID (GUID), name, or path of the SoundBank to add an inclusion to.
    pub soundbank: ObjectRef,
}

impl CoreSoundbankGetInclusionsArgs {
    /// Creates a new [`CoreSoundbankGetInclusionsArgs`] with the required fields.
    pub fn new(soundbank: ObjectRef) -> Self {
        Self { soundbank }
    }
}

/// Response from `ak.wwise.core.soundbank.getInclusions`.
///
/// Retrieves a SoundBank's inclusion list.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSoundbankGetInclusionsResponse {
    /// An array of inclusions.
    #[serde(default)]
    pub inclusions: Vec<SoundBankInclusion>,
}
