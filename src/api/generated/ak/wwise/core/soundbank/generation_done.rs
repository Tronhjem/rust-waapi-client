// Generated bindings for topic ak.wwise.core.soundbank.generationDone



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Payload for `ak.wwise.core.soundbank.generationDone`.
///
/// Sent when all SoundBanks are generated. Note: This notification is only sent when SoundBanks have been generated, it is not a reliable way to determine when `ak.wwise.core.soundbank.generate` has completed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSoundbankGenerationDonePayload {
    /// (DEPRECATED) The error message if an error occurred. Only present if an error occurred. You should check logs instead now.
    #[serde(default)]
    pub error: Option<String>,
    /// The SoundBank generation log. The log is empty when used in WwiseConsole.
    #[serde(default)]
    pub logs: Vec<LogCentralItem>,
}

