//! Shared primitive types used across multiple WAAPI endpoints.

use serde::{Deserialize, Serialize, Serializer};

use crate::api::generated::ObjectType;

// ─────────────────────────────────────────────────────────────────────────────
// Object and platform references
// ─────────────────────────────────────────────────────────────────────────────

/// Identifies an existing Wwise object.
#[derive(Debug, Clone)]
pub enum ObjectRef {
    /// A GUID, e.g. `"{FF59687C-48CF-4385-B1C5-CE84B0A63880}"`.
    Guid(String),
    /// A backslash-separated hierarchy path.
    Path(String),
    /// A unique qualified object name.
    Name(String),
}

impl ObjectRef {
    /// Creates an [`ObjectRef::Guid`] from a GUID string.
    pub fn guid(s: impl Into<String>) -> Self {
        ObjectRef::Guid(s.into())
    }
    /// Creates an [`ObjectRef::Path`] from a backslash-separated hierarchy path.
    pub fn path(s: impl Into<String>) -> Self {
        ObjectRef::Path(s.into())
    }
    /// Creates an [`ObjectRef::Name`] from an object name.
    pub fn name(s: impl Into<String>) -> Self {
        ObjectRef::Name(s.into())
    }
}

impl From<String> for ObjectRef {
    fn from(s: String) -> Self {
        if s.starts_with('{') && s.ends_with('}') {
            ObjectRef::Guid(s)
        } else if s.starts_with('\\') {
            ObjectRef::Path(s)
        } else {
            ObjectRef::Name(s)
        }
    }
}

impl From<&str> for ObjectRef {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<&String> for ObjectRef {
    fn from(s: &String) -> Self {
        Self::from(s.as_str())
    }
}

impl Serialize for ObjectRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = match self {
            ObjectRef::Guid(s) | ObjectRef::Path(s) | ObjectRef::Name(s) => s,
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for ObjectRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ObjectRefVisitor;

        impl serde::de::Visitor<'_> for ObjectRefVisitor {
            type Value = ObjectRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Wwise object GUID, path, or name string")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(ObjectRef::from(v))
            }

            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ObjectRef::from(v))
            }
        }

        deserializer.deserialize_string(ObjectRefVisitor)
    }
}

/// Identifies a Wwise platform for platform-specific property overrides.
#[derive(Debug, Clone)]
pub enum PlatformRef {
    /// Platform GUID (`{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}`).
    Guid(String),
    /// Platform name (e.g. `"Windows"`, `"Mac"`).
    Name(String),
}

impl PlatformRef {
    /// Creates a [`PlatformRef::Guid`] from a platform GUID string.
    pub fn guid(s: impl Into<String>) -> Self {
        PlatformRef::Guid(s.into())
    }
    /// Creates a [`PlatformRef::Name`] from a platform name.
    pub fn name(s: impl Into<String>) -> Self {
        PlatformRef::Name(s.into())
    }
}

impl From<String> for PlatformRef {
    fn from(s: String) -> Self {
        if s.starts_with('{') && s.ends_with('}') {
            PlatformRef::Guid(s)
        } else {
            PlatformRef::Name(s)
        }
    }
}

impl From<&str> for PlatformRef {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<&String> for PlatformRef {
    fn from(s: &String) -> Self {
        Self::from(s.as_str())
    }
}

impl Serialize for PlatformRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = match self {
            PlatformRef::Guid(s) | PlatformRef::Name(s) => s,
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for PlatformRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PlatformRefVisitor;

        impl serde::de::Visitor<'_> for PlatformRefVisitor {
            type Value = PlatformRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Wwise platform GUID or name string")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(PlatformRef::from(v))
            }

            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PlatformRef::from(v))
            }
        }

        deserializer.deserialize_string(PlatformRefVisitor)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3D position helpers for soundengine spatial audio endpoints
// ─────────────────────────────────────────────────────────────────────────────

/// A 3D vector used for position and orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Z coordinate.
    pub z: f64,
}

#[allow(non_upper_case_globals)]
impl Vector3 {
    /// Creates a new [`Vector3`] with coordinates `(x, y, z)`.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// (0, 0, 1)
    pub const forward: Self = Self::new(0.0, 0.0, 1.0);
    /// (0, 1, 0)
    pub const up: Self = Self::new(0.0, 1.0, 0.0);
    /// (1, 0, 0)
    pub const right: Self = Self::new(1.0, 0.0, 0.0);
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        v * self
    }
}

impl std::ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = *self * scalar;
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl std::ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = *self / scalar;
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

/// A complete game-object position: position plus front/top orientation vectors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Orientation of the front vector (must be normalized and orthogonal to `orientation_top`).
    #[serde(rename = "orientationFront")]
    pub orientation_front: Vector3,
    /// Orientation of the top vector (must be normalized and orthogonal to `orientation_front`).
    #[serde(rename = "orientationTop")]
    pub orientation_top: Vector3,
    /// 3D position of the game object.
    pub position: Vector3,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            position: Vector3::default(),
            orientation_front: Vector3::forward,
            orientation_top: Vector3::up,
        }
    }
}

