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
    pub state: Vec<WidgetStateUpdate>,
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
}

impl UiModelSetStateArgs {
    /// Creates a new [`UiModelSetStateArgs`] with the required fields.
    pub fn new(handle_id: impl Into<String>, state: impl Into<Vec<WidgetStateUpdate>>) -> Self {
        Self {
            handle_id: handle_id.into(),
            state: state.into(),
        }
    }
}
