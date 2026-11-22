// Generated bindings for ak.soundengine.getState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.getState`.
///
/// Gets the current state of a State Group. When using setState just prior to getState, allow a brief delay (no more than 10ms) for the information to update in the sound engine.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineGetStateArgs {
    /// Either the ID (GUID), unique qualified name, or path of the State Group.
    #[serde(rename = "stateGroup")]
    pub state_group: ObjectRef,
}

impl SoundengineGetStateArgs {
    /// Creates a new [`SoundengineGetStateArgs`] with the required fields.
    pub fn new(state_group: impl Into<ObjectRef>) -> Self {
        Self {
            state_group: state_group.into(),
        }
    }
}

impl From<ObjectRef> for SoundengineGetStateArgs {
    fn from(value: ObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SoundengineGetStateArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SoundengineGetStateArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for SoundengineGetStateArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Options for `ak.soundengine.getState`.
///
/// Specifies the return options for every object returned.
pub type SoundengineGetStateOptions = ObjectReturnOptions;

/// Response from `ak.soundengine.getState`.
///
/// Gets the current state of a State Group. When using setState just prior to getState, allow a brief delay (no more than 10ms) for the information to update in the sound engine.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundengineGetStateResponse {
    /// A Wwise object. Use the return options to specify which property is required.
    #[serde(rename = "return")]
    pub state: WwiseObject,
}

/// Module-scoped alias for [`SoundengineGetStateArgs`].
pub type GetStateArgs = SoundengineGetStateArgs;

/// Module-scoped alias for [`SoundengineGetStateOptions`].
pub type GetStateOptions = SoundengineGetStateOptions;

/// Module-scoped alias for [`SoundengineGetStateResponse`].
pub type GetStateResponse = SoundengineGetStateResponse;
