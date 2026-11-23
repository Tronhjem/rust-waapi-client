// Generated bindings for ak.wwise.core.log.addItem

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.log.addItem`.
///
/// Adds a new item to the logs on the specified channel.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreLogAddItemArgs {
    /// The severity of the message to add. Default is 'Message'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<LogSeverity>,
    /// The text of the message to add.
    pub message: String,
    /// The channel on which to add the message. Default is 'general'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<LogChannel>,
}

impl CoreLogAddItemArgs {
    /// Creates a new [`CoreLogAddItemArgs`] with the required fields.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            severity: None,
            channel: None,
        }
    }

    /// The severity of the message to add. Default is 'Message'.
    pub fn severity(mut self, severity: LogSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// The channel on which to add the message. Default is 'general'.
    pub fn channel(mut self, channel: LogChannel) -> Self {
        self.channel = Some(channel);
        self
    }
}
