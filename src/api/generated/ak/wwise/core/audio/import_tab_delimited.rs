// Generated bindings for ak.wwise.core.audio.importTabDelimited

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.importTabDelimited`.
///
/// Scripted object creation and audio file import from a tab-delimited file.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioImportTabDelimitedArgs {
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
    /// Imports language for audio file import (taken from the project's defined languages, found in the WPROJ file LanguageList).
    #[serde(rename = "importLanguage")]
    pub import_language: String,
    /// Determines how imported objects are created. For each object created:
    /// - createNew: creates a new object; the object is given the desired name if possible, otherwise a new unique name is used.
    /// - useExisting: uses the object if it exists, updating the specified properties; otherwise it creates a new object. This is the default value.
    /// - replaceExisting: creates a new object. If an existing object has the same name, the existing object is destroyed.
    #[serde(rename = "importOperation")]
    pub import_operation: String,
    /// Object ID (GUID), name, or path used as root relative object paths.
    #[serde(rename = "importLocation", skip_serializing_if = "Option::is_none")]
    pub import_location: Option<ObjectRef>,
    /// Location of tab-delimited import file. For using WAAPI on Mac, please see the WAAPI Mac path documentation.
    #[serde(rename = "importFile")]
    pub import_file: String,
}

impl CoreAudioImportTabDelimitedArgs {
    /// Creates a new [`CoreAudioImportTabDelimitedArgs`] with the required fields.
    pub fn new(
        import_language: impl Into<String>,
        import_operation: impl Into<String>,
        import_file: impl Into<String>,
    ) -> Self {
        Self {
            import_language: import_language.into(),
            import_operation: import_operation.into(),
            import_file: import_file.into(),
            auto_add_to_source_control: None,
            auto_check_out_to_source_control: None,
            import_location: None,
        }
    }

    /// Determines if Wwise automatically performs a source control Add operation on the imported files. Defaults to true.
    pub fn auto_add_to_source_control(
        mut self,
        auto_add_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_add_to_source_control = Some(auto_add_to_source_control.into());
        self
    }

    /// Determines if Wwise automatically performs a source control Checkout operation (when applicable) on the modified files. Defaults to true.
    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }

    /// Object ID (GUID), name, or path used as root relative object paths.
    pub fn import_location(mut self, import_location: impl Into<ObjectRef>) -> Self {
        self.import_location = Some(import_location.into());
        self
    }
}

impl<T0: Into<String>, T1: Into<String>, T2: Into<String>> From<(T0, T1, T2)>
    for CoreAudioImportTabDelimitedArgs
{
    fn from((import_language, import_operation, import_file): (T0, T1, T2)) -> Self {
        Self::new(import_language, import_operation, import_file)
    }
}

/// Options for `ak.wwise.core.audio.importTabDelimited`.
///
/// Specifies the return options for every object returned.
pub type CoreAudioImportTabDelimitedOptions = ObjectReturnOptions;

/// Response from `ak.wwise.core.audio.importTabDelimited`.
///
/// Scripted object creation and audio file import from a tab-delimited file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreAudioImportTabDelimitedResponse {
    /// Array of objects.
    #[serde(default)]
    pub objects: Vec<WwiseObject>,
}

/// Module-scoped alias for [`CoreAudioImportTabDelimitedArgs`].
pub type ImportTabDelimitedArgs = CoreAudioImportTabDelimitedArgs;

/// Module-scoped alias for [`CoreAudioImportTabDelimitedOptions`].
pub type ImportTabDelimitedOptions = CoreAudioImportTabDelimitedOptions;

/// Module-scoped alias for [`CoreAudioImportTabDelimitedResponse`].
pub type ImportTabDelimitedResponse = CoreAudioImportTabDelimitedResponse;
