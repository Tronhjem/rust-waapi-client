// Generated bindings for topic ak.wwise.core.object.notesChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.notesChanged`.
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
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectNotesChangedOptions {
    /// Data to be returned for every object's note change.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectNotesChangedOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_(mut self, return_: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
