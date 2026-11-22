// Generated bindings for topic ak.wwise.core.profiler.gameObjectRegistered

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.profiler.gameObjectRegistered`.
///
/// Sent when a game object has been registered.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGameObjectRegisteredPayload {
    /// The time at which the game object was registered. This is the elapsed time in milliseconds since the initialization of the sound engine.
    pub time: i32,
    /// The game object ID for the entry.
    #[serde(rename = "gameObjectId")]
    pub game_object_id: GameObjectRef,
    /// The game object name for the entry.
    #[serde(rename = "gameObjectName")]
    pub game_object_name: String,
}

/// Module-scoped alias for [`CoreProfilerGameObjectRegisteredPayload`].
pub type GameObjectRegisteredPayload = CoreProfilerGameObjectRegisteredPayload;
