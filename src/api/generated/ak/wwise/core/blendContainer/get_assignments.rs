// Generated bindings for ak.wwise.core.blendContainer.getAssignments

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.blendContainer.getAssignments`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreBlendContainerGetAssignmentsArgs {
    /// The ID (GUID), the Blend Track.
    pub object: String,
}

impl CoreBlendContainerGetAssignmentsArgs {
    pub fn new(object: String) -> Self {
        Self { object }
    }
}

/// Response from `ak.wwise.core.blendContainer.getAssignments`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreBlendContainerGetAssignmentsResponse {
    /// Array of Blend Track assignments.
    #[serde(rename = "return", default)]
    pub assignments: Vec<BlendTrackAssignment>,
}
