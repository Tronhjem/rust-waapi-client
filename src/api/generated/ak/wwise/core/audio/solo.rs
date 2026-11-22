// Generated bindings for ak.wwise.core.audio.solo

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.solo`.
///
/// Solos an object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioSoloArgs {
    /// Whether the object is soloed or not.
    pub value: bool,
    /// An array of objects to solo.
    pub objects: Vec<ObjectRef>,
}

impl CoreAudioSoloArgs {
    /// Creates a new [`CoreAudioSoloArgs`] with the required fields.
    pub fn new(objects: Vec<ObjectRef>, value: bool) -> Self {
        Self { objects, value }
    }
}

impl<T0: Into<Vec<ObjectRef>>, T1: Into<bool>> From<(T0, T1)> for CoreAudioSoloArgs {
    fn from((objects, value): (T0, T1)) -> Self {
        Self::new(objects.into(), value.into())
    }
}

/// Module-scoped alias for [`CoreAudioSoloArgs`].
pub type SoloArgs = CoreAudioSoloArgs;
