// Generated bindings for ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion`.
///
/// Gets the min/max peak pairs in the entire trimmed region of an audio source, for each channel, as an array of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, there is only one binary string representing peaks across all channels globally.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioSourcePeaksGetMinMaxPeaksInTrimmedRegionArgs {
    /// The ID (GUID), name, or path of the audio source.
    pub object: ObjectRef,
    /// The number of peaks that are required (minimum 1).
    #[serde(rename = "numPeaks")]
    pub num_peaks: i64,
    /// When true, peaks are calculated globally across channels, instead of per channel. When not specified, the default is false.
    #[serde(
        rename = "getCrossChannelPeaks",
        skip_serializing_if = "Option::is_none"
    )]
    pub get_cross_channel_peaks: Option<bool>,
}

impl CoreAudioSourcePeaksGetMinMaxPeaksInTrimmedRegionArgs {
    /// Creates a new [`CoreAudioSourcePeaksGetMinMaxPeaksInTrimmedRegionArgs`] with the required fields.
    pub fn new(object: ObjectRef, num_peaks: i64) -> Self {
        Self {
            object,
            num_peaks,
            get_cross_channel_peaks: None,
        }
    }

    /// When true, peaks are calculated globally across channels, instead of per channel. When not specified, the default is false.
    pub fn get_cross_channel_peaks(mut self, get_cross_channel_peaks: impl Into<bool>) -> Self {
        self.get_cross_channel_peaks = Some(get_cross_channel_peaks.into());
        self
    }
}

/// Response from `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion`.
///
/// Gets the min/max peak pairs in the entire trimmed region of an audio source, for each channel, as an array of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, there is only one binary string representing peaks across all channels globally.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreAudioSourcePeaksGetMinMaxPeaksInTrimmedRegionResponse {
    /// The number of peaks in the returned min max channel arrays. If peaksArrayLength is less than the numPeaks input argument, this indicates fewer samples were available than numPeaks requested. In this case, the arrays contain all of the sample values within the requested time window.
    #[serde(rename = "peaksArrayLength", default)]
    pub peaks_array_length: Option<f64>,
    /// The size of the data in the min/max peaks arrays. This can be used (in conjunction with peaksArrayLength) to decode the peaksBinaryStrings to 16 bit integer arrays.
    #[serde(rename = "peaksDataSize", default)]
    pub peaks_data_size: Option<f64>,
    /// An array of binary strings.
    #[serde(rename = "peaksBinaryStrings", default)]
    pub peaks_binary_strings: Vec<String>,
    /// The number of channels of peak data (i.e. the number of strings in peaksBinaryStrings). This is 1 if getCrossChannelPeaks is true.
    #[serde(rename = "numChannels", default)]
    pub num_channels: Option<f64>,
    /// The maximum value that a peak can take. This can be used to normalize the peaks in the array (between -1, 1), by dividing each by this number.
    #[serde(rename = "maxAbsValue", default)]
    pub max_abs_value: Option<f64>,
    /// The channel configuration.
    #[serde(rename = "channelConfig", default)]
    pub channel_config: Option<String>,
}
