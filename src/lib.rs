//! Rust client library for the Wwise Authoring API (WAAPI) over WAMP/WebSocket.
//!
//! `wwise-waapi` provides a compile-time typed, schema-driven Rust API for
//! Audiokinetic Wwise's Authoring API (supporting Wwise 2024 and Wwise 2025+
//! via multi-version union generation), along with a raw JSON escape hatch.
//!
//! # Installation
//!
//! Add `wwise-waapi` and `tokio` (for the `#[tokio::main]` async runtime) to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wwise-waapi = "0.1"
//! tokio = { version = "1", features = ["rt-multi-thread", "macros"] } # or features = ["full"]
//! ```
//!
//! # Quick Start (Chained Typed API)
//!
//! The chained API on [`WaapiClient`] mirrors WAAPI URI dot-notation
//! (e.g., `ak.wwise.core.object.get` -> `client.object().get(...)` or
//! `client.wwise().core().object().get(...)`).
//! Intermediate namespace clients borrow `&self` and implement `Copy`, so they
//! can be stored, copied, and reused concurrently across multiple calls:
//!
//! ```no_run
//! use wwise_waapi::api::ak::wwise::core::object::{
//!     CoreObjectCreateArgs, CoreObjectGetArgs, CoreObjectGetOptions,
//! };
//! use wwise_waapi::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = WaapiClient::connect().await?;
//!
//!     // Top-level shortcuts (`client.core()`, `client.object()`, `client.ui()`, `client.soundengine()`):
//!     let info = client.core().get_info().await?;
//!     println!("Connected to {} {}", info.display_name, info.version.display_name);
//!
//!     // Query objects with an explicit CoreObjectGetArgs (`CoreObjectGetArgs::new(ObjectQuery)` or `CoreObjectGetArgs::from_waql(...)`):
//!     let response = client
//!         .object()
//!         .get(
//!             CoreObjectGetArgs::new(ObjectQuery::path(r"\Containers\Default Work Unit")),
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
//!     // Create a Sound object with an explicit parent ObjectRef:
//!     let created = client
//!         .object()
//!         .create(
//!             CoreObjectCreateArgs::new(
//!                 ObjectRef::path(r"\Containers\Default Work Unit"),
//!                 ObjectType::Sound,
//!                 "MyNewSound",
//!             )
//!             .on_name_conflict(OnNameConflict::Replace)
//!             .property(Property::Volume, -6.0),
//!         )
//!         .await?;
//!     println!("Created sound id: {:?}", created.id());
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
//! use wwise_waapi::api::ak::wwise::core::object::CoreObjectCreatedOptions;
//! use wwise_waapi::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = WaapiClient::connect().await?;
//!
//!     let (sub_id, mut rx) = client
//!         .object()
//!         .subscribe_created(Some(
//!             CoreObjectCreatedOptions::new()
//!                 .return_fields([ObjectReturnField::Id, ObjectReturnField::Name]),
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
//! use wwise_waapi::prelude::*;
//! use wwise_waapi::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = WaapiClient::connect().await?;
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
//!
//! # WAAPI API Namespaces
//!
//! All 180 WAAPI functions and 35 subscription topics are organized by their WAAPI URI hierarchy
//! under [`api::ak`] (request/response/payload types) and [`api::clients`] (callable client methods on [`WaapiClient`]).
//! Click any namespace below to view its full URI table, client methods, and types:
//!
//! ## `ak.soundengine`
//!
//! | WAAPI Namespace | Client Handle (`WaapiClient`) | Client Struct (Methods) | Types Module |
//! |---|---|---|---|
//! | `ak.soundengine.*` | `client.soundengine()` | [`SoundengineClient`](api::clients::SoundengineClient) | [`api::ak::soundengine`] |
//!
//! ## `ak.wwise.core` (Objects, Audio & Authoring)
//!
//! | WAAPI Namespace | Client Handle (`WaapiClient`) | Client Struct (Methods) | Types Module |
//! |---|---|---|---|
//! | `ak.wwise.core.*` | `client.core()` | [`WwiseCoreClient`](api::clients::WwiseCoreClient) | [`api::ak::wwise::core`] |
//! | `ak.wwise.core.object.*` | `client.object()` | [`WwiseCoreObjectClient`](api::clients::WwiseCoreObjectClient) | [`api::ak::wwise::core::object`] |
//! | `ak.wwise.core.audio.*` | `client.core().audio()` | [`WwiseCoreAudioClient`](api::clients::WwiseCoreAudioClient) | [`api::ak::wwise::core::audio`] |
//! | `ak.wwise.core.audioSourcePeaks.*` | `client.core().audio_source_peaks()` | [`WwiseCoreAudioSourcePeaksClient`](api::clients::WwiseCoreAudioSourcePeaksClient) | [`api::ak::wwise::core::audioSourcePeaks`] |
//! | `ak.wwise.core.blendContainer.*` | `client.core().blend_container()` | [`WwiseCoreBlendContainerClient`](api::clients::WwiseCoreBlendContainerClient) | [`api::ak::wwise::core::blendContainer`] |
//! | `ak.wwise.core.gameParameter.*` | `client.core().game_parameter()` | [`WwiseCoreGameParameterClient`](api::clients::WwiseCoreGameParameterClient) | [`api::ak::wwise::core::gameParameter`] |
//! | `ak.wwise.core.mediaPool.*` | `client.core().media_pool()` | [`WwiseCoreMediaPoolClient`](api::clients::WwiseCoreMediaPoolClient) | [`api::ak::wwise::core::mediaPool`] |
//! | `ak.wwise.core.project.*` | `client.core().project()` | [`WwiseCoreProjectClient`](api::clients::WwiseCoreProjectClient) | [`api::ak::wwise::core::project`] |
//! | `ak.wwise.core.sound.*` | `client.core().sound()` | [`WwiseCoreSoundClient`](api::clients::WwiseCoreSoundClient) | [`api::ak::wwise::core::sound`] |
//! | `ak.wwise.core.soundbank.*` | `client.core().soundbank()` | [`WwiseCoreSoundbankClient`](api::clients::WwiseCoreSoundbankClient) | [`api::ak::wwise::core::soundbank`] |
//! | `ak.wwise.core.sourceControl.*` | `client.core().source_control()` | [`WwiseCoreSourceControlClient`](api::clients::WwiseCoreSourceControlClient) | [`api::ak::wwise::core::sourceControl`] |
//! | `ak.wwise.core.switchContainer.*` | `client.core().switch_container()` | [`WwiseCoreSwitchContainerClient`](api::clients::WwiseCoreSwitchContainerClient) | [`api::ak::wwise::core::switchContainer`] |
//! | `ak.wwise.core.transport.*` | `client.core().transport()` | [`WwiseCoreTransportClient`](api::clients::WwiseCoreTransportClient) | [`api::ak::wwise::core::transport`] |
//! | `ak.wwise.core.undo.*` | `client.core().undo()` | [`WwiseCoreUndoClient`](api::clients::WwiseCoreUndoClient) | [`api::ak::wwise::core::undo`] |
//! | `ak.wwise.core.workUnit.*` | `client.core().work_unit()` | [`WwiseCoreWorkUnitClient`](api::clients::WwiseCoreWorkUnitClient) | [`api::ak::wwise::core::workUnit`] |
//!
//! ## `ak.wwise.core.profiler` & Diagnostics
//!
//! | WAAPI Namespace | Client Handle (`WaapiClient`) | Client Struct (Methods) | Types Module |
//! |---|---|---|---|
//! | `ak.wwise.core.profiler.*` | `client.core().profiler()` | [`WwiseCoreProfilerClient`](api::clients::WwiseCoreProfilerClient) | [`api::ak::wwise::core::profiler`] |
//! | `ak.wwise.core.profiler.captureLog.*` | `client.core().profiler().capture_log()` | [`WwiseCoreProfilerCaptureLogClient`](api::clients::WwiseCoreProfilerCaptureLogClient) | [`api::ak::wwise::core::profiler::captureLog`] |
//! | `ak.wwise.core.log.*` | `client.core().log()` | [`WwiseCoreLogClient`](api::clients::WwiseCoreLogClient) | [`api::ak::wwise::core::log`] |
//! | `ak.wwise.core.remote.*` | `client.core().remote()` | [`WwiseCoreRemoteClient`](api::clients::WwiseCoreRemoteClient) | [`api::ak::wwise::core::remote`] |
//!
//! ## `ak.wwise.ui`, `debug` & `waapi`
//!
//! | WAAPI Namespace | Client Handle (`WaapiClient`) | Client Struct (Methods) | Types Module |
//! |---|---|---|---|
//! | `ak.wwise.ui.*` | `client.ui()` | [`WwiseUiClient`](api::clients::WwiseUiClient) | [`api::ak::wwise::ui`] |
//! | `ak.wwise.ui.cli.*` | `client.ui().cli()` | [`WwiseUiCliClient`](api::clients::WwiseUiCliClient) | [`api::ak::wwise::ui::cli`] |
//! | `ak.wwise.ui.commands.*` | `client.ui().commands()` | [`WwiseUiCommandsClient`](api::clients::WwiseUiCommandsClient) | [`api::ak::wwise::ui::commands`] |
//! | `ak.wwise.ui.layout.*` | `client.ui().layout()` | [`WwiseUiLayoutClient`](api::clients::WwiseUiLayoutClient) | [`api::ak::wwise::ui::layout`] |
//! | `ak.wwise.ui.model.*` | `client.ui().model()` | [`WwiseUiModelClient`](api::clients::WwiseUiModelClient) | [`api::ak::wwise::ui::model`] |
//! | `ak.wwise.ui.project.*` | `client.ui().project()` | [`WwiseUiProjectClient`](api::clients::WwiseUiProjectClient) | [`api::ak::wwise::ui::project`] |
//! | `ak.wwise.ui.signal.*` | `client.ui().signal()` | [`WwiseUiSignalClient`](api::clients::WwiseUiSignalClient) | [`api::ak::wwise::ui::signal`] |
//! | `ak.wwise.ui.window.*` | `client.ui().window()` | [`WwiseUiWindowClient`](api::clients::WwiseUiWindowClient) | [`api::ak::wwise::ui::window`] |
//! | `ak.wwise.debug.*` | `client.wwise().debug()` | [`WwiseDebugClient`](api::clients::WwiseDebugClient) | [`api::ak::wwise::debug`] |
//! | `ak.wwise.waapi.*` | `client.wwise().waapi()` | [`WwiseWaapiClient`](api::clients::WwiseWaapiClient) | [`api::ak::wwise::waapi`] |

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
pub use waapi_client::{
    Event, WaapiCallError, WaapiClient, WaapiError, WaapiErrorDetails, WaapiErrorKind, WaapiResult,
};
pub use wamp::WampError;

/// Convenience wrapper around `serde_json::json!`.
///
/// This macro is re-exported from `wwise_waapi` so applications can build
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

/// Convenience prelude re-exporting the most commonly used WAAPI client, query,
/// object, property, and SoundEngine types.
///
/// ```no_run
/// use wwise_waapi::prelude::*;
/// ```
pub mod prelude {
    pub use crate::api::{
        ChildObject, CursorMovePosition, CursorTime, EventRef, GameObjectRef, LanguageRef,
        ListMode, ObjectQuery, ObjectRef, ObjectReturnField, ObjectReturnOptions, ObjectType,
        OnNameConflict, PlatformRef, Position, ProfilerCursor, ProfilerDataType,
        ProfilerDataTypeEntry, Property, PropertyValue, RtpcRef, SoundEngineObjectRef, Transform,
        TransformSelect, Vector3, WwiseObject, WwiseObjectRef,
    };
    pub use crate::{
        Event, WaapiCallError, WaapiClient, WaapiError, WaapiErrorDetails, WaapiErrorKind,
        WaapiFunction, WaapiResult, WaapiTopic, WampError,
    };
}
