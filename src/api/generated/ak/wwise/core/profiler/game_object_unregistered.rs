// Generated bindings for topic ak.wwise.core.profiler.gameObjectUnregistered



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Payload for `ak.wwise.core.profiler.gameObjectUnregistered`.
///
/// Sent when a game object has been unregistered.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGameObjectUnregisteredPayload {
    /// The time at which the game object was unregistered. This is the elapsed time in milliseconds since the initialization of the sound engine.
    pub time: i32,
    /// The game object ID for the entry.
    #[serde(rename = "gameObjectId")]
    pub game_object_id: GameObjectRef,
    /// The game object name for the entry.
    #[serde(rename = "gameObjectName")]
    pub game_object_name: String,
}

