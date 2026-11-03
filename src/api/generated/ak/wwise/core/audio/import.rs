// Generated bindings for ak.wwise.core.audio.import

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.import`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreAudioImportArgs {
    /// Determines if Wwise automatically performs a source control Add operation on the imported files
    /// Defaults to true.
    #[serde(
        rename = "autoAddToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_add_to_source_control: Option<bool>,
    /// Determines if Wwise automatically performs a source control Checkout operation (when applicable) on the modified files
    /// Defaults to true.
    #[serde(
        rename = "autoCheckOutToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_check_out_to_source_control: Option<bool>,
    #[serde(rename = "importOperation", skip_serializing_if = "Option::is_none")]
    pub import_operation: Option<ImportOperation>,
    /// Array of import commands.
    pub imports: Vec<ImportItem>,
    /// Default values for each item in "imports"
    /// Use this object to avoid repeating common properties of every imported element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ImportDefaults>,
}

impl WwiseCoreAudioImportArgs {
    pub fn new(imports: Vec<ImportItem>) -> Self {
        Self {
            imports,
            auto_add_to_source_control: None,
            auto_check_out_to_source_control: None,
            import_operation: None,
            default: None,
        }
    }

    pub fn auto_add_to_source_control(
        mut self,
        auto_add_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_add_to_source_control = Some(auto_add_to_source_control.into());
        self
    }

    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }

    pub fn import_operation(mut self, import_operation: impl Into<ImportOperation>) -> Self {
        self.import_operation = Some(import_operation.into());
        self
    }

    pub fn default(mut self, default: impl Into<ImportDefaults>) -> Self {
        self.default = Some(default.into());
        self
    }
}

/// Options for `ak.wwise.core.audio.import`.

pub type WwiseCoreAudioImportOptions = ObjectReturnOptions;

/// Response from `ak.wwise.core.audio.import`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreAudioImportResponse {
    /// The list of imported original files.
    pub files: Vec<String>,
    /// The audio import log.
    pub log: Vec<serde_json::Value>,
    /// Array of objects.
    pub objects: Vec<WwiseObject>,
}
