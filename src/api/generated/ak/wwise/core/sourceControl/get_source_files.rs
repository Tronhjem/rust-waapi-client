// Generated bindings for ak.wwise.core.sourceControl.getSourceFiles

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.getSourceFiles`.
///
/// Retrieve all original files.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreSourceControlGetSourceFilesArgs {
    /// Filter the files are in the search result.
    /// - all: Displays all files in the Originals folder (default).
    /// - used: Only displays files that are used in the project.
    /// - unused: Only displays files that are not used in the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Base folder for search relative to Originals folder. Default to the Originals folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// Search in all subfolders of the base folder. Default to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

impl CoreSourceControlGetSourceFilesArgs {
    /// Creates a new [`CoreSourceControlGetSourceFilesArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter the files are in the search result.
    /// - all: Displays all files in the Originals folder (default).
    /// - used: Only displays files that are used in the project.
    /// - unused: Only displays files that are not used in the project.
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Base folder for search relative to Originals folder. Default to the Originals folder.
    pub fn folder(mut self, folder: impl Into<String>) -> Self {
        self.folder = Some(folder.into());
        self
    }

    /// Search in all subfolders of the base folder. Default to true.
    pub fn recursive(mut self, recursive: impl Into<bool>) -> Self {
        self.recursive = Some(recursive.into());
        self
    }
}

/// Options for `ak.wwise.core.sourceControl.getSourceFiles`.
///
/// Retrieve all original files.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreSourceControlGetSourceFilesOptions {
    /// Array of fields to return for each file or folder found.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<String>>,
    /// The array of return expressions defines which elements of the Wwise object is returned. This can include built-in accessors, such as the name or id, or object properties, such as the Volume or the Pitch.
    #[serde(rename = "objectReturn", skip_serializing_if = "Option::is_none")]
    pub object_return: Option<Vec<ObjectReturnField>>,
}

impl CoreSourceControlGetSourceFilesOptions {
    /// Creates a new [`CoreSourceControlGetSourceFilesOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Array of fields to return for each file or folder found.
    pub fn return_fields<I, T>(mut self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.return_fields = Some(return_fields.into_iter().map(Into::into).collect());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_<I, T>(self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.return_fields(return_fields)
    }

    /// The array of return expressions defines which elements of the Wwise object is returned. This can include built-in accessors, such as the name or id, or object properties, such as the Volume or the Pitch.
    pub fn object_return(mut self, object_return: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.object_return = Some(object_return.into());
        self
    }
}

impl From<()> for CoreSourceControlGetSourceFilesOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[String; N]> for CoreSourceControlGetSourceFilesOptions {
    fn from(return_fields: [String; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<String>> for CoreSourceControlGetSourceFilesOptions {
    fn from(return_fields: Vec<String>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Response from `ak.wwise.core.sourceControl.getSourceFiles`.
///
/// Retrieve all original files.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlGetSourceFilesResponse {
    /// Files and folders found.
    #[serde(rename = "return")]
    pub return_: Vec<serde_json::Value>,
}

/// Module-scoped alias for [`CoreSourceControlGetSourceFilesArgs`].
pub type GetSourceFilesArgs = CoreSourceControlGetSourceFilesArgs;

/// Module-scoped alias for [`CoreSourceControlGetSourceFilesOptions`].
pub type GetSourceFilesOptions = CoreSourceControlGetSourceFilesOptions;

/// Module-scoped alias for [`CoreSourceControlGetSourceFilesResponse`].
pub type GetSourceFilesResponse = CoreSourceControlGetSourceFilesResponse;
