// Generated bindings for ak.wwise.core.blendContainer.getAssignments

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.blendContainer.getAssignments`.
///
/// Returns a list of a Blend Track assignments.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreBlendContainerGetAssignmentsArgs {
    /// The ID (GUID), the Blend Track.
    pub object: String,
}

impl CoreBlendContainerGetAssignmentsArgs {
    /// Creates a new [`CoreBlendContainerGetAssignmentsArgs`] with the required fields.
    pub fn new(object: impl Into<String>) -> Self {
        Self {
            object: object.into(),
        }
    }
}

impl From<String> for CoreBlendContainerGetAssignmentsArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreBlendContainerGetAssignmentsArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreBlendContainerGetAssignmentsArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.blendContainer.getAssignments`.
///
/// Returns a list of a Blend Track assignments.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreBlendContainerGetAssignmentsResponse {
    /// Array of Blend Track assignments.
    #[serde(rename = "return", default)]
    pub assignments: Vec<BlendTrackAssignment>,
}

/// Module-scoped alias for [`CoreBlendContainerGetAssignmentsArgs`].
pub type GetAssignmentsArgs = CoreBlendContainerGetAssignmentsArgs;

/// Module-scoped alias for [`CoreBlendContainerGetAssignmentsResponse`].
pub type GetAssignmentsResponse = CoreBlendContainerGetAssignmentsResponse;
