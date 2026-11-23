//! Demonstrates the type-safe `client.object().set(...)` wrapper (`ak.wwise.core.object.set`).
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example object_set
//!
//! Targets the Wwise 2025 hierarchy (`\Containers`). See `object_set_2024.rs`
//! for the Wwise 2024 `\Actor-Mixer Hierarchy` variant.

use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectSetArgs, CoreObjectSetItem, CoreObjectSetOptions,
};
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    let client = WaapiClient::connect().await?;

    // Rename an object and set its Volume property.
    let item = CoreObjectSetItem::new(ObjectRef::path(r"\Containers\Default Work Unit\MySound"))
        .name("MyRenamedSound")
        .property(Property::Volume, -6.0);

    let options =
        CoreObjectSetOptions::new().return_fields([ObjectReturnField::Id, ObjectReturnField::Name]);

    match client
        .object()
        .set(CoreObjectSetArgs::from_item(item), options)
        .await
    {
        Ok(response) => {
            for obj in response.objects {
                println!(
                    "Modified: id={:?}  name={:?}  children={}",
                    obj.id(),
                    obj.name(),
                    obj.children.len()
                );
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    client.shutdown().await?;
    Ok(())
}
