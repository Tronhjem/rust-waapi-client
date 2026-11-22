// Generated bindings for ak.wwise.core.switchContainer.addAssignment

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.switchContainer.addAssignment`.
///
/// Assigns a Switch Container's child to a Switch. This is the equivalent of doing a drag&drop of the child to a state in the Assigned Objects view. The child is always added at the end for each state.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSwitchContainerAddAssignmentArgs {
    /// The ID (GUID), name, or path of the object to assign to a State. This object must be the child of a Switch Container.
    pub child: ObjectRef,
    /// The ID (GUID), name, or path of the State or Switch with which to assign. Must be the child of the Switch Group or State Group that is currently set for the Switch Container.
    #[serde(rename = "stateOrSwitch")]
    pub state_or_switch: ObjectRef,
}

impl CoreSwitchContainerAddAssignmentArgs {
    /// Creates a new [`CoreSwitchContainerAddAssignmentArgs`] with the required fields.
    pub fn new(child: ObjectRef, state_or_switch: ObjectRef) -> Self {
        Self {
            child,
            state_or_switch,
        }
    }
}
