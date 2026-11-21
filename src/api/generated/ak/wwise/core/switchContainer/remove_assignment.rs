// Generated bindings for ak.wwise.core.switchContainer.removeAssignment

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.switchContainer.removeAssignment`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSwitchContainerRemoveAssignmentArgs {
    /// The ID (GUID), name, or path of the object assigned to a State
    /// This object must be the child of a Switch Container and must be currently assigned to a State.
    pub child: ObjectRef,
    /// The ID (GUID), name, or path of the State or Switch to which the child is assigned
    /// Must be the child of the Switch Group or State Group that is currently set for the Switch Container.
    #[serde(rename = "stateOrSwitch")]
    pub state_or_switch: ObjectRef,
}

impl CoreSwitchContainerRemoveAssignmentArgs {
    pub fn new(child: ObjectRef, state_or_switch: ObjectRef) -> Self {
        Self {
            child,
            state_or_switch,
        }
    }
}
