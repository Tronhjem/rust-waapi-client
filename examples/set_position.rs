//! Set the position and orientation of a game object.
//!
//! Mirrors the Audiokinetic WAAPI example "Setting the position of a game
//! object" (`ak.soundengine.setPosition`).
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example set_position

use wwise_waapi::api::ak::soundengine::SetPositionArgs;
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = WaapiClient::connect().await?;

    let args = SetPositionArgs::new(
        GameObjectRef::id(1122334),
        Position::with_orientations(
            Vector3::new(5.0, 10.0, 100.0),
            Vector3::new(100.0, 200.0, 0.0),
            Vector3::new(0.0, 0.0, 200.0),
        ),
    );

    client.soundengine().set_position(args).await?;

    println!("Set the position of game object 1122334 to (5, 10, 100).");

    client.shutdown().await?;
    Ok(())
}
