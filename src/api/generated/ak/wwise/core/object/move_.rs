// Generated bindings for ak.wwise.core.object.move

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.move`.
///
/// Moves an object to the given parent. Returns the moved object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectMoveArgs {
    /// The ID (GUID), name, or path of the object's new parent.
    pub parent: ObjectRef,
    /// The ID (GUID), name, or path of the object to be moved.
    pub object: ObjectRef,
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

impl CoreObjectMoveArgs {
    /// Creates a new [`CoreObjectMoveArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>, parent: impl Into<ObjectRef>) -> Self {
        Self {
            object: object.into(),
            parent: parent.into(),
            on_name_conflict: None,
            auto_check_out_to_source_control: None,
        }
    }

    /// The action to take if "parent" already has a child with the same name. Default value is "fail".
    pub fn on_name_conflict(mut self, on_name_conflict: impl Into<OnNameConflict>) -> Self {
        self.on_name_conflict = Some(on_name_conflict.into());
        self
    }

    /// Determines if Wwise automatically performs a Checkout source control operation for affected work units and for the project. Defaults to true.
    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }
}

impl<T0: Into<ObjectRef>, T1: Into<ObjectRef>> From<(T0, T1)> for CoreObjectMoveArgs {
    fn from((object, parent): (T0, T1)) -> Self {
        Self::new(object, parent)
    }
}

/// Response from `ak.wwise.core.object.move`.
///
/// The moved object.
pub type CoreObjectMoveResponse = WwiseObject;

/// Module-scoped alias for [`CoreObjectMoveArgs`].
pub type MoveArgs = CoreObjectMoveArgs;

/// Module-scoped alias for [`CoreObjectMoveResponse`].
pub type MoveResponse = CoreObjectMoveResponse;
