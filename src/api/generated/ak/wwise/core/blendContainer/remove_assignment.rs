// Generated bindings for ak.wwise.core.blendContainer.removeAssignment

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.blendContainer.removeAssignment`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreBlendContainerRemoveAssignmentArgs {
    /// The ID (GUID) of the Blend Track.
    pub object: String,
    /// The ID (GUID), name, or path of the object to unassign.
    pub child: ObjectRef,
}

impl CoreBlendContainerRemoveAssignmentArgs {
    pub fn new(object: String, child: ObjectRef) -> Self {
        Self { object, child }
    }
}
