// Generated bindings for ak.wwise.core.log.clear

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.log.clear`.
///
/// Clears the logs on the specified channel.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreLogClearArgs {
    /// The channel to clear. Default is 'general'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<LogChannel>,
}

impl CoreLogClearArgs {
    /// Creates a new [`CoreLogClearArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// The channel to clear. Default is 'general'.
    pub fn channel(mut self, channel: impl Into<LogChannel>) -> Self {
        self.channel = Some(channel.into());
        self
    }
}

/// Module-scoped alias for [`CoreLogClearArgs`].
pub type ClearArgs = CoreLogClearArgs;
