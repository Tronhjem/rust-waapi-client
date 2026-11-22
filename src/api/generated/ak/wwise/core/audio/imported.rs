// Generated bindings for topic ak.wwise.core.audio.imported

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.audio.imported`.
///
/// Sent at the end of an import operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreAudioImportedPayload {
    /// The list of imported original files.
    pub files: Vec<String>,
    /// The operation applied for this import call.
    pub operation: String,
    /// The list of objects that have been created as part of the same import operation.
    pub objects: Vec<WwiseObject>,
}

/// Subscription options for `ak.wwise.core.audio.imported`.
///
/// Sent at the end of an import operation.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreAudioImportedOptions {
    /// Data to be returned for every imported object.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
}

impl CoreAudioImportedOptions {
    /// Creates a new [`CoreAudioImportedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every imported object.
    pub fn return_fields<I, T>(mut self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields = Some(return_fields.into_iter().map(Into::into).collect());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_<I, T>(self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields(return_fields)
    }
}

impl From<()> for CoreAudioImportedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreAudioImportedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreAudioImportedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreAudioImportedPayload`].
pub type ImportedPayload = CoreAudioImportedPayload;

/// Module-scoped alias for [`CoreAudioImportedOptions`].
pub type ImportedOptions = CoreAudioImportedOptions;
