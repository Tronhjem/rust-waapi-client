//! Registers game objects, sets the emitter position to the origin, and posts a SoundEngine event.
//!
//! Requires Wwise to be running with WAAPI enabled on `ws://127.0.0.1:8080/waapi` and an event
//! named `Play_EventTestSound` in the current project.

use waapi_client::{
    GameObjectRef, Position, Vector3, WaapiClient,
    api::ak::soundengine::{
        SoundenginePostEventArgs, SoundengineRegisterGameObjArgs,
        SoundengineSetDefaultListenersArgs, SoundengineSetPositionArgs,
        SoundengineUnregisterGameObjArgs,
    },
};

const LISTENER_ID: u64 = 1;
const EMITTER_ID: u64 = 2;
const EVENT_NAME: &str = "Play_EventTestSound";

#[tokio::main]
async fn main() -> waapi_client::WaapiResult<()> {
    env_logger::init();

    let mut client = WaapiClient::new(Some("ws://127.0.0.1:8080/waapi")).await?;

    // Register a listener and an emitter game object.
    client
        .soundengine()
        .register_game_obj(SoundengineRegisterGameObjArgs::new(
            GameObjectRef::game_object(LISTENER_ID),
            "WaapiExampleListener".into(),
        ))
        .await?;
    client
        .soundengine()
        .register_game_obj(SoundengineRegisterGameObjArgs::new(
            GameObjectRef::game_object(EMITTER_ID),
            "WaapiExampleEmitter".into(),
        ))
        .await?;

    // Make the listener the default listener for all emitters.
    client
        .soundengine()
        .set_default_listeners(SoundengineSetDefaultListenersArgs::new(vec![
            GameObjectRef::game_object(LISTENER_ID),
        ]))
        .await?;

    // Place the listener at the origin with default forward/up orientation.
    client
        .soundengine()
        .set_position(SoundengineSetPositionArgs::new(
            GameObjectRef::game_object(LISTENER_ID),
            Position::origin(),
        ))
        .await?;

    // Offset the emitter using Vector3 arithmetic: 2 units behind and 2 units right of the listener.
    let emitter_position = Vector3::forward * -2.0 + Vector3::right * 2.0;
    client
        .soundengine()
        .set_position(SoundengineSetPositionArgs::new(
            GameObjectRef::game_object(EMITTER_ID),
            Position::new(emitter_position),
        ))
        .await?;

    // Post the event on the emitter game object.
    let response = client
        .soundengine()
        .post_event(SoundenginePostEventArgs::new(EVENT_NAME.into()).game_object(EMITTER_ID))
        .await?;

    println!("Posted {EVENT_NAME} on game object {EMITTER_ID}");
    println!("Playing ID: {:?}", response.playing_id);

    // Clean up.
    client
        .soundengine()
        .unregister_game_obj(SoundengineUnregisterGameObjArgs::new(
            GameObjectRef::game_object(EMITTER_ID),
        ))
        .await?;
    client
        .soundengine()
        .unregister_game_obj(SoundengineUnregisterGameObjArgs::new(
            GameObjectRef::game_object(LISTENER_ID),
        ))
        .await?;

    client.shutdown().await?;
    Ok(())
}
