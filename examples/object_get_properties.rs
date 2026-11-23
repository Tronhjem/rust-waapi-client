//! Read the `Volume` property and `OutputBus` reference of every Sound object.
//!
//! Mirrors the Audiokinetic WAAPI example "Getting the properties and
//! references of an object" (`ak.wwise.core.object.get`), generalised to query
//! all Sound objects with a WAQL query instead of a single hard-coded GUID.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080 and at least one
//! Sound object in the current project.
//! Run with: cargo run --example object_get_properties

use wwise_waapi::api::ak::wwise::core::object::{CoreObjectGetArgs, CoreObjectGetOptions};
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = WaapiClient::connect().await?;

    // `$ from type Sound` returns every Sound object in the project.
    let args = CoreObjectGetArgs::from_waql("$ from type Sound");

    // Request the name, @Volume property, and @OutputBus reference for each result.
    let options = CoreObjectGetOptions::new().return_fields([
        ObjectReturnField::Name,
        ObjectReturnField::property(Property::Volume),
        ObjectReturnField::property(Property::OutputBus),
    ]);

    let response = client.object().get(args, options).await?;

    for obj in &response.objects {
        println!(
            "{:?}: Volume = {:?}, OutputBus = {:?}",
            obj.name(),
            obj.property_f64(Property::Volume),
            obj.property(Property::OutputBus),
        );
    }

    client.shutdown().await?;
    Ok(())
}
