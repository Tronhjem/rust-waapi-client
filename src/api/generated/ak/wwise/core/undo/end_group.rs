// Generated bindings for ak.wwise.core.undo.endGroup

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.undo.endGroup`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreUndoEndGroupArgs {
    /// The name that is displayed in the history for this undo group.
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl CoreUndoEndGroupArgs {
    pub fn new(display_name: String) -> Self {
        Self { display_name }
    }
}
