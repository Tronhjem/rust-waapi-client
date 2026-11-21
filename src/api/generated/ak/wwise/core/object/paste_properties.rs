// Generated bindings for ak.wwise.core.object.pasteProperties

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.pasteProperties`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectPastePropertiesArgs {
    /// The ID (GUID), name, or path of the source object.
    pub source: ObjectRef,
    /// Array of target objects to paste into.
    pub targets: Vec<ObjectRef>,
    /// Paste mode for lists
    /// Default value is "replaceEntire"
    /// With "replaceEntire" all elements in the lists of a target object are removed and all selected elements from the source's lists are copied
    /// Conversely, with "addReplace" and "addKeep" elements in a target which are not in the source are not removed
    /// Elements in the source's list which are not in a target's list are added to the target
    /// For elements which are common to the source and a target "addReplace" will copy the one from the source, replacing the target's element, whereas "addKeep" will retain the element in the target.
    #[serde(rename = "pasteMode", skip_serializing_if = "Option::is_none")]
    pub paste_mode: Option<PasteMode>,
    /// Array of properties, references and lists to exclude from the paste operation
    /// When not specified, no properties, references and lists are excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion: Option<Vec<Property>>,
    /// Array of properties, references and lists to include in the paste operation
    /// When not specified, all properties, references and lists are included, and the exclusion defines which ones to exclude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion: Option<Vec<Property>>,
}

impl CoreObjectPastePropertiesArgs {
    pub fn new(source: ObjectRef, targets: Vec<ObjectRef>) -> Self {
        Self {
            source,
            targets,
            paste_mode: None,
            exclusion: None,
            inclusion: None,
        }
    }

    pub fn paste_mode(mut self, paste_mode: impl Into<PasteMode>) -> Self {
        self.paste_mode = Some(paste_mode.into());
        self
    }

    pub fn exclusion(mut self, exclusion: impl Into<Vec<Property>>) -> Self {
        self.exclusion = Some(exclusion.into());
        self
    }

    pub fn inclusion(mut self, inclusion: impl Into<Vec<Property>>) -> Self {
        self.inclusion = Some(inclusion.into());
        self
    }
}
