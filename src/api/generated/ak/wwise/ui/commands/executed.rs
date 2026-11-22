// Generated bindings for topic ak.wwise.ui.commands.executed



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Payload for `ak.wwise.ui.commands.executed`.
///
/// Sent when a command is executed. The objects for which the command is executed are sent in the publication.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiCommandsExecutedPayload {
    /// An array of platform for which the command is executed. Each platform is an ID (GUID).
    pub platforms: Vec<PlatformRef>,
    /// The command ID that was executed.
    pub command: String,
    /// The objects for which the command was executed. The objects are usually the selected objects. They are returned in the format specified in the options. Can be empty if no object is used.
    pub objects: Vec<WwiseObject>,
}


/// Subscription options for `ak.wwise.ui.commands.executed`.
///
/// Specifies the return options for every object returned.
pub type UiCommandsExecutedOptions = ObjectReturnOptions;
