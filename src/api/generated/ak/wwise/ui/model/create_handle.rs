// Generated bindings for ak.wwise.ui.model.createHandle

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.model.createHandle`.
///
/// Creates a frontend handle and instantiates its widgets.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiModelCreateHandleArgs {
    /// The template name (from WAFM).
    pub template: String,
    /// The ID of the parent container (e.g. window).
    pub container: String,
}

impl UiModelCreateHandleArgs {
    /// Creates a new [`UiModelCreateHandleArgs`] with the required fields.
    pub fn new(container: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            container: container.into(),
            template: template.into(),
        }
    }
}

impl<T0: Into<String>, T1: Into<String>> From<(T0, T1)> for UiModelCreateHandleArgs {
    fn from((container, template): (T0, T1)) -> Self {
        Self::new(container, template)
    }
}

/// Response from `ak.wwise.ui.model.createHandle`.
///
/// Creates a frontend handle and instantiates its widgets.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiModelCreateHandleResponse {
    /// The ID of the handle.
    #[serde(rename = "handleID", default)]
    pub handle_id: Option<String>,
}

/// Module-scoped alias for [`UiModelCreateHandleArgs`].
pub type CreateHandleArgs = UiModelCreateHandleArgs;

/// Module-scoped alias for [`UiModelCreateHandleResponse`].
pub type CreateHandleResponse = UiModelCreateHandleResponse;
