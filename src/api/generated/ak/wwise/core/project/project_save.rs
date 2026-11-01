// Generated bindings for ak.wwise.core.project.save

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.wwise.core.project.save`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreProjectSaveArgs {
    /// Automatically check out affected work units.
    #[serde(rename = "autoCheckOutToSourceControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_check_out_to_source_control: Option<bool>,
}

impl WwiseCoreProjectSaveArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auto_check_out_to_source_control(mut self, auto_check_out_to_source_control: impl Into<bool>) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }
}

/// Options for `ak.wwise.core.project.save`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreProjectSaveOptions {
}

impl WwiseCoreProjectSaveOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
