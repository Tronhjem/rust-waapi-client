// Generated bindings for ak.wwise.core.undo.cancelGroup



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.undo.cancelGroup`.
///
/// Cancels the last undo group.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreUndoCancelGroupArgs {
    /// Specify if the operations are undone. False when not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo: Option<bool>,
}

impl CoreUndoCancelGroupArgs {
    /// Creates a new [`CoreUndoCancelGroupArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify if the operations are undone. False when not specified.
    pub fn undo(mut self, undo: impl Into<bool>) -> Self {
        self.undo = Some(undo.into());
        self
    }
}
