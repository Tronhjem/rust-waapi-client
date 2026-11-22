// Generated bindings for ak.soundengine.setListenerSpatialization



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.soundengine.setListenerSpatialization`.
///
/// Sets a listener's spatialization parameters. This lets you define listener-specific volume offsets for each audio channel. See `AK::SoundEngine::SetListenerSpatialization`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetListenerSpatializationArgs {
    /// Channel configuration associated with volumeOffsets. Use `AK::AkChannelConfig::Serialize` to serialize the value.
    #[serde(rename = "channelConfig")]
    pub channel_config: u32,
    /// Listener game object ID.
    pub listener: GameObjectRef,
    /// Spatialization toggle (true: enable spatialization, false: disable spatialization).
    pub spatialized: bool,
    /// Array of per-speaker volume offsets, in dB. See AkSpeakerVolumes.h for how to manipulate this array.
    #[serde(rename = "volumeOffsets")]
    pub volume_offsets: Vec<f64>,
}

impl SoundengineSetListenerSpatializationArgs {
    /// Creates a new [`SoundengineSetListenerSpatializationArgs`] with the required fields.
    pub fn new(listener: GameObjectRef, spatialized: bool, channel_config: u32, volume_offsets: Vec<f64>) -> Self {
        Self {
            listener,
            spatialized,
            channel_config,
            volume_offsets,
        }
    }
}
