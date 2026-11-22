// Generated bindings for topic ak.wwise.core.profiler.captureLog.itemAdded

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.profiler.captureLog.itemAdded`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerCaptureLogItemAddedPayload {
    /// The entry type. Equivalent to the Type column in the Capture Log.
    #[serde(rename = "type")]
    pub object_type: String,
    /// Normal: normal execution. Message: Shown in yellow in the capture log. Error: Shown in red in the capture log.
    pub severity: String,
    /// The name of the object for the entry.
    #[serde(rename = "objectName", default)]
    pub object_name: Option<String>,
    /// The time at which the event occurred. This is the elapsed time in milliseconds since the initialization of the sound engine.
    pub time: i32,
    /// The game object ID for the entry.
    #[serde(rename = "gameObjectId", default)]
    pub game_object_id: Option<GameObjectRef>,
    /// The game object name for the entry.
    #[serde(rename = "gameObjectName", default)]
    pub game_object_name: Option<String>,
    /// The playing ID for the entry.
    #[serde(rename = "playingId", default)]
    pub playing_id: Option<u32>,
    /// The description for the entry. Equivalent to the Description column in the Capture Log.
    pub description: String,
    /// The ID (GUID) of the object for the entry.
    #[serde(rename = "objectId", default)]
    pub object_id: Option<String>,
    /// The short ID (32-bit) of the object for the entry.
    #[serde(rename = "objectShortId", default)]
    pub object_short_id: Option<u32>,
    /// The error code name for the entry.
    #[serde(rename = "errorCodeName", default)]
    pub error_code_name: Option<String>,
}

/// Subscription options for `ak.wwise.core.profiler.captureLog.itemAdded`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreProfilerCaptureLogItemAddedOptions {
    /// Defines which types of capture log items to receive. This filter can help to reduce the amount of data serialized. When no types are specified, all types are being sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl CoreProfilerCaptureLogItemAddedOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn types(mut self, types: impl Into<Vec<String>>) -> Self {
        self.types = Some(types.into());
        self
    }
}
