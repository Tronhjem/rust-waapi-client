// Generated bindings for ak.wwise.ui.project.close

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.project.close`.
///
/// Closes the current project.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UiProjectCloseArgs {
    /// Indicates if the user should not be prompted to save the current project. Defaults to true.
    #[serde(rename = "bypassSave", skip_serializing_if = "Option::is_none")]
    pub bypass_save: Option<bool>,
}

impl UiProjectCloseArgs {
    /// Creates a new [`UiProjectCloseArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Indicates if the user should not be prompted to save the current project. Defaults to true.
    pub fn bypass_save(mut self, bypass_save: bool) -> Self {
        self.bypass_save = Some(bypass_save);
        self
    }
}

/// Response from `ak.wwise.ui.project.close`.
///
/// Closes the current project.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiProjectCloseResponse {
    /// True if there was a project open, false otherwise. Note that if there was no project open, no ak.wwise.core.project.preClosed or ak.wwise.core.project.postClosed event is issued.
    #[serde(rename = "hadProjectOpen")]
    pub had_project_open: bool,
}
