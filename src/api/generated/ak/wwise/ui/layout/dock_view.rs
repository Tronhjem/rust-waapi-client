// Generated bindings for ak.wwise.ui.layout.dockView

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.dockView`.
///
/// Dock a floating view into a layout.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutDockViewArgs {
    /// The name of the layout where to dock the view.
    pub name: String,
    /// The unique id of the view to dock.
    #[serde(rename = "viewID")]
    pub view_id: String,
    /// The unique id of the target element.
    #[serde(rename = "targetID")]
    pub target_id: String,
    /// The side where to drop the view on the target.
    pub side: String,
}

impl UiLayoutDockViewArgs {
    /// Creates a new [`UiLayoutDockViewArgs`] with the required fields.
    pub fn new(
        name: impl Into<String>,
        view_id: impl Into<String>,
        target_id: impl Into<String>,
        side: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            view_id: view_id.into(),
            target_id: target_id.into(),
            side: side.into(),
        }
    }
}
