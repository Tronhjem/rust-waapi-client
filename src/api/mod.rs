//! Typed WAAPI bindings, shared primitives, and chained namespace clients.

/// Auto-generated typed WAAPI bindings, object types, properties, and namespace clients.
pub mod generated;
/// Hand-written shared primitives for object references, 3D vectors/positions, and child object creation.
pub mod primitives;

pub use generated::primitives::{
    ActionOnEventType, AddOnCommand, AkCurveInterpolation, AttenuationCurveType,
    AttenuationCurveUse, AudioConvertError, AudioImportLogItem, AudioImportedOperation,
    AudioObjectMetadata, AudioObjectReturnField, AuxSendValue, BlendEdge, BlendFadeMode,
    BlendTrackAssignment, BusReturnField, CaptureLogSeverity, CaptureLogType, CommandContextMenu,
    CommandMainMenu, CommandStartMode, ConnectedConsoleInfo, CursorMovePosition, CursorTime,
    CurvePoint2D, CurveShape, DefaultWorkUnitInfo, DirectoriesInfo, EventRef, ExternalSourceInfo,
    GameObjectRef, ImportDefaults, ImportItem, ImportLogEntry, ImportOperation, LanguageRef,
    LogCentralItem, LogChannel, LogSeverity, MediaPoolFilterEntry, MediaPoolFilterOperator,
    MediaPoolFilterType, MeterChannel, MeterConfigurationType, MeterSource, MultiPosition,
    MultiPositionType, ObjectQuery, ObjectReturnField, ObjectReturnOptions, OnCurveUpdate,
    PasteMode, PlatformTemplateEntry, PlatformsToCreate, ProfilerAudioObject, ProfilerBus,
    ProfilerCpuUsageItem, ProfilerCursor, ProfilerDataType, ProfilerDataTypeEntry,
    ProfilerGameObject, ProfilerLoadedMedia, ProfilerMeter, ProfilerPerformanceCounter,
    ProfilerRtpc, ProfilerStreamedMedia, ProfilerVoice, ProjectDirectoriesInfo,
    ProjectLanguageInfo, ProjectMigrationAction, ProjectPlatformInfo, PropertyDataMeaning,
    PropertyDependency, PropertyDependencyCondition, PropertyDependencyType, PropertyDisplayInfo,
    PropertyInfoSupports, PropertyRestriction, PropertyUiDisplayAs, PropertyUiInfo,
    PropertyUiValueInfo, RemoteConsoleInfo, RtpcRef, RtpcSupportMode, ScreenRect,
    SoundBankGeneratedBankData, SoundBankInclusion, SoundBankInclusionFilter,
    SoundBankInclusionOperation, SoundBankInclusionType, SoundBankInfo, SoundBankPluginInfo,
    SoundBankPluginLib, SoundEngineObjectRef, SourceControlFileFilter, SourceControlReturnField,
    SourceControlSourceFile, SourceControlStatus, StringOrVec, StructureChange,
    StructureChangeType, StructureChangedObject, StructureNameChange, StructureNotesChange,
    StructureOwnerChange, StructureParentChange, SwitchContainerAssignment, ToneBitDepth,
    ToneChannelConfig, ToneWaveform, Transform, TransformSelect, TransportAction, TransportObject,
    TransportState, VersionInfo, ViewInstance, ViewTypeInfo, VoiceContributionObject,
    VoiceContributionParameter, VoiceContributions, VoiceReturnField, WavMarker,
    WidgetAttributeEntry, WidgetStateQuery, WidgetStateResult, WidgetStateUpdate, WindowSize,
    WwiseObject, WwiseObjectRef, WwiseObjectTypeInfo,
};
pub use generated::{ObjectType, Property};
pub use generated::{ak, clients};
pub use primitives::{
    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, Position, PropertyValue, Vector3,
};
