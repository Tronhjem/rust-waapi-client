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
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<String> for SoundenginePostMsgMonitorArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SoundenginePostMsgMonitorArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for SoundenginePostMsgMonitorArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`SoundenginePostMsgMonitorArgs`].
pub type PostMsgMonitorArgs = SoundenginePostMsgMonitorArgs;
