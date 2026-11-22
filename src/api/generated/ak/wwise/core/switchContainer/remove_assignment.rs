// Generated bindings for ak.wwise.core.switchContainer.removeAssignment

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.switchContainer.removeAssignment`.
///
/// Removes an assignment between a Switch Container's child and a State.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSwitchContainerRemoveAssignmentArgs {
    /// The ID (GUID), name, or path of the object assigned to a State. This object must be the child of a Switch Container and must be currently assigned to a State.
    pub child: ObjectRef,
    /// The ID (GUID), name, or path of the State or Switch to which the child is assigned. Must be the child of the Switch Group or State Group that is currently set for the Switch Container.
    #[serde(rename = "stateOrSwitch")]
    pub state_or_switch: ObjectRef,
}

impl CoreSwitchContainerRemoveAssignmentArgs {
    /// Creates a new [`CoreSwitchContainerRemoveAssignmentArgs`] with the required fields.
    pub fn new(child: impl Into<ObjectRef>, state_or_switch: impl Into<ObjectRef>) -> Self {
        Self {
            child: child.into(),
            state_or_switch: state_or_switch.into(),
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<ObjectRef>> From<(T0, T1)>
    for CoreSwitchContainerRemoveAssignmentArgs
{
    fn from((child, state_or_switch): (T0, T1)) -> Self {
        Self::new(child, state_or_switch)
    }
}

/// Module-scoped alias for [`CoreSwitchContainerRemoveAssignmentArgs`].
pub type RemoveAssignmentArgs = CoreSwitchContainerRemoveAssignmentArgs;
