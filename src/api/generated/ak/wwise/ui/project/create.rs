// Generated bindings for ak.wwise.ui.project.create

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.project.create`.
///
/// Creates, saves and opens new empty project, specified by path and platform. The project has no factory setting WorkUnit. Please see `ak.wwise.core.project.loaded` for further explanations on how to be notified when the operation has completed.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiProjectCreateArgs {
    /// The path to the project WPROJ file. The path must use the same name for the WPROJ and the parent directory folder. For example: c:\\Projects\\MYPROJECT\\MYPROJECT.wproj. For using WAAPI on Mac, please see the WAAPI Mac path documentation.
    pub path: String,
    /// Specifies the platform or platforms supported by the new project. If not specified, only Windows is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<PlatformsToCreate>,
    /// Array of languages to creates for this project. If not specified, the English(US) language is created. When multiple languages are specified, the first one becomes the default language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

impl UiProjectCreateArgs {
    /// Creates a new [`UiProjectCreateArgs`] with the required fields.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            platforms: None,
            languages: None,
        }
    }

    /// Specifies the platform or platforms supported by the new project. If not specified, only Windows is used.
    pub fn platforms(mut self, platforms: PlatformsToCreate) -> Self {
        self.platforms = Some(platforms);
        self
    }

    /// Array of languages to creates for this project. If not specified, the English(US) language is created. When multiple languages are specified, the first one becomes the default language.
    pub fn languages(mut self, languages: impl Into<Vec<String>>) -> Self {
        self.languages = Some(languages.into());
        self
    }
}
