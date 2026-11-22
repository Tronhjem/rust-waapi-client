// Generated bindings for ak.wwise.core.blendContainer.removeAssignment

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.blendContainer.removeAssignment`.
///
/// Removes an assignment from a Blend Track. Equivalent to deleting an entry in the Blend Tracks Editor.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreBlendContainerRemoveAssignmentArgs {
    /// The ID (GUID) of the Blend Track.
    pub object: String,
    /// The ID (GUID), name, or path of the object to unassign.
    pub child: ObjectRef,
}

impl CoreBlendContainerRemoveAssignmentArgs {
    /// Creates a new [`CoreBlendContainerRemoveAssignmentArgs`] with the required fields.
    pub fn new(object: impl Into<String>, child: impl Into<ObjectRef>) -> Self {
        Self {
            object: object.into(),
            child: child.into(),
        }
    }
}

impl<T0: Into<String>, T1: Into<ObjectRef>> From<(T0, T1)>
    for CoreBlendContainerRemoveAssignmentArgs
{
    fn from((object, child): (T0, T1)) -> Self {
        Self::new(object, child)
    }
}

/// Module-scoped alias for [`CoreBlendContainerRemoveAssignmentArgs`].
pub type RemoveAssignmentArgs = CoreBlendContainerRemoveAssignmentArgs;
