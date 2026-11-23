// Generated bindings for topic ak.wwise.core.log.itemAdded

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.log.itemAdded`.
///
/// Sent when an item is added to the log. This could be used to retrieve items added to the SoundBank generation log. To retrieve the complete log, see `ak.wwise.core.log.get`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreLogItemAddedPayload {
    /// The channel on which the item was added.
    pub channel: LogChannel,
    /// A log entry.
    pub item: LogCentralItem,
}
