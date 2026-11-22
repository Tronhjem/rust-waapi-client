// Generated bindings for ak.soundengine.executeActionOnEvent

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.executeActionOnEvent`.
///
/// Executes an action on all nodes that are referenced in the specified event in a Play action. See `AK::SoundEngine::ExecuteActionOnEvent`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineExecuteActionOnEventArgs {
    /// Associated game object ID.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// Either the ID (GUID), name or Short ID of the event.
    pub event: SoundEngineObjectRef,
    /// Action to execute on all the elements that were played using the specified event. Uses values from `AK::SoundEngine::AkActionOnEventType`.
    #[serde(rename = "actionType")]
    pub action_type: ActionOnEventType,
    /// Fade duration (ms).
    #[serde(rename = "transitionDuration")]
    pub transition_duration: i32,
    /// Use values from `AkCurveInterpolation`.
    #[serde(rename = "fadeCurve")]
    pub fade_curve: AkCurveInterpolation,
}

impl SoundengineExecuteActionOnEventArgs {
    /// Creates a new [`SoundengineExecuteActionOnEventArgs`] with the required fields.
    pub fn new(
        event: impl Into<SoundEngineObjectRef>,
        action_type: ActionOnEventType,
        game_object: impl Into<GameObjectRef>,
        transition_duration: i32,
        fade_curve: AkCurveInterpolation,
    ) -> Self {
        Self {
            event: event.into(),
            action_type,
            game_object: game_object.into(),
            transition_duration,
            fade_curve,
        }
    }
}

/// Module-scoped alias for [`SoundengineExecuteActionOnEventArgs`].
pub type ExecuteActionOnEventArgs = SoundengineExecuteActionOnEventArgs;
