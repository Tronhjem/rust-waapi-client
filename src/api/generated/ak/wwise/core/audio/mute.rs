// Generated bindings for ak.wwise.core.audio.mute

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.mute`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioMuteArgs {
    /// Whether the object is muted or not.
    pub value: bool,
    /// An array of objects to mute.
    pub objects: Vec<ObjectRef>,
}

impl CoreAudioMuteArgs {
    pub fn new(objects: Vec<ObjectRef>, value: bool) -> Self {
        Self { objects, value }
    }
}
