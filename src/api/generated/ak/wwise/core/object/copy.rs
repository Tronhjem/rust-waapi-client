// Generated bindings for ak.wwise.core.object.copy

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.copy`.
///
/// Copies an object to the given parent. Note that if a Work Unit is copied, the operation cannot be undone and the project will be saved.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectCopyArgs {
    /// The ID (GUID), name, or path of the object's new parent.
    pub parent: ObjectRef,
    /// The ID (GUID), name, or path of the object to be copied.
    pub object: ObjectRef,
    /// Determines if Wwise automatically performs an Add source control operation for affected work units. Defaults to true.
    #[serde(
        rename = "autoAddToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_add_to_source_control: Option<bool>,
    /// The action to take if "parent" already has a child with the same name. Default value is "fail".
    #[serde(rename = "onNameConflict", skip_serializing_if = "Option::is_none")]
    pub on_name_conflict: Option<OnNameConflict>,
    /// Determines if Wwise automatically performs a Checkout source control operation for affected work units and for the project. Defaults to true.
    #[serde(
        rename = "autoCheckOutToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_check_out_to_source_control: Option<bool>,
}

impl CoreObjectCopyArgs {
    /// Creates a new [`CoreObjectCopyArgs`] with the required fields.
    pub fn new(object: ObjectRef, parent: ObjectRef) -> Self {
        Self {
            object,
            parent,
            auto_add_to_source_control: None,
            on_name_conflict: None,
            auto_check_out_to_source_control: None,
        }
    }

    /// Determines if Wwise automatically performs an Add source control operation for affected work units. Defaults to true.
    pub fn auto_add_to_source_control(mut self, auto_add_to_source_control: bool) -> Self {
        self.auto_add_to_source_control = Some(auto_add_to_source_control);
        self
    }

    /// The action to take if "parent" already has a child with the same name. Default value is "fail".
    pub fn on_name_conflict(mut self, on_name_conflict: OnNameConflict) -> Self {
        self.on_name_conflict = Some(on_name_conflict);
        self
    }

    /// Determines if Wwise automatically performs a Checkout source control operation for affected work units and for the project. Defaults to true.
    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: bool,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control);
        self
    }
}

/// Options for `ak.wwise.core.object.copy`.
///
/// Specifies the return options for every object created.
pub type CoreObjectCopyOptions = ObjectReturnOptions;

/// Response from `ak.wwise.core.object.copy`.
///
/// The copied object.
pub type CoreObjectCopyResponse = WwiseObject;
