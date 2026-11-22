// Generated bindings for ak.wwise.core.workUnit.unload



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.workUnit.unload`.
///
/// Unload a Work Unit. No object contained in the Work Unit will be available after this call. If the Work Unit is modified and not saved, the function will return error. The Undo history will be cleared.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreWorkUnitUnloadArgs {
    /// The ID (GUID), name, or path of the Work Unit to unload.
    pub object: ObjectRef,
}

impl CoreWorkUnitUnloadArgs {
    /// Creates a new [`CoreWorkUnitUnloadArgs`] with the required fields.
    pub fn new(object: ObjectRef) -> Self {
        Self {
            object,
        }
    }
}
