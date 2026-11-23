// Generated bindings for ak.wwise.ui.project.open

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.project.open`.
///
/// Opens a project, specified by path. Please see `ak.wwise.core.project.loaded` for further explanations on how to be notified when the operation has completed.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiProjectOpenArgs {
    /// The path to the project WPROJ file. For using WAAPI on Mac, please see the WAAPI Mac path documentation.
    pub path: String,
    /// (DEPRECATED) The action to be executed when opening a project that needs to be upgraded. Note, this is deprecated. Please use onMigrationRequired instead.
    #[serde(rename = "onUpgrade", skip_serializing_if = "Option::is_none")]
    pub on_upgrade: Option<ProjectMigrationAction>,
    /// The action to be executed when opening a project that needs to be migrated.
    #[serde(
        rename = "onMigrationRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_migration_required: Option<ProjectMigrationAction>,
    /// Indicates if the user should not be prompted to save the current project. Defaults to true.
    #[serde(rename = "bypassSave", skip_serializing_if = "Option::is_none")]
    pub bypass_save: Option<bool>,
    /// Determines if Wwise automatically performs a Checkout source control operation for affected work units and for the project. Defaults to true.
    #[serde(
        rename = "autoCheckOutToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_check_out_to_source_control: Option<bool>,
}

impl UiProjectOpenArgs {
    /// Creates a new [`UiProjectOpenArgs`] with the required fields.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            on_upgrade: None,
            on_migration_required: None,
            bypass_save: None,
            auto_check_out_to_source_control: None,
        }
    }

    /// (DEPRECATED) The action to be executed when opening a project that needs to be upgraded. Note, this is deprecated. Please use onMigrationRequired instead.
    pub fn on_upgrade(mut self, on_upgrade: ProjectMigrationAction) -> Self {
        self.on_upgrade = Some(on_upgrade);
        self
    }

    /// The action to be executed when opening a project that needs to be migrated.
    pub fn on_migration_required(mut self, on_migration_required: ProjectMigrationAction) -> Self {
        self.on_migration_required = Some(on_migration_required);
        self
    }

    /// Indicates if the user should not be prompted to save the current project. Defaults to true.
    pub fn bypass_save(mut self, bypass_save: bool) -> Self {
        self.bypass_save = Some(bypass_save);
        self
    }

    /// Determines if Wwise automatically performs a Checkout source control operation for affected work units and for the project. Defaults to true.
    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: bool,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control);
        self
    }
}