impl Position {
    /// Position at `position` with default forward/up orientation.
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            ..Self::default()
        }
    }

    /// Full control over position and orientation.
    pub fn with_orientations(
        position: Vector3,
        orientation_front: Vector3,
        orientation_top: Vector3,
    ) -> Self {
        Self {
            position,
            orientation_front,
            orientation_top,
        }
    }

    /// Position at the origin with default forward/up orientation.
    pub fn origin() -> Self {
        Self::default()
    }
}

impl From<Vector3> for Position {
    fn from(position: Vector3) -> Self {
        Self::new(position)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared enums
// ─────────────────────────────────────────────────────────────────────────────

/// Action to take when a child with the same name already exists.
#[derive(Debug, Clone)]
pub enum OnNameConflict {
    /// The new object receives a unique name automatically by appending a number.
    Rename,
    /// The existing object is deleted along with its children and replaced by the new object.
    Replace,
    /// The call fails and returns an error.
    Fail,
    /// The existing object is kept and its properties, lists, and children are merged with the new object.
    Merge,
}

impl OnNameConflict {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            OnNameConflict::Rename => "rename",
            OnNameConflict::Replace => "replace",
            OnNameConflict::Fail => "fail",
            OnNameConflict::Merge => "merge",
        }
    }
}

impl Serialize for OnNameConflict {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OnNameConflict {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct OnNameConflictVisitor;

        impl serde::de::Visitor<'_> for OnNameConflictVisitor {
            type Value = OnNameConflict;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("one of \"rename\", \"replace\", \"fail\", or \"merge\"")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    "rename" => Ok(OnNameConflict::Rename),
                    "replace" => Ok(OnNameConflict::Replace),
                    "fail" => Ok(OnNameConflict::Fail),
                    "merge" => Ok(OnNameConflict::Merge),
                    _ => Err(E::custom(format!("unknown OnNameConflict: {s}"))),
                }
            }
        }

        deserializer.deserialize_str(OnNameConflictVisitor)
    }
}

/// Action to take when an object already has items in a named list.
#[derive(Debug, Clone)]
pub enum ListMode {
    /// Remove all existing items in the list and replace them with the provided items.
    ReplaceAll,
    /// Append the provided items to the existing items in the list.
    Append,
}

impl ListMode {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            ListMode::ReplaceAll => "replaceAll",
            ListMode::Append => "append",
        }
    }
}

impl Serialize for ListMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ListMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ListModeVisitor;

        impl serde::de::Visitor<'_> for ListModeVisitor {
            type Value = ListMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("one of \"replaceAll\" or \"append\"")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    "replaceAll" => Ok(ListMode::ReplaceAll),
                    "append" => Ok(ListMode::Append),
                    _ => Err(E::custom(format!("unknown ListMode: {s}"))),
                }
            }
        }

        deserializer.deserialize_str(ListModeVisitor)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Property values and child object definitions
// ─────────────────────────────────────────────────────────────────────────────

struct AtPrefixedKey<'a>(&'a str);

impl Serialize for AtPrefixedKey<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bare = self.0.strip_prefix('@').unwrap_or(self.0);
        serializer.collect_str(&format_args!("@{bare}"))
    }
}

/// A new Wwise object to create, nested under a parent via `children` or a
/// named list via an `@`-prefixed field.
#[derive(Debug, Clone)]
pub struct ChildObject {
    /// The Wwise object type, e.g. [`ObjectType::Sound`], [`ObjectType::Event`], [`ObjectType::RandomSequenceContainer`].
    pub object_type: ObjectType,
    /// Name of the new object.
    pub name: String,
    /// Optional notes or comments for the new object.
    pub notes: Option<String>,
    /// Optional platform override used when setting properties on the new object.
    pub platform: Option<PlatformRef>,
    /// Nested children to create under this object (recursive).
    pub children: Vec<ChildObject>,
    /// `@PropertyName` / `@ReferenceName` / `@ListName` fields.
    /// Keys are bare names without the `@` prefix - it is added during serialization.
    pub properties: Vec<(String, PropertyValue)>,
}

impl Serialize for ChildObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let len = 2
            + usize::from(self.notes.is_some())
            + usize::from(self.platform.is_some())
            + usize::from(!self.children.is_empty())
            + self.properties.len();
        let mut ser = serializer.serialize_map(Some(len))?;
        ser.serialize_entry("type", &self.object_type)?;
        ser.serialize_entry("name", &self.name)?;
        if let Some(notes) = &self.notes {
            ser.serialize_entry("notes", notes)?;
        }
        if let Some(platform) = &self.platform {
            ser.serialize_entry("platform", platform)?;
        }
        if !self.children.is_empty() {
            ser.serialize_entry("children", &self.children)?;
        }
        for (key, value) in &self.properties {
            ser.serialize_entry(&AtPrefixedKey(key), value)?;
        }
        ser.end()
    }
}

