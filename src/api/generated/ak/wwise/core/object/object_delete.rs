// Generated bindings for ak.wwise.core.object.delete

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.wwise.core.object.delete`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreObjectDeleteArgs {
    /// The object to delete.
    pub object: ObjectRef,
    /// Automatically check out affected work units.
    #[serde(rename = "autoCheckOutToSourceControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_check_out_to_source_control: Option<bool>,
}

impl WwiseCoreObjectDeleteArgs {
    pub fn new(object: ObjectRef) -> Self {
        Self {
            object,
            auto_check_out_to_source_control: None,
        }
    }

    pub fn auto_check_out_to_source_control(mut self, auto_check_out_to_source_control: impl Into<bool>) -> Self {
        self.auto_check_out_to_source_control = Some(auto_check_out_to_source_control.into());
        self
    }
}

/// Options for `ak.wwise.core.object.delete`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreObjectDeleteOptions {
}

impl WwiseCoreObjectDeleteOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
