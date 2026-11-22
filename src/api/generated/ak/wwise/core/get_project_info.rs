// Generated bindings for ak.wwise.core.getProjectInfo

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.getProjectInfo`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreGetProjectInfoResponse {
    /// Project id.
    pub id: String,
    /// The Project name. Note that the Project name can differ from the WPROJ file name, if the file was renamed after Project creation.
    pub name: String,
    /// The absolute path to the WPROJ file.
    pub path: String,
    /// Array of the Platforms defined in the project.
    pub platforms: Vec<ProjectPlatformInfo>,
    /// Array of the Languages defined in the project.
    pub languages: Vec<ProjectLanguageInfo>,
    /// Collection of directories used by Wwise.
    pub directories: ProjectDirectoriesInfo,
    /// The complete text from the Wwise titlebar.
    #[serde(rename = "displayTitle")]
    pub display_title: String,
    /// True if the Project or any of the Work Units have unsaved changes.
    #[serde(rename = "isDirty")]
    pub is_dirty: bool,
    /// The current Language set in the user interface.
    #[serde(rename = "currentLanguageId")]
    pub current_language_id: String,
    /// The reference Language set in the Language settings.
    #[serde(rename = "referenceLanguageId")]
    pub reference_language_id: String,
    /// The current Platform set in the user interface.
    #[serde(rename = "currentPlatformId")]
    pub current_platform_id: String,
    /// The default Conversion Settings object used in the Project.
    #[serde(rename = "defaultConversion")]
    pub default_conversion: WwiseObjectRef,
}
