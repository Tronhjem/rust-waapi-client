//! Registers game objects, sets the emitter position to the origin, and posts a SoundEngine event.
//!
//! Requires Wwise to be running with WAAPI enabled on `ws://127.0.0.1:8080/waapi` and an event
//! named `Play_TestSound` in the current project.

use wwise_waapi::api::ak::soundengine::PostEventArgs;
use wwise_waapi::prelude::*;

const LISTENER_ID: u64 = 1;
const EMITTER_ID: u64 = 2;
const EVENT_NAME: &str = "Play_TestSound";

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::init();

    let mut client = WaapiClient::connect().await?;
    let mut se = client.soundengine();

    // Register a listener and an emitter game object.
    se.register_game_obj((LISTENER_ID, "WaapiExampleListener"))
        .await?;
    se.register_game_obj((EMITTER_ID, "WaapiExampleEmitter"))
        .await?;

    // Make the listener the default listener for all emitters.
    se.set_default_listeners(vec![LISTENER_ID.into()]).await?;

    // Place the listener at the origin with default forward/up orientation.
    se.set_position((LISTENER_ID, Position::origin())).await?;

    // Offset the emitter using Vector3 arithmetic: 2 units behind and 2 units right of the listener.
    let emitter_position = Vector3::forward * -2.0 + Vector3::right * 2.0;
    se.set_position((EMITTER_ID, Position::new(emitter_position)))
        .await?;

    // Post the event on the emitter game object.
    let response = se
        .post_event(PostEventArgs::new(EVENT_NAME).game_object(EMITTER_ID))
        .await?;

    println!("Posted {EVENT_NAME} on game object {EMITTER_ID}");
    println!("Playing ID: {:?}", response.playing_id);

    // Clean up.
    se.unregister_game_obj(EMITTER_ID).await?;
    se.unregister_game_obj(LISTENER_ID).await?;

    client.shutdown().await?;
    Ok(())
}
