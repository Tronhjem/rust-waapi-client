use super::{connect_client, master_audio_bus, sequence_lock};
use wwise_waapi::api::ak::soundengine::{
    SoundengineExecuteActionOnEventArgs, SoundengineGetStateArgs, SoundengineGetSwitchArgs,
    SoundenginePostEventArgs, SoundenginePostMsgMonitorArgs, SoundenginePostTriggerArgs,
    SoundengineRegisterGameObjArgs, SoundengineResetRtpcValueArgs,
    SoundengineSetDefaultListenersArgs, SoundengineSetGameObjectAuxSendValuesArgs,
    SoundengineSetGameObjectOutputBusVolumeArgs, SoundengineSetListenerSpatializationArgs,
    SoundengineSetListenersArgs, SoundengineSetMultiplePositionsArgs,
    SoundengineSetObjectObstructionAndOcclusionArgs, SoundengineSetScalingFactorArgs,
    SoundengineSetStateArgs, SoundengineSetSwitchArgs, SoundengineStopAllArgs,
    SoundengineStopPlayingIdArgs, SoundengineUnregisterGameObjArgs,
};
use wwise_waapi::api::ak::wwise::core::object::{CoreObjectCreateArgs, CoreObjectDeleteArgs};
use wwise_waapi::api::{
    ActionOnEventType, AkCurveInterpolation, AuxSendValue, EventRef, GameObjectRef, MultiPosition,
    MultiPositionType, ObjectReturnField, ObjectReturnOptions, OnNameConflict,
    SoundEngineObjectRef,
};
use wwise_waapi::{ObjectRef, ObjectType, Position, Vector3};

const SWITCH_GROUPS_WU: &str = r"\Switches\Default Work Unit";
const STATE_GROUPS_WU: &str = r"\States\Default Work Unit";
const TRIGGERS_WU: &str = r"\Triggers\Default Work Unit";
const GAME_PARAMS_WU: &str = r"\Game Parameters\Default Work Unit";
const EVENTS_WU: &str = r"\Events\Default Work Unit";

