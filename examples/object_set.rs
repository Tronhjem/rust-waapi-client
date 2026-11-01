//! Demonstrates the type-safe `object_set` wrapper.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example object_set

use waapi_client::WaapiClient;
use waapi_client::api::object_set::{ObjectReturnOptions, ObjectSetArgs, ObjectSetItem};
use waapi_client::api::{ObjectRef, PropertyValue};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    let mut client = WaapiClient::new(None)
        .await
        .expect("Failed to connect to WAAPI");

    // Example: rename an object and set a property.
    let rename_and_set = ObjectSetArgs::new(vec![
        ObjectSetItem::new(ObjectRef::path(
            "\\Actor-Mixer Hierarchy\\Default Work Unit\\MySound",
        ))
        .rename("MyRenamedSound")
        .property("Volume", PropertyValue::Number(-6.0)),
    ]);

    let options = ObjectReturnOptions::new(vec!["id".to_string(), "name".to_string()]);

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

    client.shutdown().await.unwrap();
}
