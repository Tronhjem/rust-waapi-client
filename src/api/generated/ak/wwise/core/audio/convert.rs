// Generated bindings for ak.wwise.core.audio.convert

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.convert`.
///
/// Creates a converted audio file. When errors occur, this function returns a list of messages with corresponding levels of severity.
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
    /// Creates a new [`CoreAudioConvertArgs`] with the required fields.
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

impl<T0: Into<Vec<ObjectRef>>, T1: Into<Vec<PlatformRef>>, T2: Into<Vec<LanguageRef>>>
    From<(T0, T1, T2)> for CoreAudioConvertArgs
{
    fn from((objects, platforms, languages): (T0, T1, T2)) -> Self {
        Self::new(objects.into(), platforms.into(), languages.into())
    }
}

/// Response from `ak.wwise.core.audio.convert`.
///
/// Creates a converted audio file. When errors occur, this function returns a list of messages with corresponding levels of severity.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreAudioConvertResponse {
    /// List of logged entries with associated messages and severities.
    #[serde(default)]
    pub errors: Vec<AudioConvertError>,
}

/// Module-scoped alias for [`CoreAudioConvertArgs`].
pub type ConvertArgs = CoreAudioConvertArgs;

/// Module-scoped alias for [`CoreAudioConvertResponse`].
pub type ConvertResponse = CoreAudioConvertResponse;
