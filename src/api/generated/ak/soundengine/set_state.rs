// Generated bindings for ak.soundengine.setState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setState`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetStateArgs {
    /// Either the ID (GUID), name, or Short ID of the State.
    pub state: SoundEngineObjectRef,
    /// Either the ID (GUID), name, or Short ID of the State Group.
    #[serde(rename = "stateGroup")]
    pub state_group: SoundEngineObjectRef,
}

impl SoundengineSetStateArgs {
    pub fn new(state_group: SoundEngineObjectRef, state: SoundEngineObjectRef) -> Self {
        Self { state_group, state }
    }
}