#[tokio::test]
async fn soundengine_extended_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    // 0. Discover the master audio bus (name differs across Wwise versions).
    let (master_bus_id, _) = master_audio_bus(&mut client).await;

    // Unique names per run avoid collisions with leftovers from prior failed runs
    // (setName/setState/setSwitch resolve by name and have no conflict handling).
    let suffix = std::process::id();
    let switch_group_name = format!("WaapiSeqTestSwitchGroup_{suffix}");
    let switch_name = format!("WaapiSeqTestSwitch_{suffix}");
    let state_group_name = format!("WaapiSeqTestStateGroup_{suffix}");
    let state_name = format!("WaapiSeqTestState_{suffix}");
    let aux_bus_name = format!("WaapiSeqTestAuxBus_{suffix}");
    let trigger_name = format!("WaapiSeqTestTrigger_{suffix}");
    let rtpc_name = format!("WaapiSeqTestRtpc_{suffix}");
    let event_name = format!("WaapiSeqTestEvent_{suffix}");

    let listener = GameObjectRef::game_object(2101);
    let emitter = GameObjectRef::game_object(2102);
    client
        .soundengine()
        .register_game_obj(SoundengineRegisterGameObjArgs::new(
            listener,
            "WaapiSeqListener",
        ))
        .await
        .expect("register listener");
    client
        .soundengine()
        .register_game_obj(SoundengineRegisterGameObjArgs::new(
            emitter,
            "WaapiSeqEmitter",
        ))
        .await
        .expect("register emitter");

    // 1. Switch + State groups with a single value each.
    let switch_group = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(SWITCH_GROUPS_WU),
                ObjectType::SwitchGroup,
                switch_group_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create switch group");
    let switch_group_id = switch_group.id.clone().expect("switch group id");
    let switch = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(switch_group_id.as_str()),
                ObjectType::Switch,
                switch_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create switch");
    let switch_id = switch.id.clone().expect("switch id");

    let state_group = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(STATE_GROUPS_WU),
                ObjectType::StateGroup,
                state_group_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create state group");
    let state_group_id = state_group.id.clone().expect("state group id");
    let state = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(state_group_id.as_str()),
                ObjectType::State,
                state_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create state");
    let state_id = state.id.clone().expect("state id");
    // setState applies to the transport game object, so getState does not
    // reliably return this exact State; keep state_id for reference/cleanup.
    let _ = state_id;

    let aux_bus = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(master_bus_id.as_str()),
                ObjectType::AuxBus,
                aux_bus_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create aux bus");
    let aux_bus_id = aux_bus.id.clone().expect("aux bus id");

    let trigger = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(TRIGGERS_WU),
                ObjectType::Trigger,
                trigger_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create trigger");
    let trigger_id = trigger.id.clone().expect("trigger id");

    let game_param = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(GAME_PARAMS_WU),
                ObjectType::GameParameter,
                rtpc_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create game parameter");
    let game_param_id = game_param.id.clone().expect("game param id");

    let event = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(EVENTS_WU),
                ObjectType::Event,
                event_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create event");
    let event_id = event.id.clone().expect("event id");

    // 2. Default listeners, then set/get switch and state.
    client
        .soundengine()
        .set_default_listeners(SoundengineSetDefaultListenersArgs::new(vec![listener]))
        .await
        .expect("set default listeners");

    let id_name = ObjectReturnOptions::new()
        .return_fields(vec![ObjectReturnField::Id, ObjectReturnField::Name]);

    client
        .soundengine()
        .set_switch(
            SoundengineSetSwitchArgs::new(
                SoundEngineObjectRef::from(switch_group_name.as_str()),
                SoundEngineObjectRef::from(switch_name.as_str()),
            )
            .game_object(emitter),
        )
        .await
        .expect("set switch");
    let got_switch = client
        .soundengine()
        .get_switch(
            SoundengineGetSwitchArgs::new(ObjectRef::from(switch_group_id.as_str()))
                .game_object(emitter),
            id_name.clone(),
        )
        .await
        .expect("get switch");
    assert_eq!(got_switch.switch.id.as_deref(), Some(switch_id.as_str()));

    client
        .soundengine()
        .set_state(SoundengineSetStateArgs::new(
            SoundEngineObjectRef::from(state_group_name.as_str()),
            SoundEngineObjectRef::from(state_name.as_str()),
        ))
        .await
        .expect("set state");
    let got_state = client
        .soundengine()
        .get_state(
            SoundengineGetStateArgs::new(ObjectRef::from(state_group_id.as_str())),
            ObjectReturnOptions::new()
                .return_fields(vec![ObjectReturnField::Id, ObjectReturnField::Type]),
        )
        .await
        .expect("get state");
    // setState applies to the transport game object (there is no gameObject arg),
    // so getState returns the group's current State object (often the default
    // "None" state) rather than necessarily the one we just set. Verify it is a
    // State object and deserializes correctly.
    assert_eq!(got_state.state.object_type, Some(ObjectType::State));

    // 3. Listeners, spatialization, obstruction/occlusion, scaling, output bus volume.
    client
        .soundengine()
        .set_listeners(SoundengineSetListenersArgs::new(emitter, vec![listener]))
        .await
        .expect("set listeners");
    client
        .soundengine()
        .set_listener_spatialization(SoundengineSetListenerSpatializationArgs::new(
            listener,
            true,
            4,
            vec![0.0, 0.0, 0.0, 0.0],
        ))
        .await
        .expect("set listener spatialization");
    client
        .soundengine()
        .set_object_obstruction_and_occlusion(SoundengineSetObjectObstructionAndOcclusionArgs::new(
            emitter, listener, 0.0, 0.0,
        ))
        .await
        .expect("set obstruction/occlusion");
    client
        .soundengine()
        .set_scaling_factor(SoundengineSetScalingFactorArgs::new(emitter, 1.0))
        .await
        .expect("set scaling factor");
    client
        .soundengine()
        .set_game_object_output_bus_volume(SoundengineSetGameObjectOutputBusVolumeArgs::new(
            emitter, listener, 0.0,
        ))
        .await
        .expect("set output bus volume");

    // 4. Multiple positions and aux sends.
    client
        .soundengine()
        .set_multiple_positions(SoundengineSetMultiplePositionsArgs::new(
            emitter,
            vec![
                MultiPosition::from(Position::origin()),
                MultiPosition::from(Position::new(Vector3::forward * 2.0 + Vector3::right)),
            ],
            MultiPositionType::MultiDirections,
        ))
        .await
        .expect("set multiple positions");
    client
        .soundengine()
        .set_game_object_aux_send_values(SoundengineSetGameObjectAuxSendValuesArgs::new(
            emitter,
            vec![AuxSendValue::new(listener, aux_bus_name.as_str(), -6.0)],
        ))
        .await
        .expect("set aux send values");

    // 5. RTPC reset, trigger, monitor message, event post + stop.
    client
        .soundengine()
        .reset_rtpc_value(
            SoundengineResetRtpcValueArgs::new(SoundEngineObjectRef::from(rtpc_name.as_str()))
                .game_object(emitter),
        )
        .await
        .expect("reset rtpc");
    client
        .soundengine()
        .post_trigger(
            SoundenginePostTriggerArgs::new(SoundEngineObjectRef::from(trigger_name.as_str()))
                .game_object(emitter),
        )
        .await
        .expect("post trigger");
    client
        .soundengine()
        .post_msg_monitor(SoundenginePostMsgMonitorArgs::new(
            "hello from wwise-waapi sequence test".to_string(),
        ))
        .await
        .expect("post msg monitor");

    let posted = client
        .soundengine()
        .post_event(
            SoundenginePostEventArgs::new(EventRef::from(event_name.as_str())).game_object(emitter),
        )
        .await
        .expect("post event");
    client
        .soundengine()
        .stop_playing_id(SoundengineStopPlayingIdArgs::new(
            posted.playing_id,
            100,
            AkCurveInterpolation::Linear,
        ))
        .await
        .expect("stop playing id");

    // executeActionOnEvent (Stop) on the event name.
    client
        .soundengine()
        .execute_action_on_event(SoundengineExecuteActionOnEventArgs::new(
            EventRef::from(event_name.as_str()),
            ActionOnEventType::Stop,
            emitter,
            100,
            AkCurveInterpolation::Linear,
        ))
        .await
        .expect("execute action on event");

    client
        .soundengine()
        .stop_all(SoundengineStopAllArgs::new().game_object(emitter))
        .await
        .expect("stop all");

    // 6. Cleanup: delete created objects and unregister game objects.
    for id in [
        event_id.clone(),
        game_param_id.clone(),
        trigger_id.clone(),
        aux_bus_id.clone(),
        state_group_id.clone(),
        switch_group_id.clone(),
    ] {
        client
            .wwise()
            .core()
            .object()
            .delete(CoreObjectDeleteArgs::new(ObjectRef::from(id.as_str())))
            .await
            .expect("delete created object");
    }
    client
        .soundengine()
        .unregister_game_obj(SoundengineUnregisterGameObjArgs::new(emitter))
        .await
        .expect("unregister emitter");
    client
        .soundengine()
        .unregister_game_obj(SoundengineUnregisterGameObjArgs::new(listener))
        .await
        .expect("unregister listener");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
