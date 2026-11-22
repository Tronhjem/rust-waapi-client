//! Demonstrates the type-safe `wwise().core().object().set` wrapper.
//!
//! **Wwise 2024 variant** — uses the pre-2025 `Actor-Mixer Hierarchy` path
//! (renamed to `Containers` in Wwise 2025). See `object_set.rs` for the 2025
//! variant.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example object_set_2024

use wwise_waapi::api::ak::wwise::core::object::{SetItem, SetOptions};
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    let client = WaapiClient::connect()
        .await
        .expect("Failed to connect to WAAPI");

    // Example: rename an object and set a property.
    let item = SetItem::new(r"\Actor-Mixer Hierarchy\Default Work Unit\MySound")
        .name("MyRenamedSound")
        .property(Property::Volume, -6.0);

    let options = SetOptions::new().return_fields([ObjectReturnField::Id, ObjectReturnField::Name]);

    match client.object().set(item, options).await {
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

    client.shutdown().await.unwrap();
}
