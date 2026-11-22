// Generated bindings for ak.wwise.ui.model.getState

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.model.getState`.
///
/// Gets the state of widgets.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiModelGetStateArgs {
    /// The widgets and attributes to query.
    pub state: Vec<serde_json::Value>,
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
}

impl UiModelGetStateArgs {
    /// Creates a new [`UiModelGetStateArgs`] with the required fields.
    pub fn new(handle_id: impl Into<String>, state: Vec<serde_json::Value>) -> Self {
        Self {
            handle_id: handle_id.into(),
            state,
        }
    }
}

impl<T0: Into<String>, T1: Into<Vec<serde_json::Value>>> From<(T0, T1)> for UiModelGetStateArgs {
    fn from((handle_id, state): (T0, T1)) -> Self {
        Self::new(handle_id, state.into())
    }
}

/// Module-scoped alias for [`UiModelGetStateArgs`].
pub type GetStateArgs = UiModelGetStateArgs;
