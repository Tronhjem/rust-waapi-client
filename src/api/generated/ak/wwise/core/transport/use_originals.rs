// Generated bindings for ak.wwise.core.transport.useOriginals

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.useOriginals`.
///
/// Sets the Original/Converted transport toggle globally. This allows playing the original or the converted sound files.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportUseOriginalsArgs {
    /// If true, all playback will use original wav files. If false all playback will use converted files.
    pub enable: bool,
}

impl CoreTransportUseOriginalsArgs {
    /// Creates a new [`CoreTransportUseOriginalsArgs`] with the required fields.
    pub fn new(enable: bool) -> Self {
        Self { enable }
    }
}

impl From<bool> for CoreTransportUseOriginalsArgs {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`CoreTransportUseOriginalsArgs`].
pub type UseOriginalsArgs = CoreTransportUseOriginalsArgs;
