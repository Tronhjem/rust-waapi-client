// Generated bindings for ak.soundengine.setSwitch

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setSwitch`.
///
/// Sets the State of a Switch Group. See `AK::SoundEngine::SetSwitch`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetSwitchArgs {
    /// Associated game object ID. Its default value is the transport game object.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), name, or short ID of the Switch Group.
    #[serde(rename = "switchGroup")]
    pub switch_group: SoundEngineObjectRef,
    /// Either the ID (GUID), name, or Short ID of the Switch State.
    #[serde(rename = "switchState")]
    pub switch_state: SoundEngineObjectRef,
}

impl SoundengineSetSwitchArgs {
    /// Creates a new [`SoundengineSetSwitchArgs`] with the required fields.
    pub fn new(
        switch_group: impl Into<SoundEngineObjectRef>,
        switch_state: impl Into<SoundEngineObjectRef>,
    ) -> Self {
        Self {
            switch_group: switch_group.into(),
            switch_state: switch_state.into(),
            game_object: None,
        }
    }

    /// Associated game object ID. Its default value is the transport game object.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

impl<T0: Into<SoundEngineObjectRef>, T1: Into<SoundEngineObjectRef>> From<(T0, T1)>
    for SoundengineSetSwitchArgs
{
    fn from((switch_group, switch_state): (T0, T1)) -> Self {
        Self::new(switch_group, switch_state)
    }
}

/// Module-scoped alias for [`SoundengineSetSwitchArgs`].
pub type SetSwitchArgs = SoundengineSetSwitchArgs;
