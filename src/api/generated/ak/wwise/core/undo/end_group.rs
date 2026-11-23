// Generated bindings for ak.wwise.core.undo.endGroup

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.undo.endGroup`.
///
/// Ends the last undo group.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreUndoEndGroupArgs {
    /// The name that is displayed in the history for this undo group.
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl CoreUndoEndGroupArgs {
    /// Creates a new [`CoreUndoEndGroupArgs`] with the required fields.
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            display_name: display_name.into(),
        }
    }
}
