// Generated bindings for ak.soundengine.seekOnEvent



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.soundengine.seekOnEvent`.
///
/// Seeks inside all playing objects that are referenced in Play Actions of the specified Event. See `AK::SoundEngine::SeekOnEvent`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSeekOnEventArgs {
    /// Desired position where playback should restart, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Associated game object ID; use `AK_INVALID_GAME_OBJECT` to affect all game objects. When not specified, `AK_INVALID_GAME_OBJECT` is used.
    #[serde(rename = "gameObject")]
    pub game_object: GameObjectRef,
    /// Either the ID (GUID), name, or Short ID of the Event.
    pub event: SoundEngineObjectRef,
    /// Specifies the playing ID for which the seek is to be applied. This results in the seek being applied only to active actions of the playing ID. Use `AK_INVALID_PLAYING_ID` or nothing, to seek on all active Actions of this Event ID.
    #[serde(rename = "playingId")]
    pub playing_id: u32,
    /// Desired position where playback should restart, expressed in a percentage of the file's total duration, between 0 and 1.f. See note above about infinite looping sounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<f64>,
    /// If true, the final seeking position is made equal to the nearest marker.
    #[serde(rename = "seekToNearestMarker")]
    pub seek_to_nearest_marker: bool,
}

impl SoundengineSeekOnEventArgs {
    /// Creates a new [`SoundengineSeekOnEventArgs`] with the required fields.
    pub fn new(event: SoundEngineObjectRef, game_object: GameObjectRef, seek_to_nearest_marker: bool, playing_id: u32) -> Self {
        Self {
            event,
            game_object,
            seek_to_nearest_marker,
            playing_id,
            position: None,
            percent: None,
        }
    }

    /// Desired position where playback should restart, in milliseconds.
    pub fn position(mut self, position: impl Into<i32>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Desired position where playback should restart, expressed in a percentage of the file's total duration, between 0 and 1.f. See note above about infinite looping sounds.
    pub fn percent(mut self, percent: impl Into<f64>) -> Self {
        self.percent = Some(percent.into());
        self
    }
}
