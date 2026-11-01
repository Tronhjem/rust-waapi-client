// Generated bindings for ak.wwise.core.object.get

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.wwise.core.object.get`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WwiseCoreObjectGetArgs {
    /// The query starting point.
    pub from: ObjectQuery,
    /// Sequential transformations on the object list returned by from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
}

impl WwiseCoreObjectGetArgs {
    pub fn new(from: ObjectQuery) -> Self {
        Self {
            from,
            transform: None,
        }
    }

    pub fn transform(mut self, transform: impl Into<Vec<Transform>>) -> Self {
        self.transform = Some(transform.into());
        self
    }
}

/// Options for `ak.wwise.core.object.get`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreObjectGetOptions {
    /// List of fields to return for each object.
    #[serde(rename = "return")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<String>>,
    /// Platform to use for platform-specific values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Language to use for language-specific values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageRef>,
}

impl WwiseCoreObjectGetOptions {
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

/// Response from `ak.wwise.core.object.get`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreObjectGetResponse {
    /// Objects matching the query.
    pub objects: Vec<WwiseObject>,
    /// Fields that were returned.
    #[serde(rename = "return")]
    pub return_fields: Vec<String>,
}
