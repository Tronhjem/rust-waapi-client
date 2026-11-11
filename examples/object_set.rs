//! Demonstrates the type-safe `wwise().core().object().set` wrapper.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example object_set

use waapi_client::WaapiClient;
use waapi_client::api::ObjectRef;
use waapi_client::api::generated::ak::wwise::core::object::{
    WwiseCoreObjectSetArgs, WwiseCoreObjectSetItem, WwiseCoreObjectSetOptions,
};
use waapi_client::api::generated::primitives::ObjectReturnField;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    let mut client = WaapiClient::new(None)
        .await
        .expect("Failed to connect to WAAPI");

    // Example: rename an object and set a property.
    let rename_and_set = WwiseCoreObjectSetArgs::new(vec![
        WwiseCoreObjectSetItem::new(ObjectRef::path(
            "\\Actor-Mixer Hierarchy\\Default Work Unit\\MySound",
        ))
        .name("MyRenamedSound")
        .property("Volume", -6.0),
    ]);

    let options = WwiseCoreObjectSetOptions::new()
        .return_fields([ObjectReturnField::Id, ObjectReturnField::Name]);

    match client
        .wwise()
        .core()
        .object()
        .set(rename_and_set, options)
        .await
    {
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
