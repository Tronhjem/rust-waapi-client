// Generated bindings for ak.wwise.core.object.get

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.get`. Specify either a WAQL query or
/// a `from` query; both are optional in the schema but one is required in practice.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct WwiseCoreObjectGetArgs {
    /// A WAQL query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waql: Option<String>,
    /// The query starting point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<ObjectQuery>,
    /// Sequential transformations on the object list returned by `from`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
}

impl WwiseCoreObjectGetArgs {
    pub fn new(from: ObjectQuery) -> Self {
        Self {
            waql: None,
            from: Some(from),
            transform: None,
        }
    }

    pub fn from_waql(waql: impl Into<String>) -> Self {
        Self {
            waql: Some(waql.into()),
            from: None,
            transform: None,
        }
    }

    pub fn waql(mut self, waql: impl Into<String>) -> Self {
        self.waql = Some(waql.into());
        self
    }

    pub fn from(mut self, from: impl Into<ObjectQuery>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn transform(mut self, transform: impl Into<Vec<Transform>>) -> Self {
        self.transform = Some(transform.into());
        self
    }
}

/// Options for `ak.wwise.core.object.get`.

pub type WwiseCoreObjectGetOptions = ObjectReturnOptions;

/// Response from `ak.wwise.core.object.get`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreObjectGetResponse {
    /// An array of objects found, with properties matching what was supplied in the options.
    #[serde(rename = "return", default)]
    pub objects: Vec<WwiseObject>,
}
