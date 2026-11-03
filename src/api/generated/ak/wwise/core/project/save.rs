// Generated bindings for ak.wwise.core.project.save

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.project.save`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct WwiseCoreProjectSaveArgs {
    /// Determines if Wwise automatically performs a Checkout source control operation for affected work units and for the project
    /// Defaults to true.
    #[serde(
        rename = "autoCheckOutToSourceControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_check_out_to_source_control: Option<bool>,
}

impl WwiseCoreProjectSaveArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auto_check_out_to_source_control(
        mut self,
        auto_check_out_to_source_control: impl Into<bool>,
    ) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }
}
