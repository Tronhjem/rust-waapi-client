// Generated bindings for ak.wwise.core.audio.mute

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.mute`.
///
/// Mutes an object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioMuteArgs {
    /// Whether the object is muted or not.
    pub value: bool,
    /// An array of objects to mute.
    pub objects: Vec<ObjectRef>,
}

impl CoreAudioMuteArgs {
    /// Creates a new [`CoreAudioMuteArgs`] with the required fields.
    pub fn new(objects: Vec<ObjectRef>, value: bool) -> Self {
        Self { objects, value }
    }
}

impl<T0: Into<Vec<ObjectRef>>, T1: Into<bool>> From<(T0, T1)> for CoreAudioMuteArgs {
    fn from((objects, value): (T0, T1)) -> Self {
        Self::new(objects.into(), value.into())
    }
}

/// Module-scoped alias for [`CoreAudioMuteArgs`].
pub type MuteArgs = CoreAudioMuteArgs;
