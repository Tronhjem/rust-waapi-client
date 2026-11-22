// Generated bindings for ak.wwise.core.soundbank.processDefinitionFiles



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.soundbank.processDefinitionFiles`.
///
/// The path to the SoundBank Definition File.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSoundbankProcessDefinitionFilesArgs {
    /// An array of SoundBank definition files.
    pub files: Vec<String>,
}

impl CoreSoundbankProcessDefinitionFilesArgs {
    /// Creates a new [`CoreSoundbankProcessDefinitionFilesArgs`] with the required fields.
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files,
        }
    }
}
