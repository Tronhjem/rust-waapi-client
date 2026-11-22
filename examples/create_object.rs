//! Create a Random Sequence Container named "Boom" with two Sound children.
//!
//! Mirrors the Audiokinetic WAAPI example "Creating a Random Container with two
//! Sound objects" (`ak.wwise.core.object.create`).
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080. Replace the
//! parent GUID with a GUID (or path) from your own project.
//! Run with: cargo run --example create_object

use wwise_waapi::api::generated::ak::wwise::core::object::CoreObjectCreateArgs;
use wwise_waapi::{ChildObject, ObjectRef, ObjectType, WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = WaapiClient::new(None).await?;

    let args = CoreObjectCreateArgs::new(
        ObjectRef::guid("{7A12D08F-B0D9-4403-9EFA-2E6338C197C1}"),
        ObjectType::RandomSequenceContainer,
        "Boom",
    )
    .property("RandomOrSequence", 1)
    .children(vec![
        ChildObject::new(ObjectType::Sound, "A"),
        ChildObject::new(ObjectType::Sound, "B"),
    ]);

    let response = client.wwise().core().object().create(args).await?;

    println!("Created: {:?} ({:?})", response.name, response.id);
    for child in &response.children {
        println!("  child: {:?} ({:?})", child.name, child.id);
    }

    client.shutdown().await?;
    Ok(())
}
