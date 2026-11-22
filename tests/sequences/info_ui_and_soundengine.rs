use super::{connect_client, sequence_lock};
use wwise_waapi::api::ak::soundengine::{
    SoundenginePostEventArgs, SoundengineRegisterGameObjArgs, SoundengineSetDefaultListenersArgs,
    SoundengineSetPositionArgs, SoundengineSetRtpcValueArgs, SoundengineUnregisterGameObjArgs,
};
use wwise_waapi::api::ak::wwise::core::object::{CoreObjectCreateArgs, CoreObjectDeleteArgs};
use wwise_waapi::api::ak::wwise::ui::UiGetSelectedObjectsOptions;
use wwise_waapi::api::{EventRef, ObjectReturnField, OnNameConflict, RtpcRef};
use wwise_waapi::{
    GameObjectRef, ObjectRef, ObjectType, Position, Property, Vector3, WaapiFunction, json,
};

const GAME_PARAMS_WU_PATH: &str = r"\Game Parameters\Default Work Unit";
const EVENTS_WU_PATH: &str = r"\Events\Default Work Unit";
const TEST_RTPC_NAME: &str = "WaapiSeqTestRtpc";
const TEST_EVENT_NAME: &str = "WaapiSeqTestEvent";

#[tokio::test]
async fn info_ui_soundengine_and_raw_call_sequence() {
    let _guard = sequence_lock().await;
    let client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    // 1. Call ak.wwise.core.getInfo via both client.wwise() and client.ak().wwise()
    let info = client
        .wwise()
        .core()
        .get_info()
        .await
        .expect("Failed to call ak.wwise.core.getInfo");
    assert!(!info.display_name.is_empty());
    assert!(!info.platform.is_empty());
    assert!(info.version.year >= 2020);
    assert!(info.version.schema > 0);
    assert!(!info.directories.install.is_empty());
    assert!(!info.directories.authoring.is_empty());

    let info_via_ak = client
        .ak()
        .wwise()
        .core()
        .get_info()
        .await
        .expect("Failed to call ak.wwise.core.getInfo via client.ak().wwise()");
    assert_eq!(info.session_id, info_via_ak.session_id);

    // 2. Call ak.wwise.ui.getSelectedObjects
    let selected = client
        .wwise()
        .ui()
        .get_selected_objects(UiGetSelectedObjectsOptions::new().return_fields(vec![
            ObjectReturnField::Id,
            ObjectReturnField::Name,
            ObjectReturnField::Type,
            ObjectReturnField::Path,
        ]))
        .await
        .expect("Failed to call ak.wwise.ui.getSelectedObjects");
    for obj in &selected.objects {
        assert!(obj.id.is_some());
        assert!(obj.name.is_some());
    }

    // 3. Create a temporary GameParameter and Event, test soundengine endpoints and raw call, then delete
    let created_rtpc = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(GAME_PARAMS_WU_PATH),
                ObjectType::GameParameter,
                TEST_RTPC_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace)
            .property(Property::InitialValue, 25.0),
        )
        .await
        .expect("Failed to create temporary GameParameter");
    let rtpc_id = created_rtpc.id.expect("Created GameParameter missing id");

    let created_event = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(EVENTS_WU_PATH),
                ObjectType::Event,
                TEST_EVENT_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("Failed to create temporary Event");
    let event_id = created_event.id.expect("Created Event missing id");

    // Register listener and emitter game objects, set default listeners and 3D positions
    let listener = GameObjectRef::game_object(1001);
    let emitter = GameObjectRef::game_object(1002);

    client
        .soundengine()
        .register_game_obj(SoundengineRegisterGameObjArgs::new(
            listener,
            "WaapiSeqListener",
        ))
        .await
        .expect("Failed to call ak.soundengine.registerGameObj for listener");
    client
        .soundengine()
        .register_game_obj(SoundengineRegisterGameObjArgs::new(
            emitter,
            "WaapiSeqEmitter",
        ))
        .await
        .expect("Failed to call ak.soundengine.registerGameObj for emitter");

    client
        .soundengine()
        .set_default_listeners(SoundengineSetDefaultListenersArgs::new(vec![listener]))
        .await
        .expect("Failed to call ak.soundengine.setDefaultListeners");

    client
        .soundengine()
        .set_position(SoundengineSetPositionArgs::new(
            listener,
            Position::origin(),
        ))
        .await
        .expect("Failed to call ak.soundengine.setPosition for listener");

    let emitter_pos = Vector3::forward * -2.0 + Vector3::right * 2.0;
    client
        .soundengine()
        .set_position(SoundengineSetPositionArgs::new(
            emitter,
            Position::new(emitter_pos),
        ))
        .await
        .expect("Failed to call ak.soundengine.setPosition for emitter");

    // Set RTPC value via client.soundengine().set_rtpc_value
    client
        .soundengine()
        .set_rtpc_value(
            SoundengineSetRtpcValueArgs::new(RtpcRef::from(TEST_RTPC_NAME), 75.0)
                .game_object(emitter),
        )
        .await
        .expect("Failed to call ak.soundengine.setRTPCValue");

    // Post Event via client.soundengine().post_event
    let post_resp = client
        .soundengine()
        .post_event(
            SoundenginePostEventArgs::new(EventRef::from(TEST_EVENT_NAME)).game_object(emitter),
        )
        .await
        .expect("Failed to call ak.soundengine.postEvent");
    let _ = post_resp.playing_id;

    // Unregister emitter and listener game objects
    client
        .soundengine()
        .unregister_game_obj(SoundengineUnregisterGameObjArgs::new(emitter))
        .await
        .expect("Failed to call ak.soundengine.unregisterGameObj for emitter");
    client
        .soundengine()
        .unregister_game_obj(SoundengineUnregisterGameObjArgs::new(listener))
        .await
        .expect("Failed to call ak.soundengine.unregisterGameObj for listener");

    // 4. Test low-level escape hatch `client.call` + `wwise_waapi::json!` against the temporary GameParameter
    let raw_resp = client
        .call(
            WaapiFunction::ak_wwise_core_object_get,
            Some(json!({
                "from": { "id": [&rtpc_id] }
            })),
            Some(json!({
                "return": ["id", "name", "type", Property::InitialValue.as_at_str()]
            })),
        )
        .await
        .expect("Failed to execute raw client.call with WaapiFunction");

    let returned = raw_resp["return"]
        .as_array()
        .expect("Expected 'return' array in raw response");
    assert_eq!(returned.len(), 1);
    assert_eq!(returned[0]["id"].as_str(), Some(rtpc_id.as_str()));
    assert_eq!(returned[0]["name"].as_str(), Some(TEST_RTPC_NAME));
    assert_eq!(
        returned[0]["type"].as_str(),
        Some(ObjectType::GameParameter.as_str())
    );
    assert_eq!(
        returned[0][Property::InitialValue.as_at_str()].as_f64(),
        Some(25.0)
    );

    // 5. Clean up temporary GameParameter and Event
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(rtpc_id.as_str())))
        .await
        .expect("Failed to delete temporary GameParameter");

    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            event_id.as_str(),
        )))
        .await
        .expect("Failed to delete temporary Event");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
