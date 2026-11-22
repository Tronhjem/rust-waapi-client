// Generated bindings for ak.wwise.core.soundbank.convertExternalSources

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.soundbank.convertExternalSources`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSoundbankConvertExternalSourcesArgs {
    /// An array of external sources files and corrisponding arguments.
    pub sources: Vec<ExternalSourceInfo>,
}

impl CoreSoundbankConvertExternalSourcesArgs {
    pub fn new(sources: Vec<ExternalSourceInfo>) -> Self {
        Self { sources }
    }
}
