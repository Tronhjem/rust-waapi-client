// Generated bindings for topic ak.wwise.core.soundbank.generated

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.soundbank.generated`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSoundbankGeneratedPayload {
    /// The platform for which the SoundBank was generated.
    pub platform: serde_json::Value,
    /// The generated SoundBank. Use the return options to obtain specific details.
    pub soundbank: WwiseObject,
    /// The language for which the SoundBank was generated. Only present when generating a SoundBank for a specific language.
    #[serde(default)]
    pub language: Option<serde_json::Value>,
    /// SoundBank data object containing the actual data encoded in base64 and the size.
    #[serde(rename = "bankData", default)]
    pub bank_data: Option<serde_json::Value>,
    /// PluginInfo file info
    #[serde(rename = "pluginInfo", default)]
    pub plugin_info: Option<serde_json::Value>,
    /// The error message, if an error occurred. Only present if an error occurred.
    #[serde(default)]
    pub error: Option<String>,
    /// All the information for SoundBank.
    #[serde(rename = "bankInfo", default)]
    pub bank_info: Vec<serde_json::Value>,
}

/// Subscription options for `ak.wwise.core.soundbank.generated`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreSoundbankGeneratedOptions {
    /// Data to be returned for the SoundBank generated. Refer to for more information on the properties available.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// Embed the JSON SoundBank info file in the response.
    #[serde(rename = "infoFile", skip_serializing_if = "Option::is_none")]
    pub info_file: Option<bool>,
    /// Embed the base64 SoundBank data in the response.
    #[serde(rename = "bankData", skip_serializing_if = "Option::is_none")]
    pub bank_data: Option<bool>,
    /// Embed the JSON PluginInfo file in the response.
    #[serde(rename = "pluginInfo", skip_serializing_if = "Option::is_none")]
    pub plugin_info: Option<bool>,
}

impl CoreSoundbankGeneratedOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_(mut self, return_: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }

    pub fn info_file(mut self, info_file: impl Into<bool>) -> Self {
        self.info_file = Some(info_file.into());
        self
    }

    pub fn bank_data(mut self, bank_data: impl Into<bool>) -> Self {
        self.bank_data = Some(bank_data.into());
        self
    }

    pub fn plugin_info(mut self, plugin_info: impl Into<bool>) -> Self {
        self.plugin_info = Some(plugin_info.into());
        self
    }
}
