// Generated bindings for ak.wwise.core.sound.setActiveSource

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sound.setActiveSource`.
///
/// Sets which version of the source is being used for the specified sound. Use `ak.wwise.core.object.get` with the 'activeSource' return option to get the active source of a sound.
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
    /// Creates a new [`CoreSoundSetActiveSourceArgs`] with the required fields.
    pub fn new(sound: impl Into<ObjectRef>, source: impl Into<ObjectRef>) -> Self {
        Self {
            sound: sound.into(),
            source: source.into(),
            platform: None,
        }
    }

    /// Platform ID (GUID) or name. This is an optional argument. When not specified, the current platform is used.
    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}

impl<T0: Into<ObjectRef>, T1: Into<ObjectRef>> From<(T0, T1)> for CoreSoundSetActiveSourceArgs {
    fn from((sound, source): (T0, T1)) -> Self {
        Self::new(sound, source)
    }
}

/// Module-scoped alias for [`CoreSoundSetActiveSourceArgs`].
pub type SetActiveSourceArgs = CoreSoundSetActiveSourceArgs;
