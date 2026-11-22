// Generated bindings for ak.wwise.ui.layout.undockView

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.undockView`.
///
/// Undock a view from a layout.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutUndockViewArgs {
    /// The name of the layout from which to undock the view.
    pub name: String,
    /// The unique id of the view to undock.
    #[serde(rename = "viewID")]
    pub view_id: String,
    /// Position of the undocked view on the horizontal axis. Defaults to 0.
    #[serde(rename = "posX", skip_serializing_if = "Option::is_none")]
    pub pos_x: Option<i64>,
    /// Position of the undocked view on the vertical axis. Defaults to 0.
    #[serde(rename = "posY", skip_serializing_if = "Option::is_none")]
    pub pos_y: Option<i64>,
}

impl UiLayoutUndockViewArgs {
    /// Creates a new [`UiLayoutUndockViewArgs`] with the required fields.
    pub fn new(name: String, view_id: String) -> Self {
        Self {
            name,
            view_id,
            pos_x: None,
            pos_y: None,
        }
    }

    /// Position of the undocked view on the horizontal axis. Defaults to 0.
    pub fn pos_x(mut self, pos_x: impl Into<i64>) -> Self {
        self.pos_x = Some(pos_x.into());
        self
    }

    /// Position of the undocked view on the vertical axis. Defaults to 0.
    pub fn pos_y(mut self, pos_y: impl Into<i64>) -> Self {
        self.pos_y = Some(pos_y.into());
        self
    }
}
