// Generated bindings for ak.soundengine.stopAll

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.stopAll`.
///
/// Stop playing the current content associated to the specified game object ID. If no game object is specified, all sounds are stopped. See `AK::SoundEngine::StopAll`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SoundengineStopAllArgs {
    /// The game object on which to stop playback. If not specified, stops all sounds.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
}

impl SoundengineStopAllArgs {
    /// Creates a new [`SoundengineStopAllArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// The game object on which to stop playback. If not specified, stops all sounds.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

/// Module-scoped alias for [`SoundengineStopAllArgs`].
pub type StopAllArgs = SoundengineStopAllArgs;
