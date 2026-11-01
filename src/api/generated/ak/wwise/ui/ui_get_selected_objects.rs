// Generated bindings for ak.wwise.ui.getSelectedObjects

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.wwise.ui.getSelectedObjects`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseUiGetselectedobjectsArgs {
}

impl WwiseUiGetselectedobjectsArgs {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Options for `ak.wwise.ui.getSelectedObjects`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseUiGetselectedobjectsOptions {
    /// Fields to return.
    #[serde(rename = "return")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<String>>,
    /// Platform for return values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Language for return values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageRef>,
}

impl WwiseUiGetselectedobjectsOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_fields(mut self, return_fields: impl Into<Vec<String>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn language(mut self, language: impl Into<LanguageRef>) -> Self {
        self.language = Some(language.into());
        self
    }
}

/// Response from `ak.wwise.ui.getSelectedObjects`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseUiGetselectedobjectsResponse {
    /// Selected objects.
    pub objects: Vec<WwiseObject>,
}
