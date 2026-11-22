//! Post an Event to the sound engine by name, Short ID, or GUID.
//!
//! Mirrors the Audiokinetic WAAPI examples "Posting an Event by name",
//! "Posting an Event by Short ID", and "Posting an Event by object GUID"
//! (`ak.soundengine.postEvent`).
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example post_event

use wwise_waapi::api::ak::soundengine::PostEventArgs;
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = WaapiClient::connect().await?;

    let game_object = 1122334u64;

    // The same Event can be addressed three ways. Post by name here; uncomment
    // one of the other two to post by Short ID or GUID instead.
    let args =
        PostEventArgs::new(SoundEngineObjectRef::name("Play_TestSound")).game_object(game_object);
    // let args = PostEventArgs::new(SoundEngineObjectRef::short(2932040671))
    //     .game_object(game_object);
    // let args = PostEventArgs::new(SoundEngineObjectRef::guid(
    //     "{F546017D-201A-49BD-8D4E-0A28F5DBB28D}",
    // ))
    // .game_object(game_object);

    let response = client.soundengine().post_event(args).await?;

    println!(
        "Posted the Event on game object {game_object}; playing ID = {}",
        response.playing_id
    );

    client.shutdown().await?;
    Ok(())
}
