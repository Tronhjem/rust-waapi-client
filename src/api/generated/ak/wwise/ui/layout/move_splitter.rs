// Generated bindings for ak.wwise.ui.layout.moveSplitter

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.moveSplitter`.
///
/// Moves a splitter by a delta given in pixels.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutMoveSplitterArgs {
    /// The ID of the splitter to move.
    pub id: String,
    /// The delta, in pixels, to add to the splitter position.
    pub delta: i64,
}

impl UiLayoutMoveSplitterArgs {
    /// Creates a new [`UiLayoutMoveSplitterArgs`] with the required fields.
    pub fn new(id: impl Into<String>, delta: i64) -> Self {
        Self {
            id: id.into(),
            delta,
        }
    }
}

impl<T0: Into<String>, T1: Into<i64>> From<(T0, T1)> for UiLayoutMoveSplitterArgs {
    fn from((id, delta): (T0, T1)) -> Self {
        Self::new(id, delta.into())
    }
}

/// Module-scoped alias for [`UiLayoutMoveSplitterArgs`].
pub type MoveSplitterArgs = UiLayoutMoveSplitterArgs;
