// Generated bindings for topic ak.wwise.core.soundbank.generated

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.soundbank.generated`.
///
/// Sent when a single SoundBank is generated. This could be sent multiple times during SoundBank generation, for every SoundBank generated and for every platform. To generate SoundBanks, see `ak.wwise.core.soundbank.generate` or `ak.wwise.ui.commands.execute` with one of the SoundBank generation commands. See the Wwise global command IDs reference for the list of commands.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSoundbankGeneratedPayload {
    /// The platform for which the SoundBank was generated.
    pub platform: serde_json::Value,
    /// The generated SoundBank. Use the return options to obtain specific details.
    pub soundbank: WwiseObject,
    /// The language for which the SoundBank was generated. Only present when generating a SoundBank for a specific language.
    #[serde(default)]
    pub language: Option<serde_json::Value>,
    /// The error message, if an error occurred. Only present if an error occurred.
    #[serde(default)]
    pub error: Option<String>,
    /// SoundBank data object containing the actual data encoded in base64 and the size.
    #[serde(rename = "bankData", default)]
    pub bank_data: Option<serde_json::Value>,
    /// PluginInfo file info.
    #[serde(rename = "pluginInfo", default)]
    pub plugin_info: Option<serde_json::Value>,
    /// All the information for SoundBank.
    #[serde(rename = "bankInfo", default)]
    pub bank_info: Vec<serde_json::Value>,
}

/// Subscription options for `ak.wwise.core.soundbank.generated`.
///
/// Sent when a single SoundBank is generated. This could be sent multiple times during SoundBank generation, for every SoundBank generated and for every platform. To generate SoundBanks, see `ak.wwise.core.soundbank.generate` or `ak.wwise.ui.commands.execute` with one of the SoundBank generation commands. See the Wwise global command IDs reference for the list of commands.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreSoundbankGeneratedOptions {
    /// Data to be returned for the SoundBank generated. See the Wwise Objects Reference for more information on the properties available.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
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
    /// Creates a new [`CoreSoundbankGeneratedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for the SoundBank generated. See the Wwise Objects Reference for more information on the properties available.
    pub fn return_fields(mut self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_(self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields(return_fields)
    }

    /// Embed the JSON SoundBank info file in the response.
    pub fn info_file(mut self, info_file: bool) -> Self {
        self.info_file = Some(info_file);
        self
    }

    /// Embed the base64 SoundBank data in the response.
    pub fn bank_data(mut self, bank_data: bool) -> Self {
        self.bank_data = Some(bank_data);
        self
    }

    /// Embed the JSON PluginInfo file in the response.
    pub fn plugin_info(mut self, plugin_info: bool) -> Self {
        self.plugin_info = Some(plugin_info);
        self
    }
}
