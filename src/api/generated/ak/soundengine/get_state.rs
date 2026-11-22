// Generated bindings for ak.soundengine.getState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.getState`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineGetStateArgs {
    /// Either the ID (GUID), unique qualified name, or path of the State Group.
    #[serde(rename = "stateGroup")]
    pub state_group: ObjectRef,
}

impl SoundengineGetStateArgs {
    pub fn new(state_group: ObjectRef) -> Self {
        Self { state_group }
    }
}

/// Options for `ak.soundengine.getState`.
pub type SoundengineGetStateOptions = ObjectReturnOptions;

/// Response from `ak.soundengine.getState`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundengineGetStateResponse {
    #[serde(rename = "return")]
    pub state: WwiseObject,
}
