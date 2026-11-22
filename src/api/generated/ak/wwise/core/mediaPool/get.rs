// Generated bindings for ak.wwise.core.mediaPool.get

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.mediaPool.get`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreMediaPoolGetArgs {
    /// The list of databases to include. If not specified, all databases are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<ObjectRef>>,
    /// The list of filters to apply. When not specified, no filters are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
    /// The text for the fuzzy search in the Media Pool. The search is case-insensitive. When not specified, all files matching the specified filter and specified databases are returned.
    #[serde(rename = "searchText", skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    /// The maximum number of results to return. If not specified, a maximum 1000 files are returned.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
}

impl CoreMediaPoolGetArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn databases(mut self, databases: impl Into<Vec<ObjectRef>>) -> Self {
        self.databases = Some(databases.into());
        self
    }

    pub fn filters(mut self, filters: impl Into<Vec<serde_json::Value>>) -> Self {
        self.filters = Some(filters.into());
        self
    }

    pub fn search_text(mut self, search_text: impl Into<String>) -> Self {
        self.search_text = Some(search_text.into());
        self
    }

    pub fn max_results(mut self, max_results: impl Into<i64>) -> Self {
        self.max_results = Some(max_results.into());
        self
    }
}

/// Options for `ak.wwise.core.mediaPool.get`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreMediaPoolGetOptions {
    /// The list of properties to return for each file. If not specified, the following properties are returned: 'Path', 'FileId', 'Db'.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<String>>,
}

impl CoreMediaPoolGetOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_(mut self, return_: impl Into<Vec<String>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }
}

/// Response from `ak.wwise.core.mediaPool.get`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreMediaPoolGetResponse {
    /// The list of files matching the search criteria. Each file is represented as an object with properties defined by the options.
    #[serde(rename = "return", default)]
    pub return_: Vec<serde_json::Value>,
}
