// Generated bindings for ak.wwise.core.audio.import

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.wwise.core.audio.import`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreAudioImportArgs {
    /// How imported objects are created.
    #[serde(rename = "importOperation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_operation: Option<ImportOperation>,
    /// Array of import commands.
    pub imports: Vec<ImportItem>,
    /// Default values for each import item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ImportDefaults>,
    /// Automatically add imported files to source control.
    #[serde(rename = "autoAddToSourceControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_to_source_control: Option<bool>,
    /// Automatically check out modified files.
    #[serde(rename = "autoCheckOutToSourceControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_check_out_to_source_control: Option<bool>,
}

impl WwiseCoreAudioImportArgs {
    pub fn new(imports: Vec<ImportItem>) -> Self {
        Self {
            imports,
            import_operation: None,
            default: None,
            auto_add_to_source_control: None,
            auto_check_out_to_source_control: None,
        }
    }

    pub fn import_operation(mut self, import_operation: impl Into<ImportOperation>) -> Self {
        self.import_operation = Some(import_operation.into());
        self
    }

    pub fn default(mut self, default: impl Into<ImportDefaults>) -> Self {
        self.default = Some(default.into());
        self
    }

    pub fn auto_add_to_source_control(mut self, auto_add_to_source_control: impl Into<bool>) -> Self {
        self.auto_add_to_source_control = Some(auto_add_to_source_control.into());
        self
    }

    pub fn auto_check_out_to_source_control(mut self, auto_check_out_to_source_control: impl Into<bool>) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }
}

/// Options for `ak.wwise.core.audio.import`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreAudioImportOptions {
    /// Fields to return for each imported object.
    #[serde(rename = "return")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<String>>,
    /// Platform for return values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Language for return values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageRef>,
}

impl WwiseCoreAudioImportOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_fields(mut self, return_fields: impl Into<Vec<String>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn language(mut self, language: impl Into<LanguageRef>) -> Self {
        self.language = Some(language.into());
        self
    }
}

/// Response from `ak.wwise.core.audio.import`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreAudioImportResponse {
    /// Imported original file paths.
    pub files: Vec<String>,
    /// Audio import log.
    pub log: Vec<ImportLogEntry>,
    /// Imported objects.
    pub objects: Vec<WwiseObject>,
}
