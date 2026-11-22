// Generated bindings for ak.wwise.core.transport.useOriginals

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.useOriginals`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportUseOriginalsArgs {
    /// If true, all playback will use original wav files. If false all playback will use converted files.
    pub enable: bool,
}

impl CoreTransportUseOriginalsArgs {
    pub fn new(enable: bool) -> Self {
        Self { enable }
    }
}
