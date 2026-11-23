//! Create a Random Sequence Container named "Boom" with two Sound children
//! at the top level of `\Containers\Default Work Unit`.
//!
//! Mirrors the Audiokinetic WAAPI example "Creating a Random Container with two
//! Sound objects" (`ak.wwise.core.object.create`).
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example create_object

use wwise_waapi::api::ak::wwise::core::object::CoreObjectCreateArgs;
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = WaapiClient::connect().await?;

    let args = CoreObjectCreateArgs::new(
        ObjectRef::path(r"\Containers\Default Work Unit"),
        ObjectType::RandomSequenceContainer,
        "Boom",
    )
    .on_name_conflict(OnNameConflict::Replace)
    .property(Property::RandomOrSequence, 1)
    .children(vec![
        ChildObject::new(ObjectType::Sound, "A"),
        ChildObject::new(ObjectType::Sound, "B"),
    ]);

    let response = client.object().create(args).await?;

    println!("Created: {:?} ({:?})", response.name(), response.id());
    for child in &response.children {
        println!("  child: {:?} ({:?})", child.name(), child.id());
    }

    client.shutdown().await?;
    Ok(())
}
