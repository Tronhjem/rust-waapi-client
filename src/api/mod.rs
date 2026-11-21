pub mod generated;
pub mod primitives;

pub use generated::ak;
pub use generated::primitives::{
    AttenuationCurveType, AttenuationCurveUse, CurvePoint2D, CurveShape, EventRef, GameObjectRef,
    ImportDefaults, ImportItem, ImportOperation, LanguageRef, LogSeverity, ObjectQuery,
    ObjectReturnField, ObjectReturnOptions, PasteMode, ProjectDirectoriesInfo, ProjectLanguageInfo,
    ProjectMigrationAction, ProjectPlatformInfo, RtpcRef, SoundEngineObjectRef,
    SwitchContainerAssignment, Transform, TransformSelect, TransportAction, TransportObject,
    TransportState, WwiseObject, WwiseObjectRef,
};
pub use generated::{ObjectType, Property};
pub use primitives::{
    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, Position, PropertyValue, Vector3,
};
