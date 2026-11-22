// Generated bindings for ak.wwise.ui.model.setState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.model.setState`.
///
/// Sets the state of widgets.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiModelSetStateArgs {
    /// The widgets and attributes to set.
    pub state: Vec<serde_json::Value>,
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
}

impl UiModelSetStateArgs {
    /// Creates a new [`UiModelSetStateArgs`] with the required fields.
    pub fn new(handle_id: impl Into<String>, state: Vec<serde_json::Value>) -> Self {
        Self {
            handle_id: handle_id.into(),
            state,
        }
    }
}

impl<T0: Into<String>, T1: Into<Vec<serde_json::Value>>> From<(T0, T1)> for UiModelSetStateArgs {
    fn from((handle_id, state): (T0, T1)) -> Self {
        Self::new(handle_id, state.into())
    }
}

/// Module-scoped alias for [`UiModelSetStateArgs`].
pub type SetStateArgs = UiModelSetStateArgs;
