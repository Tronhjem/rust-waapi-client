// Generated bindings for ak.wwise.core.soundbank.generate



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.soundbank.generate`.
///
/// Generate a list of SoundBanks with the import definition specified in the WAAPI request. If you do not write the SoundBanks to disk, subscribe to `ak.wwise.core.soundbank.generated` to receive SoundBank structure info and the bank data as base64. Note: This is a synchronous operation.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreSoundbankGenerateArgs {
    /// List of platforms to generate. If you don't specify any platforms, all the platforms will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    /// List of languages to generate. If you don't specify any languages, all the languages will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<LanguageRef>>,
    /// List of user-defined SoundBanks to generate. If the array is empty then all user-defined SoundBanks are generated. Note that auto-defined SoundBanks cannot be specified, and all auto-defined SoundBanks are generated regardless of this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soundbanks: Option<Vec<SoundBankInfo>>,
    /// By default, if you don't specify any languages all languages will be generated. If you set this parameter to true, no localized SoundBank will be generated.
    #[serde(rename = "skipLanguages", skip_serializing_if = "Option::is_none")]
    pub skip_languages: Option<bool>,
    /// If you don't use rebuildSoundBanks, use this option to force a rebuild of the Init bank for each specified platform.
    #[serde(rename = "rebuildInitBank", skip_serializing_if = "Option::is_none")]
    pub rebuild_init_bank: Option<bool>,
    /// Will rebuild all SoundBanks if true. If you want to clear the converted media as well, use clearAudioFileCache parameter. Default value: false.
    #[serde(rename = "rebuildSoundBanks", skip_serializing_if = "Option::is_none")]
    pub rebuild_sound_banks: Option<bool>,
    /// Deletes the content of the Wwise audio file cache folder prior to converting source files and generating SoundBanks, which ensures that all source files are reconverted. Note that the whole cache is cleared, for all platforms, regardless of the platforms arguments. Default value: false.
    #[serde(rename = "clearAudioFileCache", skip_serializing_if = "Option::is_none")]
    pub clear_audio_file_cache: Option<bool>,
    /// Use the normal SoundBank generation process and write the sound bank and info file to disk. Default value: false.
    #[serde(rename = "writeToDisk", skip_serializing_if = "Option::is_none")]
    pub write_to_disk: Option<bool>,
}

impl CoreSoundbankGenerateArgs {
    /// Creates a new [`CoreSoundbankGenerateArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// List of platforms to generate. If you don't specify any platforms, all the platforms will be generated.
    pub fn platforms(mut self, platforms: impl Into<Vec<String>>) -> Self {
        self.platforms = Some(platforms.into());
        self
    }

    /// List of languages to generate. If you don't specify any languages, all the languages will be generated.
    pub fn languages(mut self, languages: impl Into<Vec<LanguageRef>>) -> Self {
        self.languages = Some(languages.into());
        self
    }

    /// List of user-defined SoundBanks to generate. If the array is empty then all user-defined SoundBanks are generated. Note that auto-defined SoundBanks cannot be specified, and all auto-defined SoundBanks are generated regardless of this parameter.
    pub fn soundbanks(mut self, soundbanks: impl Into<Vec<SoundBankInfo>>) -> Self {
        self.soundbanks = Some(soundbanks.into());
        self
    }

    /// By default, if you don't specify any languages all languages will be generated. If you set this parameter to true, no localized SoundBank will be generated.
    pub fn skip_languages(mut self, skip_languages: impl Into<bool>) -> Self {
        self.skip_languages = Some(skip_languages.into());
        self
    }

    /// If you don't use rebuildSoundBanks, use this option to force a rebuild of the Init bank for each specified platform.
    pub fn rebuild_init_bank(mut self, rebuild_init_bank: impl Into<bool>) -> Self {
        self.rebuild_init_bank = Some(rebuild_init_bank.into());
        self
    }

    /// Will rebuild all SoundBanks if true. If you want to clear the converted media as well, use clearAudioFileCache parameter. Default value: false.
    pub fn rebuild_sound_banks(mut self, rebuild_sound_banks: impl Into<bool>) -> Self {
        self.rebuild_sound_banks = Some(rebuild_sound_banks.into());
        self
    }

    /// Deletes the content of the Wwise audio file cache folder prior to converting source files and generating SoundBanks, which ensures that all source files are reconverted. Note that the whole cache is cleared, for all platforms, regardless of the platforms arguments. Default value: false.
    pub fn clear_audio_file_cache(mut self, clear_audio_file_cache: impl Into<bool>) -> Self {
        self.clear_audio_file_cache = Some(clear_audio_file_cache.into());
        self
    }

    /// Use the normal SoundBank generation process and write the sound bank and info file to disk. Default value: false.
    pub fn write_to_disk(mut self, write_to_disk: impl Into<bool>) -> Self {
        self.write_to_disk = Some(write_to_disk.into());
        self
    }
}

/// Response from `ak.wwise.core.soundbank.generate`.
///
/// Generate a list of SoundBanks with the import definition specified in the WAAPI request. If you do not write the SoundBanks to disk, subscribe to `ak.wwise.core.soundbank.generated` to receive SoundBank structure info and the bank data as base64. Note: This is a synchronous operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSoundbankGenerateResponse {
    /// First error message from the SoundBank generation log.
    #[serde(default)]
    pub error: Option<String>,
    /// The SoundBank generation log. The log is empty when used in WwiseConsole.
    #[serde(default)]
    pub logs: Vec<LogCentralItem>,
}

