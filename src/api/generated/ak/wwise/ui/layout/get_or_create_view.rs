// Generated bindings for ak.wwise.ui.layout.getOrCreateView

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.getOrCreateView`.
///
/// Gets a view, if it exists in the current layout, or creates a new one.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutGetOrCreateViewArgs {
    /// The unique identifier, as a string, of the view type to get or create.
    pub name: String,
    /// Initial position of the created view on the horizontal axis. Defaults to -1.
    #[serde(rename = "posX", skip_serializing_if = "Option::is_none")]
    pub pos_x: Option<i64>,
    /// Initial position of the created view on the vertical axis. Defaults to -1.
    #[serde(rename = "posY", skip_serializing_if = "Option::is_none")]
    pub pos_y: Option<i64>,
}

impl UiLayoutGetOrCreateViewArgs {
    /// Creates a new [`UiLayoutGetOrCreateViewArgs`] with the required fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pos_x: None,
            pos_y: None,
        }
    }

    /// Initial position of the created view on the horizontal axis. Defaults to -1.
    pub fn pos_x(mut self, pos_x: i64) -> Self {
        self.pos_x = Some(pos_x);
        self
    }

    /// Initial position of the created view on the vertical axis. Defaults to -1.
    pub fn pos_y(mut self, pos_y: i64) -> Self {
        self.pos_y = Some(pos_y);
        self
    }
}

/// Response from `ak.wwise.ui.layout.getOrCreateView`.
///
/// Gets a view, if it exists in the current layout, or creates a new one.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutGetOrCreateViewResponse {
    /// The unique ID of the view.
    #[serde(default)]
    pub id: Option<String>,
}

impl UiLayoutGetOrCreateViewResponse {
    /// Returns the object GUID string (`id`), if present in the response.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}
