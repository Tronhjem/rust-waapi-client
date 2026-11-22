//! Typed WAAPI bindings, shared primitives, and chained namespace clients.

/// Auto-generated typed WAAPI bindings, object types, properties, and namespace clients.
pub mod generated;
/// Hand-written shared primitives for object references, 3D vectors/positions, and child object creation.
pub mod primitives;

pub use generated::ak;
pub use generated::primitives::{
    ActionOnEventType, AkCurveInterpolation, AttenuationCurveType, AttenuationCurveUse,
    AudioConvertError, AuxSendValue, BlendEdge, BlendFadeMode, BlendTrackAssignment, CurvePoint2D,
    CurveShape, EventRef, ExternalSourceInfo, GameObjectRef, ImportDefaults, ImportItem,
    ImportOperation, LanguageRef, LogCentralItem, LogChannel, LogSeverity, MultiPosition,
    MultiPositionType, ObjectQuery, ObjectReturnField, ObjectReturnOptions, OnCurveUpdate,
    PasteMode, ProjectDirectoriesInfo, ProjectLanguageInfo, ProjectMigrationAction,
    ProjectPlatformInfo, PropertyInfoSupports, RtpcRef, RtpcSupportMode, SoundBankInclusion,
    SoundBankInclusionFilter, SoundBankInclusionOperation, SoundBankInclusionType, SoundBankInfo,
    SoundEngineObjectRef, SwitchContainerAssignment, Transform, TransformSelect, TransportAction,
    TransportObject, TransportState, WwiseObject, WwiseObjectRef, WwiseObjectTypeInfo,
};
pub use generated::{ObjectType, Property};
pub use primitives::{
    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, Position, PropertyValue, Vector3,
};
