//! Playground for the chained typed API: get an object, then modify it with
//! `object.set`.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example object_get_set
//!
//! The example modifies whatever object is currently selected in the Wwise
//! authoring UI (sets its notes and Volume), so select something first.

use waapi_client::api::ObjectRef;
use waapi_client::api::generated::ak::wwise::core::object::{
    WwiseCoreObjectGetArgs, WwiseCoreObjectGetOptions, WwiseCoreObjectSetArgs,
    WwiseCoreObjectSetItem, WwiseCoreObjectSetOptions,
};
use waapi_client::api::generated::ak::wwise::ui::WwiseUiGetSelectedObjectsOptions;
use waapi_client::api::generated::primitives::{ObjectQuery, ObjectReturnField};
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    let mut client = WaapiClient::new(None).await?;

    // 1. Get an object by path. The Default Work Unit exists in every project.
    let get = client
        .wwise()
        .core()
        .object()
        .get(
            WwiseCoreObjectGetArgs::new(ObjectQuery::from(
                r"\Actor-Mixer Hierarchy\Default Work Unit",
            )),
            WwiseCoreObjectGetOptions::new().return_fields([
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::ChildrenCount,
            ]),
        )
        .await?;
    for obj in &get.objects {
        println!(
            "get: {:?} ({:?}) has {:?} children",
            obj.name, obj.id, obj.children_count
        );
    }

    // 2. See what is selected in the Wwise UI.
    let selected = client
        .wwise()
        .ui()
        .get_selected_objects(WwiseUiGetSelectedObjectsOptions::new().return_fields([
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
    let target_id = target.id.clone().expect("requested id return field");
    println!(
        "Selected: {:?} (type {:?})",
        target.name, target.object_type
    );

    // 3. Modify it: set notes and the Volume property.
    let set_args = WwiseCoreObjectSetArgs::new(vec![
        WwiseCoreObjectSetItem::new(ObjectRef::guid(target_id))
            .notes("Modified by the waapi-client object_get_set example.")
            .property("Volume", -3.0),
    ]);
    let set_options = WwiseCoreObjectSetOptions::new().return_fields([
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Custom("@Volume".to_string()),
    ]);

    match client
        .wwise()
        .core()
        .object()
        .set(set_args, set_options)
        .await
    {
        Ok(response) => {
            for obj in &response.objects {
                println!(
                    "set: {:?} ({:?}) @Volume is now {:?}",
                    obj.name,
                    obj.id,
                    obj.extra.get("@Volume")
                );
            }
        }
        Err(e) => eprintln!("object.set failed (does this object have a Volume?): {e}"),
    }

    client.shutdown().await?;
    Ok(())
}
