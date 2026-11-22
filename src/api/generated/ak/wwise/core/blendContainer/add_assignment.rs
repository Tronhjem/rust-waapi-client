// Generated bindings for ak.wwise.core.blendContainer.addAssignment



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.blendContainer.addAssignment`.
///
/// Adds a new assignment to a Blend Track. Equivalent to performing a drag-and-drop operation in the Blend Tracks Editor.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreBlendContainerAddAssignmentArgs {
    /// The ID (GUID) of the Blend Track.
    pub object: String,
    /// The ID (GUID), name, or path of the object to assign. This object must be the child of a Blend Track's Blend Container.
    pub child: ObjectRef,
    /// The index of the child among the Blend Track's assignments. If a value is provided, it will be clamped to the range `[0-n]`, where n is the current number of assignments. If a value is not provided, then the assignment will be added at the end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// The configuration of the edges of the assignment within the Blend Track. This information is only useful if the Blend Track has a crossfade Game Parameter. If not specified, the edges will be configured using default values. If specified, there must be two edges: 0 = left, 1 = right.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<BlendEdge>>,
}

impl CoreBlendContainerAddAssignmentArgs {
    /// Creates a new [`CoreBlendContainerAddAssignmentArgs`] with the required fields.
    pub fn new(object: String, child: ObjectRef) -> Self {
        Self {
            object,
            child,
            index: None,
            edges: None,
        }
    }

    /// The index of the child among the Blend Track's assignments. If a value is provided, it will be clamped to the range `[0-n]`, where n is the current number of assignments. If a value is not provided, then the assignment will be added at the end.
    pub fn index(mut self, index: impl Into<i64>) -> Self {
        self.index = Some(index.into());
        self
    }

    /// The configuration of the edges of the assignment within the Blend Track. This information is only useful if the Blend Track has a crossfade Game Parameter. If not specified, the edges will be configured using default values. If specified, there must be two edges: 0 = left, 1 = right.
    pub fn edges(mut self, edges: impl Into<Vec<BlendEdge>>) -> Self {
        self.edges = Some(edges.into());
        self
    }
}

/// Response from `ak.wwise.core.blendContainer.addAssignment`.
///
/// Adds a new assignment to a Blend Track. Equivalent to performing a drag-and-drop operation in the Blend Tracks Editor.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreBlendContainerAddAssignmentResponse {
    /// The ID (GUID) the assigned object.
    #[serde(default)]
    pub child: Option<String>,
    /// The index of the child among the Blend Track's assignments.
    #[serde(default)]
    pub index: Option<i64>,
    /// The configuration of the edges of the assignment within the Blend Track; 0 = left, 1 = right.
    #[serde(default)]
    pub edges: Vec<BlendEdge>,
}

