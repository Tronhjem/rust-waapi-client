// Generated bindings for ak.soundengine.executeActionOnEvent

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.executeActionOnEvent`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineExecuteActionOnEventArgs {
    /// Associated game object ID.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// Either the ID (GUID), name or Short ID of the event.
    pub event: SoundEngineObjectRef,
    /// Action to execute on all the elements that were played using the specified event. Uses values from <tt>AK::SoundEngine::AkActionOnEventType</tt>.
    #[serde(rename = "actionType")]
    pub action_type: ActionOnEventType,
    /// Fade duration (ms).
    #[serde(rename = "transitionDuration")]
    pub transition_duration: i32,
    /// Use values from .
    #[serde(rename = "fadeCurve")]
    pub fade_curve: AkCurveInterpolation,
}

impl SoundengineExecuteActionOnEventArgs {
    pub fn new(
        event: SoundEngineObjectRef,
        action_type: ActionOnEventType,
        game_object: GameObjectRef,
        transition_duration: i32,
        fade_curve: AkCurveInterpolation,
    ) -> Self {
        Self {
            event,
            action_type,
            game_object,
            transition_duration,
            fade_curve,
        }
    }
}
