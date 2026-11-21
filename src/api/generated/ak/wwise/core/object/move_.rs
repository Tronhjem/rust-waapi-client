// Generated bindings for ak.wwise.core.object.move

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.move`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectMoveArgs {
    /// The ID (GUID), name, or path of the object's new parent.
    pub parent: ObjectRef,
    /// The ID (GUID), name, or path of the object to be moved.
    pub object: ObjectRef,
    /// The action to take if "parent" already has a child with the same name
    /// Default value is "fail".
    #[serde(rename = "onNameConflict", skip_serializing_if = "Option::is_none")]
    pub on_name_conflict: Option<OnNameConflict>,
    /// Determines if Wwise automatically performs a Checkout source control operation for affected work units and for the project
    /// Defaults to true.
    #[serde(
        rename = "autoCheckOutToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_check_out_to_source_control: Option<bool>,
}

impl CoreObjectMoveArgs {
    pub fn new(object: ObjectRef, parent: ObjectRef) -> Self {
        Self {
            object,
            parent,
            on_name_conflict: None,
            auto_check_out_to_source_control: None,
        }
    }

    pub fn on_name_conflict(mut self, on_name_conflict: impl Into<OnNameConflict>) -> Self {
        self.on_name_conflict = Some(on_name_conflict.into());
        self
    }

    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }
}

/// Response from `ak.wwise.core.object.move`.
pub type CoreObjectMoveResponse = WwiseObject;
