// Generated bindings for ak.wwise.ui.getSelectedFiles

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.ui.getSelectedFiles`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiGetSelectedFilesResponse {
    /// The selected files. Can be empty if no file is selected.
    pub files: Vec<String>,
}
