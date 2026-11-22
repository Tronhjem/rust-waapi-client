// Generated bindings for topic ak.wwise.core.profiler.gameObjectReset

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.profiler.gameObjectReset`.
///
/// Sent when the game objects have been reset, such as closing a connection to a game while profiling.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGameObjectResetPayload {}

/// Module-scoped alias for [`CoreProfilerGameObjectResetPayload`].
pub type GameObjectResetPayload = CoreProfilerGameObjectResetPayload;
