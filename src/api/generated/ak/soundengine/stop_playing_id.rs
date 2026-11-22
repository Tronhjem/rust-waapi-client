// Generated bindings for ak.soundengine.stopPlayingID

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.stopPlayingID`.
///
/// Stops the current content, associated to the specified playing ID, from playing. See `AK::SoundEngine::StopPlayingID`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineStopPlayingIdArgs {
    /// Fade duration (ms).
    #[serde(rename = "transitionDuration")]
    pub transition_duration: i32,
    /// Curve type to be used for the transition. Uses values from `AkCurveInterpolation`.
    #[serde(rename = "fadeCurve")]
    pub fade_curve: AkCurveInterpolation,
    /// Playing ID to be stopped.
    #[serde(rename = "playingId")]
    pub playing_id: u32,
}

impl SoundengineStopPlayingIdArgs {
    /// Creates a new [`SoundengineStopPlayingIdArgs`] with the required fields.
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

impl<T0: Into<u32>, T1: Into<i32>, T2: Into<AkCurveInterpolation>> From<(T0, T1, T2)>
    for SoundengineStopPlayingIdArgs
{
    fn from((playing_id, transition_duration, fade_curve): (T0, T1, T2)) -> Self {
        Self::new(
            playing_id.into(),
            transition_duration.into(),
            fade_curve.into(),
        )
    }
}

/// Module-scoped alias for [`SoundengineStopPlayingIdArgs`].
pub type StopPlayingIdArgs = SoundengineStopPlayingIdArgs;
