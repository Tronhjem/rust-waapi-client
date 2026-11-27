// Generated bindings for topic ak.wwise.ui.selectionChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.ui.selectionChanged`.
///
/// Sent when the selection changes in the project.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiSelectionChangedPayload {
    /// The selected objects, in the format specified in the options. Can be empty if no object is selected.
    #[serde(default)]
    pub objects: Vec<WwiseObject>,
}

/// Subscription options for `ak.wwise.ui.selectionChanged`.
///
/// Specifies the return options for every returned object.
pub type UiSelectionChangedOptions = ObjectReturnOptions;
