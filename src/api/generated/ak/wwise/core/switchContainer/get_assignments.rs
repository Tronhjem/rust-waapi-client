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
    pub fn new(id: ObjectRef) -> Self {
        Self {
            id,
        }
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

