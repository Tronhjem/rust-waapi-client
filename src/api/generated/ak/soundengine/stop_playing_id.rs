// Generated bindings for ak.soundengine.stopPlayingID

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.stopPlayingID`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineStopPlayingIdArgs {
    /// Fade duration (ms).
    #[serde(rename = "transitionDuration")]
    pub transition_duration: i32,
    /// Curve type to be used for the transition. Uses values from .
    #[serde(rename = "fadeCurve")]
    pub fade_curve: AkCurveInterpolation,
    /// Playing ID to be stopped.
    #[serde(rename = "playingId")]
    pub playing_id: u32,
}

impl SoundengineStopPlayingIdArgs {
    pub fn new(
        playing_id: u32,
        transition_duration: i32,
        fade_curve: AkCurveInterpolation,
    ) -> Self {
        Self {
            playing_id,
            transition_duration,
            fade_curve,
        }
    }
}
