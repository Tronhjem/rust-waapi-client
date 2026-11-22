//! Rust client library for the Wwise Authoring API (WAAPI) over WAMP/WebSocket.

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
