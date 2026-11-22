// Generated bindings for ak.wwise.core.object.setReference

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setReference`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetReferenceArgs {
    /// The ID (GUID), name, path or definition of the object to be referred to.
    pub value: ObjectRef,
    /// The ID (GUID), name, or path of the object which has the reference.
    pub object: ObjectRef,
    /// The ID (GUID) or unique name of the platform to link the reference. Set to null-guid for unlinked reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// The name of the reference to set. See for more information on the references available.
    pub reference: Property,
}

impl CoreObjectSetReferenceArgs {
    pub fn new(object: ObjectRef, reference: Property, value: ObjectRef) -> Self {
        Self {
            object,
            reference,
            value,
            platform: None,
        }
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
