// Generated bindings for ak.wwise.ui.getSelectedObjects

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Options for `ak.wwise.ui.getSelectedObjects`.
///
/// Specifies the return options for every object returned.
pub type UiGetSelectedObjectsOptions = ObjectReturnOptions;

/// Response from `ak.wwise.ui.getSelectedObjects`.
///
/// Retrieves the list of objects currently selected by the user in the active view. Note that this function is not available in WwiseConsole.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiGetSelectedObjectsResponse {
    /// The selected objects, in the format specified in the options. Can be empty if no object is selected.
    #[serde(default)]
    pub objects: Vec<WwiseObject>,
}
