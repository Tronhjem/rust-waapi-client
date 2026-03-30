//! Demonstrates the type-safe `object_set` wrapper.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example object_set

use waapi_client::WaapiClient;
use waapi_client::api::object_set::{ObjectReturnOptions, ObjectSetArgs, ObjectSetItem};
use waapi_client::api::{ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, PropertyValue};
use waapi_client::types::ReturnType;

#[tokio::main]
async fn main() {
    WaapiClient::initialize_logger(Some("error"));

    let mut client = WaapiClient::new(None).expect("Failed to connect to WAAPI");

    // ── Example 1: rename an object and set a property ────────────────────────
    //
    // JSON equivalent:
    // {
    //   "objects": [{
    //     "object": "\\Actor-Mixer Hierarchy\\Default Work Unit\\MySound",
    //     "name": "MyRenamedSound",
    //     "@Volume": -6.0
    //   }]
    // }
    let rename_and_set = ObjectSetArgs::new(vec![
        ObjectSetItem::new(ObjectRef::path(
            "\\Actor-Mixer Hierarchy\\Default Work Unit\\MySound",
        ))
        .rename("MyRenamedSound")
        .property("Volume", PropertyValue::Number(-6.0)),
    ]);

    // ── Example 2: set a platform-specific property ───────────────────────────
    //
    // Passing the wrong type here (e.g. PropertyValue::Boolean instead of
    // PropertyValue::Number) is a compile error, not a runtime surprise.
    let platform_specific = ObjectSetArgs::new(vec![
        ObjectSetItem::new(ObjectRef::path(
            "\\Master-Mixer Hierarchy\\Default Work Unit\\Master Audio Bus",
        ))
        .platform(PlatformRef::name("Windows"))
        .property("Volume", PropertyValue::Boolean(false)),
    ]);

    // ── Example 3: create children under an object ────────────────────────────
    //
    // OnNameConflict::Replace is an enum — passing the string "replacee" or
    // any other typo is a compile error.
    let create_children = ObjectSetArgs::new(vec![
        ObjectSetItem::new(ObjectRef::path(
            "\\Actor-Mixer Hierarchy\\Default Work Unit",
        ))
        .on_name_conflict(OnNameConflict::Replace)
        .child(
            ChildObject::new("Sound", "Explosion_Large")
                .notes("Created via WAAPI object.set")
                .property("Volume", PropertyValue::Number(-12.0)),
        )
        .child(
            ChildObject::new("Sound", "Explosion_Small")
                .child(
                    // Nested child — type-checked recursively
                    ChildObject::new("AudioFileSource", "explosion_small_01"),
                ),
        ),
    ]);

    // ── Example 4: batch — multiple objects in one call, with a shared default ─
    let batch = ObjectSetArgs::new(vec![
        ObjectSetItem::new(ObjectRef::guid("{AA79CD6E-0D44-46FC-B065-15F2D578473B}"))
            .property("Volume", PropertyValue::Number(0.0)),
        ObjectSetItem::new(ObjectRef::name("Play_Footsteps"))
            .property("Volume", PropertyValue::Number(-3.0))
            // This item overrides the top-level OnNameConflict default:
            .on_name_conflict(OnNameConflict::Merge),
    ])
    // Top-level defaults apply to all items that don't override them:
    .platform(PlatformRef::name("Windows"))
    .list_mode(ListMode::ReplaceAll)
    .auto_add_to_source_control(false);

    // ── Return options ─────────────────────────────────────────────────────────
    //
    // ReturnType is already a typed enum — no string typos possible.
    let options = ObjectReturnOptions::new(vec![ReturnType::Id, ReturnType::Name]);

    // Run one of the examples (pick whichever matches your project):
    match client.object_set(rename_and_set, options).await {
        Ok(response) => {
            for obj in response.objects {
                println!(
                    "Modified: id={:?}  name={:?}  children={}",
                    obj.id,
                    obj.name,
                    obj.children.len()
                );
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    // Suppress unused-variable warnings for the other examples in this file:
    let _ = (platform_specific, create_children, batch);

    client.shutdown().await.unwrap();
}
