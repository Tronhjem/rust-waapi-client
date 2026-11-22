// Generated bindings for ak.wwise.core.transport.prepare



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.transport.prepare`.
///
/// Prepare the object and its dependencies for playback. Use this function before calling PostEventSync or PostMIDIOnEventSync from IAkGlobalPluginContext.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportPrepareArgs {
    /// The ID (GUID), name, or path of the object to control via the transport object.
    pub object: ObjectRef,
}

impl CoreTransportPrepareArgs {
    /// Creates a new [`CoreTransportPrepareArgs`] with the required fields.
    pub fn new(object: ObjectRef) -> Self {
        Self {
            object,
        }
    }
}
