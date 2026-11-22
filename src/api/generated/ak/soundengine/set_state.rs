// Generated bindings for ak.soundengine.setState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setState`.
///
/// Sets the State of a State Group. See `AK::SoundEngine::SetState`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetStateArgs {
    /// Either the ID (GUID), name, or Short ID of the State.
    pub state: SoundEngineObjectRef,
    /// Either the ID (GUID), name, or Short ID of the State Group.
    #[serde(rename = "stateGroup")]
    pub state_group: SoundEngineObjectRef,
}

impl SoundengineSetStateArgs {
    /// Creates a new [`SoundengineSetStateArgs`] with the required fields.
    pub fn new(
        state_group: impl Into<SoundEngineObjectRef>,
        state: impl Into<SoundEngineObjectRef>,
    ) -> Self {
        Self {
            state_group: state_group.into(),
            state: state.into(),
        }
    }
}

impl<T0: Into<SoundEngineObjectRef>, T1: Into<SoundEngineObjectRef>> From<(T0, T1)>
    for SoundengineSetStateArgs
{
    fn from((state_group, state): (T0, T1)) -> Self {
        Self::new(state_group, state)
    }
}

/// Module-scoped alias for [`SoundengineSetStateArgs`].
pub type SetStateArgs = SoundengineSetStateArgs;
