// Generated bindings for ak.wwise.core.switchContainer.getAssignments

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.switchContainer.getAssignments`.
///
/// Returns the list of assignments between a Switch Container's children and states.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSwitchContainerGetAssignmentsArgs {
    /// The ID (GUID), name, or path of the Switch Container.
    pub id: ObjectRef,
}

impl CoreSwitchContainerGetAssignmentsArgs {
    /// Creates a new [`CoreSwitchContainerGetAssignmentsArgs`] with the required fields.
    pub fn new(id: impl Into<ObjectRef>) -> Self {
        Self { id: id.into() }
    }
}

impl From<ObjectRef> for CoreSwitchContainerGetAssignmentsArgs {
    fn from(value: ObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreSwitchContainerGetAssignmentsArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CoreSwitchContainerGetAssignmentsArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreSwitchContainerGetAssignmentsArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.switchContainer.getAssignments`.
///
/// Returns the list of assignments between a Switch Container's children and states.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSwitchContainerGetAssignmentsResponse {
    /// List of assignments (pairs of child and state).
    #[serde(rename = "return", default)]
    pub assignments: Vec<SwitchContainerAssignment>,
}

/// Module-scoped alias for [`CoreSwitchContainerGetAssignmentsArgs`].
pub type GetAssignmentsArgs = CoreSwitchContainerGetAssignmentsArgs;

/// Module-scoped alias for [`CoreSwitchContainerGetAssignmentsResponse`].
pub type GetAssignmentsResponse = CoreSwitchContainerGetAssignmentsResponse;
