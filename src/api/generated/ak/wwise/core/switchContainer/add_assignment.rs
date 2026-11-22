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
    pub fn new(child: impl Into<ObjectRef>, state_or_switch: impl Into<ObjectRef>) -> Self {
        Self {
            child: child.into(),
            state_or_switch: state_or_switch.into(),
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<ObjectRef>> From<(T0, T1)>
    for CoreSwitchContainerAddAssignmentArgs
{
    fn from((child, state_or_switch): (T0, T1)) -> Self {
        Self::new(child, state_or_switch)
    }
}

/// Module-scoped alias for [`CoreSwitchContainerAddAssignmentArgs`].
pub type AddAssignmentArgs = CoreSwitchContainerAddAssignmentArgs;
