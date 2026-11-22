// Generated bindings for ak.soundengine.postMsgMonitor

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.postMsgMonitor`.
///
/// Display a message in the Profiler's Capture Log view.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundenginePostMsgMonitorArgs {
    /// The message to display.
    pub message: String,
}

impl SoundenginePostMsgMonitorArgs {
    /// Creates a new [`SoundenginePostMsgMonitorArgs`] with the required fields.
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
