// Generated bindings for ak.soundengine.setListeners

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setListeners`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetListenersArgs {
    /// Emitter game object.
    pub emitter: GameObjectRef,
    /// Array of listener game object IDs. Game objects must have been previously registered.
    pub listeners: Vec<GameObjectRef>,
}

impl SoundengineSetListenersArgs {
    pub fn new(emitter: GameObjectRef, listeners: Vec<GameObjectRef>) -> Self {
        Self { emitter, listeners }
    }
}
