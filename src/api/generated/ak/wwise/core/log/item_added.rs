// Generated bindings for topic ak.wwise.core.log.itemAdded

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.log.itemAdded`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreLogItemAddedPayload {
    /// The channel on which the item was added.
    pub channel: LogChannel,
    pub item: LogCentralItem,
}
