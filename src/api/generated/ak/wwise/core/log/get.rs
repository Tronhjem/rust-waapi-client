// Generated bindings for ak.wwise.core.log.get

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.log.get`.
///
/// Retrieves the latest log for a specific channel. See `ak.wwise.core.log.itemAdded` to be notified when an item is added to the log. The log is empty when used in WwiseConsole.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreLogGetArgs {
    /// The log channel.
    pub channel: LogChannel,
}

impl CoreLogGetArgs {
    /// Creates a new [`CoreLogGetArgs`] with the required fields.
    pub fn new(channel: LogChannel) -> Self {
        Self { channel }
    }
}

impl From<LogChannel> for CoreLogGetArgs {
    fn from(value: LogChannel) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.log.get`.
///
/// Retrieves the latest log for a specific channel. See `ak.wwise.core.log.itemAdded` to be notified when an item is added to the log. The log is empty when used in WwiseConsole.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreLogGetResponse {
    /// The entries of the log.
    pub items: Vec<LogCentralItem>,
}

/// Module-scoped alias for [`CoreLogGetArgs`].
pub type GetArgs = CoreLogGetArgs;

/// Module-scoped alias for [`CoreLogGetResponse`].
pub type GetResponse = CoreLogGetResponse;
