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
    pub fn new(object: String, child: ObjectRef) -> Self {
        Self {
            object,
            child,
        }
    }
}