impl ChildObject {
    /// Creates a new [`ChildObject`] of the given `object_type` and `name`.
    pub fn new(object_type: ObjectType, name: impl Into<String>) -> Self {
        ChildObject {
            object_type,
            name: name.into(),
            notes: None,
            platform: None,
            children: Vec::new(),
            properties: Vec::new(),
        }
    }

    /// Sets the notes or comments for the new object.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Sets the platform used when applying property values to the new object.
    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    /// Add a property, reference, or list field.
    /// Accepts a [`crate::Property`] variant (e.g. `Property::Volume`) or a property name (`"Volume"`).
    pub fn property(mut self, name: impl AsRef<str>, value: impl Into<PropertyValue>) -> Self {
        let bare = name.as_ref().strip_prefix('@').unwrap_or(name.as_ref());
        self.properties.push((bare.to_string(), value.into()));
        self
    }

    /// Appends a nested [`ChildObject`] to create under this object.
    pub fn child(mut self, child: ChildObject) -> Self {
        self.children.push(child);
        self
    }

    /// Appends multiple nested [`ChildObject`] entries to create under this object.
    pub fn children(mut self, children: impl Into<Vec<ChildObject>>) -> Self {
        self.children.extend(children.into());
        self
    }
}

/// The value of a Wwise property or reference, set via an `@`-prefixed field.
#[derive(Debug, Clone)]
pub enum PropertyValue {
    /// A numeric property (WAAPI accepts both float and integer).
    Number(f64),
    /// A boolean property.
    Boolean(bool),
    /// A string value or object reference path/GUID/name.
    String(String),
    /// An array of string references, e.g. a list of GUIDs.
    StringList(Vec<String>),
    /// Create a new object and assign it to this reference slot.
    Object(Box<ChildObject>),
    /// Create new objects and insert them into this named list.
    ObjectList(Vec<ChildObject>),
}

impl PropertyValue {
    /// Creates a [`PropertyValue::Number`].
    pub fn number(n: impl Into<f64>) -> Self {
        PropertyValue::Number(n.into())
    }

    /// Creates a [`PropertyValue::Boolean`].
    pub fn boolean(b: bool) -> Self {
        PropertyValue::Boolean(b)
    }

    /// Creates a [`PropertyValue::String`] from a string or `&str`.
    pub fn string(s: impl Into<String>) -> Self {
        PropertyValue::String(s.into())
    }

    /// Creates a [`PropertyValue::StringList`] from an iterator of strings or `&str` slices.
    pub fn string_list<I, S>(list: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        PropertyValue::StringList(list.into_iter().map(Into::into).collect())
    }

    /// Creates a [`PropertyValue::Object`] from a [`ChildObject`].
    pub fn object(obj: ChildObject) -> Self {
        PropertyValue::Object(Box::new(obj))
    }

    /// Creates a [`PropertyValue::ObjectList`] from a list of [`ChildObject`] entries.
    pub fn object_list(list: impl Into<Vec<ChildObject>>) -> Self {
        PropertyValue::ObjectList(list.into())
    }
}

impl From<f64> for PropertyValue {
    fn from(n: f64) -> Self {
        PropertyValue::Number(n)
    }
}

impl From<f32> for PropertyValue {
    fn from(n: f32) -> Self {
        PropertyValue::Number(f64::from(n))
    }
}

impl From<i64> for PropertyValue {
    fn from(n: i64) -> Self {
        PropertyValue::Number(n as f64)
    }
}

impl From<i32> for PropertyValue {
    fn from(n: i32) -> Self {
        PropertyValue::Number(f64::from(n))
    }
}

impl From<bool> for PropertyValue {
    fn from(b: bool) -> Self {
        PropertyValue::Boolean(b)
    }
}

impl From<String> for PropertyValue {
    fn from(s: String) -> Self {
        PropertyValue::String(s)
    }
}

impl From<&str> for PropertyValue {
    fn from(s: &str) -> Self {
        PropertyValue::String(s.to_string())
    }
}

impl From<&String> for PropertyValue {
    fn from(s: &String) -> Self {
        PropertyValue::String(s.clone())
    }
}

impl From<ChildObject> for PropertyValue {
    fn from(obj: ChildObject) -> Self {
        PropertyValue::Object(Box::new(obj))
    }
}

impl From<Vec<ChildObject>> for PropertyValue {
    fn from(list: Vec<ChildObject>) -> Self {
        PropertyValue::ObjectList(list)
    }
}

impl Serialize for PropertyValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PropertyValue::Number(n) => serializer.serialize_f64(*n),
            PropertyValue::Boolean(b) => serializer.serialize_bool(*b),
            PropertyValue::String(s) => serializer.serialize_str(s),
            PropertyValue::StringList(v) => v.serialize(serializer),
            PropertyValue::Object(obj) => obj.serialize(serializer),
            PropertyValue::ObjectList(v) => v.serialize(serializer),
        }
    }
}
