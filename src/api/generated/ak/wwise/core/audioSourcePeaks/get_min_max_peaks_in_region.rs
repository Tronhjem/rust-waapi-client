// Generated bindings for ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion`.
///
/// Gets the min/max peak pairs, in the given region of an audio source, as a collection of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, only one binary string represents the peaks across all channels globally.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioSourcePeaksGetMinMaxPeaksInRegionArgs {
    /// The ID (GUID), name, or path of the audio source object.
    pub object: ObjectRef,
    /// The start time, in seconds, of the section of the audio source for which peaks are required. This number must be smaller than timeTo.
    #[serde(rename = "timeFrom")]
    pub time_from: f64,
    /// The end time, in seconds, of the section of the audio source for which peaks are required. This number must be larger than timeFrom.
    #[serde(rename = "timeTo")]
    pub time_to: f64,
    /// The number of peaks that are required (minimum 1).
    #[serde(rename = "numPeaks")]
    pub num_peaks: i64,
    /// When true, peaks are calculated globally across channels, instead of per channel. The default value is false.
    #[serde(
        rename = "getCrossChannelPeaks",
        skip_serializing_if = "Option::is_none"
    )]
    pub get_cross_channel_peaks: Option<bool>,
}

impl CoreAudioSourcePeaksGetMinMaxPeaksInRegionArgs {
    /// Creates a new [`CoreAudioSourcePeaksGetMinMaxPeaksInRegionArgs`] with the required fields.
    pub fn new(object: ObjectRef, time_from: f64, time_to: f64, num_peaks: i64) -> Self {
        Self {
            object,
            time_from,
            time_to,
            num_peaks,
            get_cross_channel_peaks: None,
        }
    }

    /// When true, peaks are calculated globally across channels, instead of per channel. The default value is false.
    pub fn get_cross_channel_peaks(mut self, get_cross_channel_peaks: impl Into<bool>) -> Self {
        self.get_cross_channel_peaks = Some(get_cross_channel_peaks.into());
        self
    }
}

/// Response from `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion`.
///
/// Gets the min/max peak pairs, in the given region of an audio source, as a collection of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, only one binary string represents the peaks across all channels globally.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreAudioSourcePeaksGetMinMaxPeaksInRegionResponse {
    /// The number of peaks in the returned min max channel arrays. If peaksArrayLength is less than the numPeaks input argument, this indicates fewer samples were available than numPeaks requested. In this case, the arrays contain all of the sample values within the requested time window.
    #[serde(rename = "peaksArrayLength", default)]
    pub peaks_array_length: Option<f64>,
    /// The size of the data in the peaks min max arrays. This can be used (in conjunction with peaksArrayLength) to decode the peaksBinaryStrings to 16 bit integer arrays.
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
