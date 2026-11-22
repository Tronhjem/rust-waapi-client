// Generated bindings for ak.soundengine.postTrigger

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.postTrigger`.
///
/// Posts the specified Trigger. See `AK::SoundEngine::PostTrigger`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundenginePostTriggerArgs {
    /// Associated game object ID. Its default value is the transport game object.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), name, or Short ID of the Trigger.
    pub trigger: SoundEngineObjectRef,
}

impl SoundenginePostTriggerArgs {
    /// Creates a new [`SoundenginePostTriggerArgs`] with the required fields.
    pub fn new(trigger: SoundEngineObjectRef) -> Self {
        Self {
            trigger,
            game_object: None,
        }
    }

    /// Associated game object ID. Its default value is the transport game object.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}
