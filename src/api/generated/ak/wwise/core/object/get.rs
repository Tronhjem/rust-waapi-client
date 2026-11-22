// Generated bindings for ak.wwise.core.object.get



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.object.get`. Specify either a WAQL query or
/// a `from` query; both are optional in the schema but one is required in practice.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectGetArgs {
    /// Specifies a query in the WAQL language. See the WAQL introduction documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waql: Option<String>,
    /// The query starting point. Note, this is deprecated. The WAQL argument should be used instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<ObjectQuery>,
    /// Array of sequential transformations chained on the object list returned by `from`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
}

impl CoreObjectGetArgs {
    /// Creates a new [`CoreObjectGetArgs`] starting from a `from` [`ObjectQuery`].
    pub fn new(from: ObjectQuery) -> Self {
        Self {
            waql: None,
            from: Some(from),
            transform: None,
        }
    }

    /// Creates a new [`CoreObjectGetArgs`] from a WAQL query string.
    pub fn from_waql(waql: impl Into<String>) -> Self {
        Self {
            waql: Some(waql.into()),
            from: None,
            transform: None,
        }
    }

    /// Specifies a query in the WAQL language.
    pub fn waql(mut self, waql: impl Into<String>) -> Self {
        self.waql = Some(waql.into());
        self
    }

    /// Sets the query starting point (`from`).
    pub fn from(mut self, from: impl Into<ObjectQuery>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the sequential transformations applied to the object list returned by `from`.
    pub fn transform(mut self, transform: impl Into<Vec<Transform>>) -> Self {
        self.transform = Some(transform.into());
        self
    }
}

/// Options for `ak.wwise.core.object.get`.
///
/// Specifies the return options for every object returned.
pub type CoreObjectGetOptions = ObjectReturnOptions;

/// Response from `ak.wwise.core.object.get`.
///
/// Performs a query and returns the data, as specified in the options, for each object in the query result. The query can specify either a 'waql' argument or a 'from' argument with an optional 'transform' argument. See the WAQL introduction documentation or the WAAPI query documentation for more information. See the WAAPI query return options documentation to learn about options.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectGetResponse {
    /// An array of objects found, with properties matching what was supplied in the options.
    #[serde(rename = "return", default)]
    pub objects: Vec<WwiseObject>,
}

