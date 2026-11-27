// Generated bindings for topic ak.wwise.core.object.structureChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.structureChanged`.
///
/// Sent when the project structure changes. Publishes a summary of the changes. The publication is sent after the undo stack is modified. The changes are grouped in batch and redundant changes are collapsed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectStructureChangedPayload {
    /// The list of changed objects.
    #[serde(default)]
    pub objects: Vec<StructureChangedObject>,
    /// The operation that triggered the changes. Generally, the name of the top-level undo operation.
    pub summary: String,
}

/// Subscription options for `ak.wwise.core.object.structureChanged`.
///
/// Sent when the project structure changes. Publishes a summary of the changes. The publication is sent after the undo stack is modified. The changes are grouped in batch and redundant changes are collapsed.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectStructureChangedOptions {
    /// Data to be returned for the objects involved in the structure changes.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// If true, all loaded or unloaded objects are provided when a Work Unit is loaded or unloaded. If false or not provided, only the Work Unit objects that are loaded or unloaded is provided. Note that several Work Unit objects could be provided in a single structure change operation since Work Unit objects can be children of other Work Unit objects.
    #[serde(
        rename = "includeLoadedAndUnloadedNestedObjects",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_loaded_and_unloaded_nested_objects: Option<bool>,
}

impl CoreObjectStructureChangedOptions {
    /// Creates a new [`CoreObjectStructureChangedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for the objects involved in the structure changes.
    pub fn return_fields(mut self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_(self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields(return_fields)
    }

    /// If true, all loaded or unloaded objects are provided when a Work Unit is loaded or unloaded. If false or not provided, only the Work Unit objects that are loaded or unloaded is provided. Note that several Work Unit objects could be provided in a single structure change operation since Work Unit objects can be children of other Work Unit objects.
    pub fn include_loaded_and_unloaded_nested_objects(
        mut self,
        include_loaded_and_unloaded_nested_objects: bool,
    ) -> Self {
        self.include_loaded_and_unloaded_nested_objects =
            Some(include_loaded_and_unloaded_nested_objects);
        self
    }
}
