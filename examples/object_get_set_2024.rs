//! Playground for the chained typed API: get an object, then modify it with
//! `object.set`.
//!
//! **Wwise 2024 variant** — uses the pre-2025 `Actor-Mixer Hierarchy` path
//! (renamed to `Containers` in Wwise 2025). See `object_get_set.rs` for the
//! 2025 variant.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example object_get_set_2024
//!
//! The example modifies whatever object is currently selected in the Wwise
//! authoring UI (sets its notes and Volume), so select something first.

use wwise_waapi::api::ak::wwise::core::object::{GetOptions, SetItem, SetOptions};
use wwise_waapi::api::ak::wwise::ui::GetSelectedObjectsOptions;
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    let mut client = WaapiClient::connect().await?;

    // 1. Get an object by path. The Default Work Unit exists in every project.
    let get = client
        .object()
        .get(
            r"\Actor-Mixer Hierarchy\Default Work Unit",
            GetOptions::new().return_fields([
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::ChildrenCount,
            ]),
        )
        .await?;

    for obj in &get.objects {
        println!(
            "get: {:?} ({:?}) has {:?} children",
            obj.name(),
            obj.id(),
            obj.children_count
        );
    }

    // 2. See what is selected in the Wwise UI.
    let selected = client
        .ui()
        .get_selected_objects(GetSelectedObjectsOptions::new().return_fields([
            ObjectReturnField::Id,
            ObjectReturnField::Name,
            ObjectReturnField::Type,
        ]))
        .await?;

    if selected.objects.is_empty() {
        println!("Nothing selected in Wwise - select an object and run again.");
        client.shutdown().await?;
        return Ok(());
    }

    let target = &selected.objects[0];
    let target_id = target.id().expect("requested id return field");

    println!(
        "Selected: {:?} (type {:?})",
        target.name(),
        target.object_type()
    );

    // 3. Modify it: set notes and the Volume property.
    let item = SetItem::new(target_id)
        .notes("Modified by the wwise-waapi object_get_set_2024 example.")
        .property(Property::Volume, -3.0);

    let set_options = SetOptions::new().return_fields([
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Property(Property::Volume),
    ]);

    match client.object().set(item, set_options).await {
        Ok(response) => {
            for obj in &response.objects {
                println!(
                    "set: {:?} ({:?}) @Volume is now {:?}",
                    obj.name,
                    obj.id,
                    obj.property(Property::Volume)
                );
            }
        }
        Err(e) => eprintln!("object.set failed (does this object have a Volume?): {e}"),
    }

    client.shutdown().await?;
    Ok(())
}
