// Generated bindings for topic ak.wwise.core.project.saved

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.project.saved`.
///
/// Sent when the project has been saved.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProjectSavedPayload {
    /// Array of paths.
    #[serde(rename = "modifiedPaths")]
    pub modified_paths: Vec<String>,
}
