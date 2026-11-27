// Generated bindings for ak.wwise.core.audio.import

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.import`.
///
/// Creates Wwise objects and imports audio files. This function does not return an error when something fails during the import process, please see the log for the result of each import command. This function uses the same importation processor available through the Tab Delimited import in the Audio File Importer. The function returns an array of all objects created, replaced or re-used. Use the options to specify how the objects are returned. For more information, see the WAAPI Import documentation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioImportArgs {
    /// Determines if Wwise automatically performs a source control Add operation on the imported files. Defaults to true.
    #[serde(
        rename = "autoAddToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_add_to_source_control: Option<bool>,
    /// Determines if Wwise automatically performs a source control Checkout operation (when applicable) on the modified files. Defaults to true.
    #[serde(
        rename = "autoCheckOutToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_check_out_to_source_control: Option<bool>,
    /// Determines how imported objects are created.
    #[serde(rename = "importOperation", skip_serializing_if = "Option::is_none")]
    pub import_operation: Option<ImportOperation>,
    /// Array of import commands.
    pub imports: Vec<ImportItem>,
    /// Default values for each item in "imports". Use this object to avoid repeating common properties of every imported element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ImportDefaults>,
}

impl CoreAudioImportArgs {
    /// Creates a new [`CoreAudioImportArgs`] with the required fields.
    pub fn new(imports: impl Into<Vec<ImportItem>>) -> Self {
        Self {
            imports: imports.into(),
            auto_add_to_source_control: None,
            auto_check_out_to_source_control: None,
            import_operation: None,
            default: None,
        }
    }

    /// Determines if Wwise automatically performs a source control Add operation on the imported files. Defaults to true.
    pub fn auto_add_to_source_control(mut self, auto_add_to_source_control: bool) -> Self {
        self.auto_add_to_source_control = Some(auto_add_to_source_control);
        self
    }

    /// Determines if Wwise automatically performs a source control Checkout operation (when applicable) on the modified files. Defaults to true.
    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: bool,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control);
        self
    }

    /// Determines how imported objects are created.
    pub fn import_operation(mut self, import_operation: ImportOperation) -> Self {
        self.import_operation = Some(import_operation);
        self
    }

    /// Default values for each item in "imports". Use this object to avoid repeating common properties of every imported element.
    pub fn default(mut self, default: ImportDefaults) -> Self {
        self.default = Some(default);
        self
    }
}

/// Options for `ak.wwise.core.audio.import`.
///
/// Specifies the return options for every object returned.
pub type CoreAudioImportOptions = ObjectReturnOptions;

/// Response from `ak.wwise.core.audio.import`.
///
/// Creates Wwise objects and imports audio files. This function does not return an error when something fails during the import process, please see the log for the result of each import command. This function uses the same importation processor available through the Tab Delimited import in the Audio File Importer. The function returns an array of all objects created, replaced or re-used. Use the options to specify how the objects are returned. For more information, see the WAAPI Import documentation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreAudioImportResponse {
    /// The list of imported original files.
    #[serde(default)]
    pub files: Vec<String>,
    /// The audio import log.
    #[serde(default)]
    pub log: Vec<AudioImportLogItem>,
    /// Array of objects.
    #[serde(default)]
    pub objects: Vec<WwiseObject>,
}
