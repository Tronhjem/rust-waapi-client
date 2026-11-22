//! Rust client library for the Wwise Authoring API (WAAPI) over WAMP/WebSocket.
//!
//! `waapi-client` provides a compile-time typed, schema-driven Rust API for
//! Audiokinetic Wwise's Authoring API (supporting Wwise 2024 and Wwise 2025+
//! via multi-version union generation), along with a raw JSON escape hatch.
//!
//! # Quick Start (Chained Typed API)
//!
//! The chained API on [`WaapiClient`] mirrors WAAPI URI dot-notation
//! (e.g., `ak.wwise.core.object.get` -> `client.wwise().core().object().get(...)`).
//! Intermediate namespace clients borrow `&mut self`, so they can be stored and
//! reused across multiple calls:
//!
//! ```no_run
//! use waapi_client::api::ak::wwise::core::object::{
//!     CoreObjectCreateArgs, CoreObjectGetArgs, CoreObjectGetOptions,
//! };
//! use waapi_client::api::{ObjectQuery, ObjectReturnField, OnNameConflict};
//! use waapi_client::{ObjectRef, ObjectType, Property, WaapiClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = WaapiClient::new(None).await?;
//!
//!     // Intermediate client handles can be stored and reused:
//!     let mut wwise = client.wwise();
//!     let info = wwise.core().get_info().await?;
//!     println!("Connected to {} {}", info.display_name, info.version.display_name);
//!
//!     // Query objects with typed return fields and properties:
//!     let response = wwise
//!         .core()
//!         .object()
//!         .get(
//!             CoreObjectGetArgs::new(ObjectQuery::from(r"\Containers\Default Work Unit")),
//!             CoreObjectGetOptions::new().return_fields([
//!                 ObjectReturnField::Id,
//!                 ObjectReturnField::Name,
//!                 ObjectReturnField::Type,
//!                 ObjectReturnField::Property(Property::Volume),
//!             ]),
//!         )
//!         .await?;
//!
//!     for obj in &response.objects {
//!         println!("{:?} ({:?}): {:?}", obj.name(), obj.object_type(), obj.id());
//!     }
//!
//!     // Create a Sound object with typed ObjectType and Property:
//!     let created = wwise
//!         .core()
//!         .object()
//!         .create(
//!             CoreObjectCreateArgs::new(
//!                 ObjectRef::path(r"\Containers\Default Work Unit"),
//!                 ObjectType::Sound,
//!                 "MyNewSound".to_string(),
//!             )
//!             .on_name_conflict(OnNameConflict::Replace)
//!             .property(Property::Volume, -6.0),
//!         )
//!         .await?;
//!     println!("Created sound id: {:?}", created.id);
//!
//!     client.shutdown().await?;
//!     Ok(())
//! }
//! ```
//!
//! # Typed Topic Subscriptions
//!
//! Every WAAPI subscription topic has a generated `<Base>Payload` struct and a
//! `subscribe_*` method on its namespace client:
//!
//! ```no_run
//! use waapi_client::api::ak::wwise::core::object::CoreObjectCreatedOptions;
//! use waapi_client::api::ObjectReturnField;
//! use waapi_client::WaapiClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = WaapiClient::new(None).await?;
//!
//!     let (sub_id, mut rx) = client
//!         .wwise()
//!         .core()
//!         .object()
//!         .subscribe_created(Some(
//!             CoreObjectCreatedOptions::new()
//!                 .return_(vec![ObjectReturnField::Id, ObjectReturnField::Name]),
//!         ))
//!         .await?;
//!
//!     if let Some(payload) = rx.recv().await {
//!         println!("Created object: {:?}", payload.object.name());
//!     }
//!
//!     client.unsubscribe(sub_id).await?;
//!     client.shutdown().await?;
//!     Ok(())
//! }
//! ```
//!
//! # Raw JSON Escape Hatch
//!
//! For dynamic queries or custom payloads, use [`WaapiClient::call`] with
//! [`WaapiFunction`] and [`json!`]:
//!
//! ```no_run
//! use waapi_client::{json, WaapiClient, WaapiFunction};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = WaapiClient::new(None).await?;
//!     let res = client
//!         .call(
//!             WaapiFunction::ak_wwise_core_object_get,
//!             Some(json!({ "from": { "ofType": ["Sound"] } })),
//!             Some(json!({ "return": ["id", "name"] })),
//!         )
//!         .await?;
//!     println!("Raw result: {res}");
//!     client.shutdown().await?;
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

pub mod api;
/// Re-exports of `serde_json::Map` and `serde_json::Value`.
pub mod types;
/// Auto-generated WAAPI RPC function URI constants and [`WaapiFunction`] enum.
pub mod waapi_function_api;
/// Auto-generated WAAPI subscription topic URI constants and [`WaapiTopic`] enum.
pub mod waapi_topics_api;

mod waapi_client;
mod wamp;

pub use serde_json;
pub use waapi_client::{Event, WaapiClient, WaapiError, WaapiResult};

/// Convenience wrapper around `serde_json::json!`.
///
/// This macro is re-exported from `waapi_client` so applications can build
/// raw WAAPI kwargs/options without adding a direct `serde_json` dependency.
#[macro_export]
macro_rules! json {
    ($($json:tt)*) => {
        $crate::serde_json::json!($($json)*)
    };
}
pub use api::{ChildObject, GameObjectRef, ObjectRef, ObjectType, Position, Property, Vector3};
pub use waapi_function_api::WaapiFunction;
pub use waapi_topics_api::WaapiTopic;
