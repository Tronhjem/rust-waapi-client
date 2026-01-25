use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum WaapiValue {
    Dict(HashMap<String, WaapiValue>),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    String(String),
    List(Vec<WaapiValue>),
    Boolean(bool),
}

impl From<WaapiValue> for wampire::Value {
    fn from(value: WaapiValue) -> Self {
        match value {
            WaapiValue::Dict(map) => {
                wampire::Value::Dict(map.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            WaapiValue::Integer(i) => wampire::Value::Integer(i),
            WaapiValue::UnsignedInteger(u) => wampire::Value::UnsignedInteger(u),
            WaapiValue::Float(f) => wampire::Value::Float(f),
            WaapiValue::String(s) => wampire::Value::String(s),
            WaapiValue::List(list) => {
                wampire::Value::List(list.into_iter().map(|v| v.into()).collect())
            }
            WaapiValue::Boolean(b) => wampire::Value::Boolean(b),
        }
    }
}

impl From<wampire::Value> for WaapiValue {
    fn from(value: wampire::Value) -> Self {
        match value {
            wampire::Value::Dict(map) => {
                WaapiValue::Dict(map.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            wampire::Value::Integer(i) => WaapiValue::Integer(i),
            wampire::Value::UnsignedInteger(u) => WaapiValue::UnsignedInteger(u),
            wampire::Value::Float(f) => WaapiValue::Float(f),
            wampire::Value::String(s) => WaapiValue::String(s),
            wampire::Value::List(list) => {
                WaapiValue::List(list.into_iter().map(|v| v.into()).collect())
            }
            wampire::Value::Boolean(b) => WaapiValue::Boolean(b),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Id,
    Name,
    Notes,
    Type,
    PluginName,
    ShortId,
    ClassId,
    Category,
    FilePath,
    Workunit,
    Parent,
    Owner,
    Path,
    IsPlayable,
    ChildrenCount,
    TotalSize,
    MediaSize,
    ObjectSize,
    StructureSize,
    SoundConvertedWemFilePath,
    SoundOriginalWavFilePath,
    SoundbankBnkFilePath,
    MusicTransitionRoot,
    MusicPlaylistRoot,
    AudioSourcePlaybackDuration,
    AudioSourceMaxDurationSource,
    AudioSourceTrimValues,
    AudioSourceMaxRadiusAttenuation,
    AudioSourceLanguage,
    WorkunitIsDefault,
    WorkunitType,
    WorkunitIsDirty,
    SwitchContainerChildContext,
    ConvertedWemFilePath,
    OriginalFilePath,
    OriginalRelativeFilePath,
    ConvertedFilePath,
    OriginalWavFilePath,
    SoundbankBnkFilePath2,
    MusicTransitionRoot2,
    MusicPlaylistRoot2,
    MusicTransitionObject,
    PlaybackDuration,
    Duration,
    MaxDurationSource,
    AudioSourceTrimValues2,
    MaxRadiusAttenuation,
    AudioSourceLanguage2,
    WorkunitIsDefault2,
    WorkunitType2,
    WorkunitIsDirty2,
    WorkunitIsLoaded,
    SwitchContainerChildContext2,
    IsExplicitMute,
    IsExplicitSolo,
    IsImplicitMute,
    IsImplicitSolo,
    IsIncluded,
    IsLanguageIncluded,
    Points,
    States,
    StateProperties,
    StateGroups,
    CustomStates,
    OriginalState,
    BlendTracks,
    SupportsStates,
    SupportsRandomizer,
    SwitchGroupGameParameter,
    HasEmptySwitchStateAssignment,
    ExtractEvents,
    ExtractStructures,
    ExtractMedia,
    SoundbanksReferencingEvent,
}

impl ReturnType {
    pub fn as_str(&self) -> &str {
        match self {
            ReturnType::Id => "id",
            ReturnType::Name => "name",
            ReturnType::Notes => "notes",
            ReturnType::Type => "type",
            ReturnType::PluginName => "pluginName",
            ReturnType::ShortId => "shortId",
            ReturnType::ClassId => "classId",
            ReturnType::Category => "category",
            ReturnType::FilePath => "filePath",
            ReturnType::Workunit => "workunit",
            ReturnType::Parent => "parent",
            ReturnType::Owner => "owner",
            ReturnType::Path => "path",
            ReturnType::IsPlayable => "isPlayable",
            ReturnType::ChildrenCount => "childrenCount",
            ReturnType::TotalSize => "totalSize",
            ReturnType::MediaSize => "mediaSize",
            ReturnType::ObjectSize => "objectSize",
            ReturnType::StructureSize => "structureSize",
            ReturnType::SoundConvertedWemFilePath => "sound:convertedWemFilePath",
            ReturnType::SoundOriginalWavFilePath => "sound:originalWavFilePath",
            ReturnType::SoundbankBnkFilePath => "soundbank:bnkFilePath",
            ReturnType::MusicTransitionRoot => "music:transitionRoot",
            ReturnType::MusicPlaylistRoot => "music:playlistRoot",
            ReturnType::AudioSourcePlaybackDuration => "audioSource:playbackDuration",
            ReturnType::AudioSourceMaxDurationSource => "audioSource:maxDurationSource",
            ReturnType::AudioSourceTrimValues => "audioSource:trimValues",
            ReturnType::AudioSourceMaxRadiusAttenuation => "audioSource:maxRadiusAttenuation",
            ReturnType::AudioSourceLanguage => "audioSource:language",
            ReturnType::WorkunitIsDefault => "workunit:isDefault",
            ReturnType::WorkunitType => "workunit:type",
            ReturnType::WorkunitIsDirty => "workunit:isDirty",
            ReturnType::SwitchContainerChildContext => "switchContainerChild:context",
            ReturnType::ConvertedWemFilePath => "convertedWemFilePath",
            ReturnType::OriginalFilePath => "originalFilePath",
            ReturnType::OriginalRelativeFilePath => "originalRelativeFilePath",
            ReturnType::ConvertedFilePath => "convertedFilePath",
            ReturnType::OriginalWavFilePath => "originalWavFilePath",
            ReturnType::SoundbankBnkFilePath2 => "soundbankBnkFilePath",
            ReturnType::MusicTransitionRoot2 => "musicTransitionRoot",
            ReturnType::MusicPlaylistRoot2 => "musicPlaylistRoot",
            ReturnType::MusicTransitionObject => "musicTransitionObject",
            ReturnType::PlaybackDuration => "playbackDuration",
            ReturnType::Duration => "duration",
            ReturnType::MaxDurationSource => "maxDurationSource",
            ReturnType::AudioSourceTrimValues2 => "audioSourceTrimValues",
            ReturnType::MaxRadiusAttenuation => "maxRadiusAttenuation",
            ReturnType::AudioSourceLanguage2 => "audioSourceLanguage",
            ReturnType::WorkunitIsDefault2 => "workunitIsDefault",
            ReturnType::WorkunitType2 => "workunitType",
            ReturnType::WorkunitIsDirty2 => "workunitIsDirty",
            ReturnType::WorkunitIsLoaded => "workunitIsLoaded",
            ReturnType::SwitchContainerChildContext2 => "switchContainerChildContext",
            ReturnType::IsExplicitMute => "isExplicitMute",
            ReturnType::IsExplicitSolo => "isExplicitSolo",
            ReturnType::IsImplicitMute => "isImplicitMute",
            ReturnType::IsImplicitSolo => "isImplicitSolo",
            ReturnType::IsIncluded => "isIncluded",
            ReturnType::IsLanguageIncluded => "isLanguageIncluded",
            ReturnType::Points => "points",
            ReturnType::States => "states",
            ReturnType::StateProperties => "stateProperties",
            ReturnType::StateGroups => "stateGroups",
            ReturnType::CustomStates => "customStates",
            ReturnType::OriginalState => "originalState",
            ReturnType::BlendTracks => "blendTracks",
            ReturnType::SupportsStates => "supportsStates",
            ReturnType::SupportsRandomizer => "suppportsRandomizer",
            ReturnType::SwitchGroupGameParameter => "switchGroupGameParameter",
            ReturnType::HasEmptySwitchStateAssignment => "hasEmptySwitchStateAssignment",
            ReturnType::ExtractEvents => "extractEvents",
            ReturnType::ExtractStructures => "extractStructures",
            ReturnType::ExtractMedia => "extractMedia",
            ReturnType::SoundbanksReferencingEvent => "soundbanksReferencingEvent",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WaapiOptions {
    return_types: Vec<ReturnType>,
}

impl WaapiOptions {
    pub fn new(return_types: Vec<ReturnType>) -> Self {
        Self { return_types }
    }

    pub fn with_return(return_types: &[ReturnType]) -> Self {
        Self {
            return_types: return_types.to_vec(),
        }
    }

    pub(crate) fn to_hashmap(&self) -> HashMap<String, WaapiValue> {
        let mut map = HashMap::new();
        let return_values: Vec<WaapiValue> = self
            .return_types
            .iter()
            .map(|rt| WaapiValue::String(rt.as_str().to_string()))
            .collect();
        map.insert("return".to_string(), WaapiValue::List(return_values));
        map
    }
}

#[derive(Debug, Clone, Default)]
pub struct WaapiArgs {
    args: HashMap<String, WaapiValue>,
}

impl WaapiArgs {
    pub fn new() -> Self {
        Self {
            args: HashMap::new(),
        }
    }

    pub fn insert(mut self, key: impl Into<String>, value: WaapiValue) -> Self {
        self.args.insert(key.into(), value);
        self
    }

    pub fn insert_mut(&mut self, key: impl Into<String>, value: WaapiValue) -> &mut Self {
        self.args.insert(key.into(), value);
        self
    }

    pub(crate) fn into_hashmap(self) -> HashMap<String, WaapiValue> {
        self.args
    }
}

impl From<WaapiArgs> for HashMap<String, WaapiValue> {
    fn from(args: WaapiArgs) -> Self {
        args.args
    }
}
