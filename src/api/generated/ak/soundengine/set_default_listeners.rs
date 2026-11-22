// Generated bindings for ak.soundengine.setDefaultListeners



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.soundengine.setDefaultListeners`.
///
/// Sets the default active listeners for all subsequent game objects that are registered. See `AK::SoundEngine::SetDefaultListeners`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetDefaultListenersArgs {
    /// Array of listener game object IDs. Game objects must have been previously registered.
    pub listeners: Vec<GameObjectRef>,
}

impl SoundengineSetDefaultListenersArgs {
    /// Creates a new [`SoundengineSetDefaultListenersArgs`] with the required fields.
    pub fn new(listeners: Vec<GameObjectRef>) -> Self {
        Self {
            listeners,
        }
    }
}
