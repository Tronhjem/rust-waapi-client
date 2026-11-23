// Generated bindings for ak.wwise.core.soundbank.convertExternalSources

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.soundbank.convertExternalSources`.
///
/// Converts the external sources files for the project as detailed in the wsources file, and places them into either the default folder, or the folder specified by the output argument. External Sources are a special type of source that you can put in a Sound object in Wwise. It indicates that the real sound data will be provided at run time. While External Source conversion is also triggered by SoundBank generation, this operation can be used to process sources not contained in the Wwise Project. Please see Wwise SDK help page "Integrating External Sources".
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSoundbankConvertExternalSourcesArgs {
    /// An array of external sources files and corrisponding arguments.
    pub sources: Vec<ExternalSourceInfo>,
}

impl CoreSoundbankConvertExternalSourcesArgs {
    /// Creates a new [`CoreSoundbankConvertExternalSourcesArgs`] with the required fields.
    pub fn new(sources: impl Into<Vec<ExternalSourceInfo>>) -> Self {
        Self {
            sources: sources.into(),
        }
    }
}
