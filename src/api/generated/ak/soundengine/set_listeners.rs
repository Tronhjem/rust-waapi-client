// Generated bindings for ak.soundengine.setListeners

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setListeners`.
///
/// Sets a single game object's active listeners. By default, all new game objects have no listeners active, but this behavior can be overridden with `SetDefaultListeners()`. Inactive listeners are not computed. See `AK::SoundEngine::SetListeners`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetListenersArgs {
    /// Emitter game object.
    pub emitter: GameObjectRef,
    /// Array of listener game object IDs. Game objects must have been previously registered.
    pub listeners: Vec<GameObjectRef>,
}

impl SoundengineSetListenersArgs {
    /// Creates a new [`SoundengineSetListenersArgs`] with the required fields.
    pub fn new(emitter: impl Into<GameObjectRef>, listeners: Vec<GameObjectRef>) -> Self {
        Self {
            emitter: emitter.into(),
            listeners,
        }
    }
}

impl<T0: Into<GameObjectRef>, T1: Into<Vec<GameObjectRef>>> From<(T0, T1)>
    for SoundengineSetListenersArgs
{
    fn from((emitter, listeners): (T0, T1)) -> Self {
        Self::new(emitter, listeners.into())
    }
}

/// Module-scoped alias for [`SoundengineSetListenersArgs`].
pub type SetListenersArgs = SoundengineSetListenersArgs;
