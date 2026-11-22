// Generated bindings for ak.wwise.ui.layout.getElementRectangle

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.getElementRectangle`.
///
/// Retrieves the current allocated rectangle of a layout element. An empty rect is returned if the element is not found.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutGetElementRectangleArgs {
    /// The ID of the element.
    pub id: String,
}

impl UiLayoutGetElementRectangleArgs {
    /// Creates a new [`UiLayoutGetElementRectangleArgs`] with the required fields.
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Response from `ak.wwise.ui.layout.getElementRectangle`.
///
/// Retrieves the current allocated rectangle of a layout element. An empty rect is returned if the element is not found.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutGetElementRectangleResponse {
    /// The left position of the element.
    #[serde(default)]
    pub left: Option<f64>,
    /// The top position of the element.
    #[serde(default)]
    pub top: Option<f64>,
    /// The right position of the element.
    #[serde(default)]
    pub right: Option<f64>,
    /// The bottom position of the element.
    #[serde(default)]
    pub bottom: Option<f64>,
}
