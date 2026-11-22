// Generated bindings for ak.wwise.core.audio.convert

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.convert`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioConvertArgs {
    /// An array of platforms. Each platform is an ID (GUID) or a unique name.
    pub platforms: Vec<PlatformRef>,
    /// An array of languages. Each language is a unique name.
    pub languages: Vec<LanguageRef>,
    /// An array of objects. Each object is an ID (GUID), name, or object path.
    pub objects: Vec<ObjectRef>,
}

impl CoreAudioConvertArgs {
    pub fn new(
        objects: Vec<ObjectRef>,
        platforms: Vec<PlatformRef>,
        languages: Vec<LanguageRef>,
    ) -> Self {
        Self {
            objects,
            platforms,
            languages,
        }
    }
}

/// Response from `ak.wwise.core.audio.convert`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreAudioConvertResponse {
    /// List of logged entries with associated messages and severities.
    #[serde(default)]
    pub errors: Vec<AudioConvertError>,
}
