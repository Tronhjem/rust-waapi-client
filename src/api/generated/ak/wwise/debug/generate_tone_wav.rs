// Generated bindings for ak.wwise.debug.generateToneWAV

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.debug.generateToneWAV`.
///
/// Generate a WAV file playing a tone with a simple envelope and save it to the specified location. This is provided as a utility to generate test WAV files.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DebugGenerateToneWavArgs {
    /// File path to write the generated WAV file. This path normally uses the file extension WAV.
    pub path: String,
    /// Channel configuration of the audio file. Default is 1.0. Please use the silence waveform for the ambisonics configurations, as the other tone signals will be incompatible with the format.
    #[serde(rename = "channelConfig", skip_serializing_if = "Option::is_none")]
    pub channel_config: Option<String>,
    /// Bit depth of the audio file. Default is int16.
    #[serde(rename = "bitDepth", skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<String>,
    /// Sample rate of the audio file. Default is 48000.
    #[serde(rename = "sampleRate", skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
    /// Sets the channel configuration type to anonymous, this overrides the channel config but keeps the number of channels specified. When not specified, the default is false.
    #[serde(rename = "setAnonymous", skip_serializing_if = "Option::is_none")]
    pub set_anonymous: Option<bool>,
    /// Number of seconds of the signal holds at the specified level. Default is 1 second.
    #[serde(rename = "sustainTime", skip_serializing_if = "Option::is_none")]
    pub sustain_time: Option<f64>,
    /// Decibel attenuation for the sustained signal. Default is 0 dB.
    #[serde(rename = "sustainLevel", skip_serializing_if = "Option::is_none")]
    pub sustain_level: Option<f64>,
    /// Number of seconds for the signal to reach sustain level. Default is 0 second.
    #[serde(rename = "attackTime", skip_serializing_if = "Option::is_none")]
    pub attack_time: Option<f64>,
    /// Number of seconds for the signal to release from sustain level. Default is 0 second.
    #[serde(rename = "releaseTime", skip_serializing_if = "Option::is_none")]
    pub release_time: Option<f64>,
    /// Waveform type. Default is silence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,
    /// Specifies which channels receive waveform data. The channel mask is a bit field where each bit represents a channel. The least significant bit represents the channel at index zero. The bits with 0 are silent. When not specified, the default outputs waveform data on all channels.
    #[serde(
        rename = "waveformChannelMask",
        skip_serializing_if = "Option::is_none"
    )]
    pub waveform_channel_mask: Option<i64>,
    /// Waveform frequency. Default is 440.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<f64>,
    /// A list of markers to add to the generated WAV file. Each marker is an object with 'label' and 'position' properties, where 'position' is the position in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<serde_json::Value>>,
}

impl DebugGenerateToneWavArgs {
    /// Creates a new [`DebugGenerateToneWavArgs`] with the required fields.
    pub fn new(path: String) -> Self {
        Self {
            path,
            channel_config: None,
            bit_depth: None,
            sample_rate: None,
            set_anonymous: None,
            sustain_time: None,
            sustain_level: None,
            attack_time: None,
            release_time: None,
            waveform: None,
            waveform_channel_mask: None,
            frequency: None,
            markers: None,
        }
    }

    /// Channel configuration of the audio file. Default is 1.0. Please use the silence waveform for the ambisonics configurations, as the other tone signals will be incompatible with the format.
    pub fn channel_config(mut self, channel_config: impl Into<String>) -> Self {
        self.channel_config = Some(channel_config.into());
        self
    }

    /// Bit depth of the audio file. Default is int16.
    pub fn bit_depth(mut self, bit_depth: impl Into<String>) -> Self {
        self.bit_depth = Some(bit_depth.into());
        self
    }

    /// Sample rate of the audio file. Default is 48000.
    pub fn sample_rate(mut self, sample_rate: impl Into<f64>) -> Self {
        self.sample_rate = Some(sample_rate.into());
        self
    }

    /// Sets the channel configuration type to anonymous, this overrides the channel config but keeps the number of channels specified. When not specified, the default is false.
    pub fn set_anonymous(mut self, set_anonymous: impl Into<bool>) -> Self {
        self.set_anonymous = Some(set_anonymous.into());
        self
    }

    /// Number of seconds of the signal holds at the specified level. Default is 1 second.
    pub fn sustain_time(mut self, sustain_time: impl Into<f64>) -> Self {
        self.sustain_time = Some(sustain_time.into());
        self
    }

    /// Decibel attenuation for the sustained signal. Default is 0 dB.
    pub fn sustain_level(mut self, sustain_level: impl Into<f64>) -> Self {
        self.sustain_level = Some(sustain_level.into());
        self
    }

    /// Number of seconds for the signal to reach sustain level. Default is 0 second.
    pub fn attack_time(mut self, attack_time: impl Into<f64>) -> Self {
        self.attack_time = Some(attack_time.into());
        self
    }

    /// Number of seconds for the signal to release from sustain level. Default is 0 second.
    pub fn release_time(mut self, release_time: impl Into<f64>) -> Self {
        self.release_time = Some(release_time.into());
        self
    }

    /// Waveform type. Default is silence.
    pub fn waveform(mut self, waveform: impl Into<String>) -> Self {
        self.waveform = Some(waveform.into());
        self
    }

    /// Specifies which channels receive waveform data. The channel mask is a bit field where each bit represents a channel. The least significant bit represents the channel at index zero. The bits with 0 are silent. When not specified, the default outputs waveform data on all channels.
    pub fn waveform_channel_mask(mut self, waveform_channel_mask: impl Into<i64>) -> Self {
        self.waveform_channel_mask = Some(waveform_channel_mask.into());
        self
    }

    /// Waveform frequency. Default is 440.
    pub fn frequency(mut self, frequency: impl Into<f64>) -> Self {
        self.frequency = Some(frequency.into());
        self
    }

    /// A list of markers to add to the generated WAV file. Each marker is an object with 'label' and 'position' properties, where 'position' is the position in seconds.
    pub fn markers(mut self, markers: impl Into<Vec<serde_json::Value>>) -> Self {
        self.markers = Some(markers.into());
        self
    }
}
