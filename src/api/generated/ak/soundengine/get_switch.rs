// Generated bindings for ak.soundengine.getSwitch

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.getSwitch`.
///
/// Gets the current state of a Switch Group for a given Game Object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineGetSwitchArgs {
    /// The unique ID of the game object. When not specified, `AK_INVALID_GAME_OBJECT` is used.
    #[serde(rename = "gameObject", skip_serializing_if = "Option::is_none")]
    pub game_object: Option<GameObjectRef>,
    /// Either the ID (GUID), unique qualified name, or path of the Switch Group.
    #[serde(rename = "switchGroup")]
    pub switch_group: ObjectRef,
}

impl SoundengineGetSwitchArgs {
    /// Creates a new [`SoundengineGetSwitchArgs`] with the required fields.
    pub fn new(switch_group: ObjectRef) -> Self {
        Self {
            switch_group,
            game_object: None,
        }
    }

    /// The unique ID of the game object. When not specified, `AK_INVALID_GAME_OBJECT` is used.
    pub fn game_object(mut self, game_object: impl Into<GameObjectRef>) -> Self {
        self.game_object = Some(game_object.into());
        self
    }
}

/// Options for `ak.soundengine.getSwitch`.
///
/// Specifies the return options for every object returned.
pub type SoundengineGetSwitchOptions = ObjectReturnOptions;

/// Response from `ak.soundengine.getSwitch`.
///
/// Gets the current state of a Switch Group for a given Game Object.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundengineGetSwitchResponse {
    /// A Wwise object. Use the return options to specify which property is required.
    #[serde(rename = "return")]
    pub switch: WwiseObject,
}
