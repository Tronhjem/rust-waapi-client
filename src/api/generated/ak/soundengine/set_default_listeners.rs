// Generated bindings for ak.soundengine.setDefaultListeners

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setDefaultListeners`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetDefaultListenersArgs {
    /// Array of listener game object IDs
    /// Game objects must have been previously registered.
    pub listeners: Vec<GameObjectRef>,
}

impl SoundengineSetDefaultListenersArgs {
    pub fn new(listeners: Vec<GameObjectRef>) -> Self {
        Self { listeners }
    }
}
