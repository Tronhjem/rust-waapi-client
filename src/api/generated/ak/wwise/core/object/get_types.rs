// Generated bindings for ak.wwise.core.object.getTypes



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Response from `ak.wwise.core.object.getTypes`.
///
/// Retrieves the list of all object types registered in Wwise's object model. This function returns the equivalent of the Wwise Objects Reference.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectGetTypesResponse {
    /// An array of all Wwise object types.
    #[serde(rename = "return", default)]
    pub types: Vec<WwiseObjectTypeInfo>,
}

