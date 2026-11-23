//! Registers game objects, sets the emitter position, and posts a SoundEngine event.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080 and an event
//! named `Play_TestSound` in the current project.
//! Run with: cargo run --example register_and_post_event

use wwise_waapi::api::ak::soundengine::{
    SoundenginePostEventArgs, SoundengineRegisterGameObjArgs, SoundengineSetDefaultListenersArgs,
    SoundengineSetPositionArgs, SoundengineUnregisterGameObjArgs,
};
use wwise_waapi::prelude::*;

const LISTENER_ID: u64 = 1;
const EMITTER_ID: u64 = 2;
const EVENT_NAME: &str = "Play_TestSound";

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::init();

    let client = WaapiClient::connect().await?;
    let se = client.soundengine();

    let listener = GameObjectRef::id(LISTENER_ID);
    let emitter = GameObjectRef::id(EMITTER_ID);

    // Register a listener and an emitter game object.
    se.register_game_obj(SoundengineRegisterGameObjArgs::new(
        listener,
        "WaapiExampleListener",
    ))
    .await?;
    se.register_game_obj(SoundengineRegisterGameObjArgs::new(
        emitter,
        "WaapiExampleEmitter",
    ))
    .await?;

    // Make the listener the default listener for all emitters.
    se.set_default_listeners(SoundengineSetDefaultListenersArgs::new(vec![listener]))
        .await?;

    // Place the listener at the origin with default forward/up orientation.
    se.set_position(SoundengineSetPositionArgs::new(
        listener,
        Position::origin(),
    ))
    .await?;

    // Offset the emitter using Vector3 arithmetic: 2 units behind and 2 units right of the listener.
    let emitter_position = Vector3::forward * -2.0 + Vector3::right * 2.0;
    se.set_position(SoundengineSetPositionArgs::new(
        emitter,
        Position::new(emitter_position),
    ))
    .await?;

    // Post the event on the emitter game object.
    let response = se
        .post_event(SoundenginePostEventArgs::new(EventRef::name(EVENT_NAME)).game_object(emitter))
        .await?;

    println!("Posted {EVENT_NAME} on game object {EMITTER_ID}");
    println!("Playing ID: {:?}", response.playing_id);

    // Clean up.
    se.unregister_game_obj(SoundengineUnregisterGameObjArgs::new(emitter))
        .await?;
    se.unregister_game_obj(SoundengineUnregisterGameObjArgs::new(listener))
        .await?;

    client.shutdown().await?;
    Ok(())
}
