// Generated bindings for topic ak.wwise.core.object.notesChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.notesChanged`.
///
/// Sent when the object's notes are changed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectNotesChangedPayload {
    /// The object that had its notes changed.
    pub object: WwiseObject,
    /// The object's new notes.
    #[serde(rename = "newNotes")]
    pub new_notes: String,
    /// The previous notes of the object.
    #[serde(rename = "oldNotes")]
    pub old_notes: String,
}

/// Subscription options for `ak.wwise.core.object.notesChanged`.
///
/// Sent when the object's notes are changed.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectNotesChangedOptions {
    /// Data to be returned for every object's note change.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectNotesChangedOptions {
    /// Creates a new [`CoreObjectNotesChangedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every object's note change.
    pub fn return_fields(mut self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_(self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields(return_fields)
    }

    /// The ID (GUID) of the platform.
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
