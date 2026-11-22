// Generated bindings for ak.wwise.core.sound.setActiveSource

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sound.setActiveSource`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSoundSetActiveSourceArgs {
    /// Platform ID (GUID) or name. This is an optional argument. When not specified, the current platform is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Sound ID (GUID), name, or path.
    pub sound: ObjectRef,
    /// Source ID (GUID), name, or path. The source must be a child of the sound.
    pub source: ObjectRef,
}

impl CoreSoundSetActiveSourceArgs {
    pub fn new(sound: ObjectRef, source: ObjectRef) -> Self {
        Self {
            sound,
            source,
            platform: None,
        }
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
