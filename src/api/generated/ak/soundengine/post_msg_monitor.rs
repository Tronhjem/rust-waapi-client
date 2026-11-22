// Generated bindings for ak.soundengine.postMsgMonitor

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.postMsgMonitor`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundenginePostMsgMonitorArgs {
    /// The message to display.
    pub message: String,
}

impl SoundenginePostMsgMonitorArgs {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
