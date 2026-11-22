// Generated bindings for ak.wwise.ui.cli.launch

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.cli.launch`.
///
/// Launch the main window of WwiseXP.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UiCliLaunchArgs {
    /// When no project is specified, a temporary project is loaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl UiCliLaunchArgs {
    /// Creates a new [`UiCliLaunchArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// When no project is specified, a temporary project is loaded.
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }
}

/// Module-scoped alias for [`UiCliLaunchArgs`].
pub type LaunchArgs = UiCliLaunchArgs;
