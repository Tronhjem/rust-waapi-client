use serde_json::json;
use waapi_client::api::generated::ak::soundengine::{
    SoundenginePostEventArgs, SoundengineRegisterGameObjArgs, SoundengineSetDefaultListenersArgs,
    SoundengineSetPositionArgs, SoundengineSetRtpcValueArgs, SoundengineUnregisterGameObjArgs,
};
use waapi_client::api::generated::ak::wwise::core::CoreGetInfoResponse;
use waapi_client::api::generated::ak::wwise::core::audio::{
    CoreAudioImportArgs, CoreAudioImportOptions,
};
use waapi_client::api::generated::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs, CoreObjectGetOptions,
    CoreObjectGetResponse, CoreObjectSetArgs, CoreObjectSetItem, CoreObjectSetResponse,
};
use waapi_client::api::generated::ak::wwise::core::project::CoreProjectSaveArgs;
use waapi_client::api::generated::ak::wwise::ui::{
    UiGetSelectedObjectsOptions, UiGetSelectedObjectsResponse,
};
use waapi_client::api::generated::primitives::{
    ChildObject, GameObjectRef, ImportDefaults, ImportItem, ImportOperation, ObjectQuery,
    ObjectReturnField, ObjectType, OnNameConflict, PlatformRef, Position, Property, PropertyValue,
    RtpcRef, Transform, TransformSelect, Vector3,
};
use waapi_client::api::primitives::ObjectRef;

#[test]
fn object_get_args_serializes_to_expected_shape() {
    let args = CoreObjectGetArgs::new(ObjectQuery::from(
        r"\Actor-Mixer Hierarchy\Default Work Unit",
    ))
    .transform(vec![
        Transform::select(TransformSelect::Children),
        Transform::distinct(),
        Transform::range(0, 10),
        Transform::where_name_contains("Footstep"),
        Transform::where_name_matches(".*_01$"),
        Transform::where_type_is_in([ObjectType::Sound, ObjectType::ActorMixer]),
        Transform::where_category_is_in(["Actor-Mixer Hierarchy"]),
    ]);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(
        value["from"]["path"],
        json!([r"\Actor-Mixer Hierarchy\Default Work Unit"])
    );
    assert_eq!(value["transform"][0]["select"], json!(["children"]));
    assert_eq!(value["transform"][1], json!("distinct"));
    assert_eq!(value["transform"][2]["range"], json!([0, 10]));
    assert_eq!(
        value["transform"][3]["where"],
        json!(["name:contains", "Footstep"])
    );
    assert_eq!(
        value["transform"][4]["where"],
        json!(["name:matches", ".*_01$"])
    );
    assert_eq!(
        value["transform"][5]["where"],
        json!(["type:isIn", ["Sound", "ActorMixer"]])
    );
    assert_eq!(
        value["transform"][6]["where"],
        json!(["category:isIn", ["Actor-Mixer Hierarchy"]])
    );
}

#[test]
fn object_get_options_serializes_return_keyword() {
    let options = CoreObjectGetOptions::new()
        .return_fields(vec!["id".to_string(), "name".to_string()])
        .platform(PlatformRef::Name("Windows".to_string()));

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id", "name"]]);
    assert_eq!(value["platform"], "Windows");
}

#[test]
fn object_return_options_accept_typed_fields() {
    let options = CoreObjectGetOptions::new().return_fields([
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Property(Property::Volume),
    ]);

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json!(["id", "name", "@Volume"]));
}

#[test]
fn object_create_args_serializes_required_and_optional_fields() {
    let args = CoreObjectCreateArgs::new(
        ObjectRef::path(r"\Actor-Mixer Hierarchy"),
        ObjectType::Folder,
        "MyFolder".to_string(),
    )
    .on_name_conflict(OnNameConflict::Rename)
    .children(vec![
        ChildObject::new(ObjectType::Sound, "MySound")
            .property(Property::Volume, PropertyValue::Number(-3.0)),
    ]);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["parent"], r"\Actor-Mixer Hierarchy");
    assert_eq!(value["type"], "Folder");
    assert_eq!(value["name"], "MyFolder");
    assert_eq!(value["onNameConflict"], "rename");
    assert_eq!(value["children"][0]["type"], "Sound");
    assert_eq!(value["children"][0]["name"], "MySound");
    assert_eq!(value["children"][0]["@Volume"], -3.0);
}

#[test]
fn object_delete_args_serializes_object_ref() {
    let args = CoreObjectDeleteArgs::new(ObjectRef::guid("{00000000-0000-0000-0000-000000000001}"));

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["object"], "{00000000-0000-0000-0000-000000000001}");
}

#[test]
fn soundengine_post_event_args_serializes() {
    let args =
        SoundenginePostEventArgs::new("Play_Test".into()).game_object(GameObjectRef::from(42u64));

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["event"], "Play_Test");
    assert_eq!(value["gameObject"], 42);
}

#[test]
fn soundengine_set_rtpc_value_args_serializes() {
    let args = SoundengineSetRtpcValueArgs::new("Volume".into(), 0.75)
        .game_object(GameObjectRef::from(100u32));

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["rtpc"], "Volume");
    assert_eq!(value["value"], 0.75);
    assert_eq!(value["gameObject"], 100);
}

#[test]
fn soundengine_register_position_and_listeners_serialize() {
    let reg = serde_json::to_value(SoundengineRegisterGameObjArgs::new(
        GameObjectRef::game_object(1),
        "Listener".into(),
    ))
    .unwrap();
    assert_eq!(reg, json!({ "gameObject": 1, "name": "Listener" }));

    let listeners = serde_json::to_value(SoundengineSetDefaultListenersArgs::new(vec![
        GameObjectRef::game_object(1),
    ]))
    .unwrap();
    assert_eq!(listeners, json!({ "listeners": [1] }));

    let pos_val = Vector3::forward * -2.0 + Vector3::right * 2.0;
    let set_pos = serde_json::to_value(SoundengineSetPositionArgs::new(
        GameObjectRef::game_object(2),
        Position::new(pos_val),
    ))
    .unwrap();
    assert_eq!(
        set_pos,
        json!({
            "gameObject": 2,
            "position": {
                "orientationFront": { "x": 0.0, "y": 0.0, "z": 1.0 },
                "orientationTop": { "x": 0.0, "y": 1.0, "z": 0.0 },
                "position": { "x": 2.0, "y": 0.0, "z": -2.0 }
            }
        })
    );

    let unreg = serde_json::to_value(SoundengineUnregisterGameObjArgs::new(
        GameObjectRef::game_object(2),
    ))
    .unwrap();
    assert_eq!(unreg, json!({ "gameObject": 2 }));
}

#[test]
fn audio_import_args_serializes_imports_and_defaults() {
    let mut item = ImportItem::new(r"\Actor-Mixer Hierarchy\Default Work Unit\MySound".to_string());
    item.audio_file = Some(r"C:\Audio\MySound.wav".to_string());

    let defaults = ImportDefaults {
        object_type: Some(ObjectType::Sound),
        ..Default::default()
    };

    let args = CoreAudioImportArgs::new(vec![item])
        .import_operation(ImportOperation::CreateNew)
        .default(defaults);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["importOperation"], "createNew");
    assert_eq!(
        value["imports"][0]["objectPath"],
        r"\Actor-Mixer Hierarchy\Default Work Unit\MySound"
    );
    assert_eq!(value["imports"][0]["audioFile"], r"C:\Audio\MySound.wav");
    assert_eq!(value["default"]["objectType"], "Sound");
}

#[test]
fn audio_import_options_serializes_return_and_platform() {
    let options = CoreAudioImportOptions::new()
        .return_fields(vec!["id".to_string(), "name".to_string()])
        .platform(PlatformRef::Guid("{GUID}".to_string()));

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id", "name"]]);
    assert_eq!(value["platform"], "{GUID}");
}

#[test]
fn project_save_args_serializes_empty_args() {
    let args = CoreProjectSaveArgs::new();
    let value = serde_json::to_value(&args).unwrap();
    assert!(value.as_object().unwrap().is_empty());
}

#[test]
fn ui_get_selected_objects_options_serializes() {
    let options = UiGetSelectedObjectsOptions::new().return_fields(vec!["id".to_string()]);
    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id"]]);
}

#[test]
fn get_info_response_deserializes() {
    let json = json!({
        "displayName": "Wwise",
        "platform": "x64",
        "version": {
            "displayName": "2024.1.7",
            "year": 2024,
            "major": 1,
            "minor": 7,
            "build": 8863,
            "nickname": "",
            "schema": 108
        },
        "sessionId": "{FF59687C-48CF-4385-B1C5-CE84B0A63880}",
        "apiVersion": 1,
        "branch": "v2024.1/wwise_main",
        "copyright": "Copyright 2024",
        "configuration": "release",
        "isCommandLine": false,
        "processId": 12345,
        "processPath": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring/x64/Release/bin/Wwise.exe",
        "directories": {
            "user": "%APPDATA%/Audiokinetic/Wwise/",
            "install": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863",
            "authoring": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring",
            "bin": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring/x64/Release/bin",
            "help": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring/Help"
        }
    });

    let resp: CoreGetInfoResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.display_name, "Wwise");
    assert_eq!(resp.platform, "x64");
    assert_eq!(resp.version.display_name, "2024.1.7");
    assert_eq!(resp.version.schema, 108);
    assert_eq!(
        resp.session_id,
        "{FF59687C-48CF-4385-B1C5-CE84B0A63880}".to_string()
    );
    assert_eq!(resp.api_version, 1.0);
    assert_eq!(resp.configuration, "release");
    assert_eq!(resp.process_id, 12345);
    assert_eq!(
        resp.directories.install,
        "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863"
    );
}

#[test]
fn object_get_response_deserializes() {
    // Real wire shape: the object array is returned under the "return" key.
    let json = json!({
        "return": [
            {
                "id": "{GUID-1}",
                "name": "MySound",
                "type": "Sound",
                "shortId": 12345
            }
        ]
    });

    let resp: CoreObjectGetResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.objects.len(), 1);
    let obj = &resp.objects[0];
    assert_eq!(obj.id, Some("{GUID-1}".to_string()));
    assert_eq!(obj.name, Some("MySound".to_string()));
    assert_eq!(obj.object_type, Some(ObjectType::Sound));
    assert_eq!(obj.short_id, Some(12345));
}

#[test]
fn ui_get_selected_objects_response_deserializes() {
    let json = json!({
        "objects": [
            {
                "id": "{GUID-2}",
                "name": "MyEvent",
                "type": "Event"
            }
        ],
        "return": ["id", "name", "type"]
    });

    let resp: UiGetSelectedObjectsResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.objects.len(), 1);
    assert_eq!(resp.objects[0].name, Some("MyEvent".to_string()));
}

#[test]
fn rtpc_ref_serializes_name_guid_and_short_id() {
    assert_eq!(
        serde_json::to_value(RtpcRef::from("Volume")).unwrap(),
        "Volume"
    );
    assert_eq!(
        serde_json::to_value(RtpcRef::Guid("{GUID}".to_string())).unwrap(),
        "{GUID}"
    );
    assert_eq!(serde_json::to_value(RtpcRef::from(123u32)).unwrap(), 123);
}

#[test]
fn object_ref_deserialization_roundtrip() {
    let name: waapi_client::api::primitives::ObjectRef =
        serde_json::from_value(json!("Play_Test")).unwrap();
    assert_eq!(serde_json::to_value(name).unwrap(), "Play_Test");

    let guid: waapi_client::api::primitives::ObjectRef =
        serde_json::from_value(json!("{00000000-0000-0000-0000-000000000001}")).unwrap();
    assert_eq!(
        serde_json::to_value(guid).unwrap(),
        "{00000000-0000-0000-0000-000000000001}"
    );

    let path: waapi_client::api::primitives::ObjectRef =
        serde_json::from_value(json!(r"\Events\Default Work Unit\Play_Test")).unwrap();
    assert_eq!(
        serde_json::to_value(path).unwrap(),
        r"\Events\Default Work Unit\Play_Test"
    );
}

#[test]
fn object_set_args_serializes_to_expected_shape() {
    let args = CoreObjectSetArgs::new(vec![
        CoreObjectSetItem::new(ObjectRef::path(
            r"\Actor-Mixer Hierarchy\Default Work Unit\MySound",
        ))
        .name("NewName")
        .notes("set from the typed API")
        .property(Property::Volume, -6.0)
        .property(
            Property::OutputBus,
            r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus",
        ),
    ])
    .list_mode(waapi_client::api::generated::primitives::ListMode::Append);

    let value = serde_json::to_value(&args).unwrap();
    let item = &value["objects"][0];
    assert_eq!(
        item["object"],
        r"\Actor-Mixer Hierarchy\Default Work Unit\MySound"
    );
    assert_eq!(item["name"], "NewName");
    assert_eq!(item["notes"], "set from the typed API");
    assert_eq!(item["@Volume"], -6.0);
    assert_eq!(
        item["@OutputBus"],
        r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus"
    );
    assert!(item.get("children").is_none());
    assert_eq!(value["listMode"], "append");
}

#[test]
fn object_set_response_deserializes_with_extra_fields() {
    let raw = json!({
        "objects": [
            {
                "id": "{BAE891BA-E025-43ED-A4D1-4DC55C68E7CA}",
                "name": "Sound",
                "@RTPC": [{"id": "{31DA7AE1-B215-4689-A5F7-67A9C5B3DFDC}", "name": ""}]
            },
            {
                "id": "{7A12D08F-B0D9-4403-9EFA-2E6338C197C1}",
                "name": "Default Work Unit",
                "children": [{"id": "{66666666-7777-8888-9999-AAAAAAAAAAAA}", "name": "Boom"}]
            }
        ]
    });

    let response: CoreObjectSetResponse = serde_json::from_value(raw).unwrap();
    assert_eq!(response.objects.len(), 2);
    assert_eq!(
        response.objects[0].id.as_deref(),
        Some("{BAE891BA-E025-43ED-A4D1-4DC55C68E7CA}")
    );
    assert!(response.objects[0].property(Property::RTPC).is_some());
    assert_eq!(response.objects[1].children.len(), 1);
    assert_eq!(
        response.objects[1].children[0].name.as_deref(),
        Some("Boom")
    );
}

#[test]
fn property_enum_maps_to_bare_and_at_prefixed_strings() {
    assert_eq!(Property::Volume.as_str(), "Volume");
    assert_eq!(Property::Volume.as_at_str(), "@Volume");
    assert_eq!(Property::Volume.as_ref(), "Volume");
    assert_eq!(Property::_3DSpatialization.as_str(), "3DSpatialization");
    assert_eq!(Property::_3DSpatialization.as_at_str(), "@3DSpatialization");
    assert_eq!(serde_json::to_value(Property::Pitch).unwrap(), "Pitch");
    assert_eq!(
        serde_json::from_value::<Property>(json!("Volume")).unwrap(),
        Property::Volume
    );
    assert_eq!(
        serde_json::from_value::<Property>(json!("@Volume")).unwrap(),
        Property::Volume
    );
}

#[test]
fn object_type_enum_maps_to_names_class_ids_and_queries() {
    assert_eq!(ObjectType::Sound.as_str(), "Sound");
    assert_eq!(ObjectType::Sound.as_ref(), "Sound");
    assert_eq!(ObjectType::Sound.class_id(), 65552);
    assert_eq!(ObjectType::from_class_id(65552), Some(ObjectType::Sound));
    assert_eq!(ObjectType::ActorMixer.class_id(), 524304);
    assert_eq!(
        "ActorMixer".parse::<ObjectType>().unwrap(),
        ObjectType::ActorMixer
    );

    let query = ObjectQuery::from(ObjectType::Sound);
    let value = serde_json::to_value(&query).unwrap();
    assert_eq!(value["ofType"], json!(["Sound"]));
}

#[test]
fn waapi_uri_enums_map_to_const_strings() {
    use waapi_client::{WaapiFunction, WaapiTopic, waapi_function_api, waapi_topics_api};

    assert_eq!(
        WaapiFunction::ak_wwise_core_getInfo.as_str(),
        waapi_function_api::ak::wwise::core::getInfo
    );
    assert_eq!(
        WaapiFunction::ak_wwise_core_object_move.as_str(),
        waapi_function_api::ak::wwise::core::object::move_
    );
    assert_eq!(
        WaapiTopic::ak_wwise_core_object_created.as_str(),
        waapi_topics_api::ak::wwise::core::object::created
    );
}

#[test]
fn undo_args_serialize() {
    use waapi_client::api::generated::ak::wwise::core::undo::{
        CoreUndoCancelGroupArgs, CoreUndoEndGroupArgs,
    };

    let end = CoreUndoEndGroupArgs::new("Batch Rename".to_string());
    assert_eq!(
        serde_json::to_value(end).unwrap(),
        json!({"displayName": "Batch Rename"})
    );

    let cancel = CoreUndoCancelGroupArgs::new().undo(true);
    assert_eq!(serde_json::to_value(cancel).unwrap(), json!({"undo": true}));
}

#[test]
fn transport_args_and_responses_roundtrip() {
    use waapi_client::api::generated::ak::wwise::core::transport::{
        CoreTransportCreateArgs, CoreTransportCreateResponse, CoreTransportDestroyArgs,
        CoreTransportExecuteActionArgs, CoreTransportGetListResponse, CoreTransportGetStateArgs,
        CoreTransportGetStateResponse, CoreTransportPrepareArgs, CoreTransportUseOriginalsArgs,
    };
    use waapi_client::api::generated::primitives::{
        GameObjectRef, TransportAction, TransportState,
    };

    let create =
        CoreTransportCreateArgs::new(ObjectRef::guid("{00000000-0000-0000-0000-000000000001}"))
            .game_object(GameObjectRef::game_object(100));
    assert_eq!(
        serde_json::to_value(create).unwrap(),
        json!({
            "object": "{00000000-0000-0000-0000-000000000001}",
            "gameObject": 100
        })
    );

    let create_resp: CoreTransportCreateResponse =
        serde_json::from_value(json!({"transport": 42})).unwrap();
    assert_eq!(create_resp.transport, 42);

    let exec = CoreTransportExecuteActionArgs::new(TransportAction::Play).transport(42u32);
    assert_eq!(
        serde_json::to_value(exec).unwrap(),
        json!({"action": "play", "transport": 42})
    );

    let get_state = CoreTransportGetStateArgs::new(42);
    assert_eq!(
        serde_json::to_value(get_state).unwrap(),
        json!({"transport": 42})
    );

    let state_resp: CoreTransportGetStateResponse =
        serde_json::from_value(json!({"state": "playing"})).unwrap();
    assert_eq!(state_resp.state, TransportState::Playing);

    let list_resp: CoreTransportGetListResponse = serde_json::from_value(json!({
        "list": [
            {
                "transport": 42,
                "object": "{00000000-0000-0000-0000-000000000001}",
                "gameObject": 100
            }
        ]
    }))
    .unwrap();
    assert_eq!(list_resp.list.len(), 1);
    assert_eq!(list_resp.list[0].transport, 42);
    assert_eq!(
        list_resp.list[0].object,
        "{00000000-0000-0000-0000-000000000001}"
    );

    let prep = CoreTransportPrepareArgs::new(ObjectRef::from("Sound:MySound"));
    assert_eq!(
        serde_json::to_value(prep).unwrap(),
        json!({"object": "Sound:MySound"})
    );

    let orig = CoreTransportUseOriginalsArgs::new(true);
    assert_eq!(serde_json::to_value(orig).unwrap(), json!({"enable": true}));

    let destroy = CoreTransportDestroyArgs::new(42);
    assert_eq!(
        serde_json::to_value(destroy).unwrap(),
        json!({"transport": 42})
    );
}

#[test]
fn object_helpers_and_attenuation_curve_roundtrip() {
    use waapi_client::api::generated::ak::wwise::core::object::{
        CoreObjectCopyArgs, CoreObjectDiffArgs, CoreObjectDiffResponse,
        CoreObjectGetAttenuationCurveArgs, CoreObjectGetAttenuationCurveResponse,
        CoreObjectMoveArgs, CoreObjectPastePropertiesArgs, CoreObjectSetAttenuationCurveArgs,
        CoreObjectSetNameArgs, CoreObjectSetNotesArgs, CoreObjectSetPropertyArgs,
        CoreObjectSetRandomizerArgs, CoreObjectSetReferenceArgs,
    };
    use waapi_client::api::generated::primitives::{
        AttenuationCurveType, AttenuationCurveUse, CurvePoint2D, CurveShape, OnNameConflict,
        PasteMode, PropertyValue,
    };

    let mv = CoreObjectMoveArgs::new(ObjectRef::guid("{OBJ-1}"), ObjectRef::guid("{PARENT-2}"))
        .on_name_conflict(OnNameConflict::Rename);
    assert_eq!(
        serde_json::to_value(mv).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "parent": "{PARENT-2}",
            "onNameConflict": "rename"
        })
    );

    let cp = CoreObjectCopyArgs::new(ObjectRef::guid("{OBJ-1}"), ObjectRef::guid("{PARENT-2}"))
        .on_name_conflict(OnNameConflict::Replace);
    assert_eq!(
        serde_json::to_value(cp).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "parent": "{PARENT-2}",
            "onNameConflict": "replace"
        })
    );

    let set_name = CoreObjectSetNameArgs::new(ObjectRef::guid("{OBJ-1}"), "Renamed".to_string());
    assert_eq!(
        serde_json::to_value(set_name).unwrap(),
        json!({"object": "{OBJ-1}", "value": "Renamed"})
    );

    let set_notes = CoreObjectSetNotesArgs::new(ObjectRef::guid("{OBJ-1}"), "Notes".to_string());
    assert_eq!(
        serde_json::to_value(set_notes).unwrap(),
        json!({"object": "{OBJ-1}", "value": "Notes"})
    );

    let set_prop = CoreObjectSetPropertyArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        Property::Volume,
        PropertyValue::from(-6.0),
    );
    assert_eq!(
        serde_json::to_value(set_prop).unwrap(),
        json!({"object": "{OBJ-1}", "property": "Volume", "value": -6.0})
    );

    let set_ref = CoreObjectSetReferenceArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        Property::OutputBus,
        ObjectRef::path(r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus"),
    );
    assert_eq!(
        serde_json::to_value(set_ref).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "reference": "OutputBus",
            "value": r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus"
        })
    );

    let rand_args = CoreObjectSetRandomizerArgs::new(ObjectRef::guid("{OBJ-1}"), Property::Volume)
        .enabled(true)
        .min(-3.0)
        .max(3.0);
    assert_eq!(
        serde_json::to_value(rand_args).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "property": "Volume",
            "enabled": true,
            "min": -3.0,
            "max": 3.0
        })
    );

    let points = vec![
        CurvePoint2D::new(0.0, 0.0, CurveShape::Linear),
        CurvePoint2D::new(50.0, -6.0, CurveShape::Log2),
        CurvePoint2D::new(100.0, -200.0, CurveShape::Constant),
    ];
    let set_curve = CoreObjectSetAttenuationCurveArgs::new(
        ObjectRef::guid("{ATTEN-1}"),
        AttenuationCurveType::VolumeDryUsage,
        AttenuationCurveUse::Custom,
        points.clone(),
    );
    let set_curve_json = serde_json::to_value(&set_curve).unwrap();
    assert_eq!(set_curve_json["curveType"], "VolumeDryUsage");
    assert_eq!(set_curve_json["use"], "Custom");
    assert_eq!(set_curve_json["points"].as_array().unwrap().len(), 3);
    assert_eq!(set_curve_json["points"][1]["shape"], "Log2");

    let get_curve = CoreObjectGetAttenuationCurveArgs::new(
        ObjectRef::guid("{ATTEN-1}"),
        AttenuationCurveType::VolumeDryUsage,
    );
    assert_eq!(
        serde_json::to_value(get_curve).unwrap(),
        json!({"object": "{ATTEN-1}", "curveType": "VolumeDryUsage"})
    );

    let curve_resp: CoreObjectGetAttenuationCurveResponse = serde_json::from_value(json!({
        "curveType": "VolumeDryUsage",
        "use": "Custom",
        "points": [
            {"x": 0.0, "y": 0.0, "shape": "Linear"},
            {"x": 50.0, "y": -6.0, "shape": "Log2"},
            {"x": 100.0, "y": -200.0, "shape": "Constant"}
        ]
    }))
    .unwrap();
    assert_eq!(curve_resp.curve_type, AttenuationCurveType::VolumeDryUsage);
    assert_eq!(curve_resp.use_, AttenuationCurveUse::Custom);
    assert_eq!(curve_resp.points, points);

    let diff = CoreObjectDiffArgs::new(ObjectRef::guid("{SRC}"), ObjectRef::guid("{DST}"));
    assert_eq!(
        serde_json::to_value(diff).unwrap(),
        json!({"source": "{SRC}", "target": "{DST}"})
    );
    let diff_resp: CoreObjectDiffResponse = serde_json::from_value(json!({
        "properties": ["Volume", "Pitch"],
        "lists": []
    }))
    .unwrap();
    assert_eq!(diff_resp.properties, vec!["Volume", "Pitch"]);
    assert!(diff_resp.lists.is_empty());

    let paste = CoreObjectPastePropertiesArgs::new(
        ObjectRef::guid("{SRC}"),
        vec![ObjectRef::guid("{DST}")],
    )
    .paste_mode(PasteMode::ReplaceEntire)
    .inclusion(vec![Property::Volume, Property::Pitch]);
    assert_eq!(
        serde_json::to_value(paste).unwrap(),
        json!({
            "source": "{SRC}",
            "targets": ["{DST}"],
            "pasteMode": "replaceEntire",
            "inclusion": ["Volume", "Pitch"]
        })
    );
}

#[test]
fn switch_container_assignments_roundtrip() {
    use waapi_client::api::generated::ak::wwise::core::switchContainer::{
        CoreSwitchContainerAddAssignmentArgs, CoreSwitchContainerGetAssignmentsArgs,
        CoreSwitchContainerGetAssignmentsResponse, CoreSwitchContainerRemoveAssignmentArgs,
    };
    use waapi_client::api::generated::primitives::SwitchContainerAssignment;

    let add = CoreSwitchContainerAddAssignmentArgs::new(
        ObjectRef::guid("{CHILD-1}"),
        ObjectRef::guid("{SWITCH-1}"),
    );
    assert_eq!(
        serde_json::to_value(add).unwrap(),
        json!({"child": "{CHILD-1}", "stateOrSwitch": "{SWITCH-1}"})
    );

    let rem = CoreSwitchContainerRemoveAssignmentArgs::new(
        ObjectRef::guid("{CHILD-1}"),
        ObjectRef::guid("{SWITCH-1}"),
    );
    assert_eq!(
        serde_json::to_value(rem).unwrap(),
        json!({"child": "{CHILD-1}", "stateOrSwitch": "{SWITCH-1}"})
    );

    let get = CoreSwitchContainerGetAssignmentsArgs::new(ObjectRef::guid("{CONTAINER-1}"));
    assert_eq!(
        serde_json::to_value(get).unwrap(),
        json!({"id": "{CONTAINER-1}"})
    );

    let resp: CoreSwitchContainerGetAssignmentsResponse = serde_json::from_value(json!({
        "return": [
            {
                "child": "{CHILD-1}",
                "stateOrSwitch": "{SWITCH-1}"
            }
        ]
    }))
    .unwrap();
    assert_eq!(
        resp.assignments,
        vec![SwitchContainerAssignment {
            child: "{CHILD-1}".to_string(),
            state_or_switch: "{SWITCH-1}".to_string(),
        }]
    );
}

#[test]
fn project_info_and_ui_commands_roundtrip() {
    use waapi_client::api::generated::ak::wwise::core::CoreGetProjectInfoResponse;
    use waapi_client::api::generated::ak::wwise::ui::commands::{
        UiCommandsExecuteArgs, UiCommandsGetCommandsResponse,
    };
    use waapi_client::api::generated::ak::wwise::ui::project::{
        UiProjectCloseArgs, UiProjectCloseResponse, UiProjectOpenArgs,
    };
    use waapi_client::api::generated::primitives::ProjectMigrationAction;

    let proj: CoreGetProjectInfoResponse = serde_json::from_value(json!({
        "name": "WwiseProject",
        "displayTitle": "WwiseProject*",
        "path": "/Projects/WwiseProject/WwiseProject.wproj",
        "id": "{PROJ-ID}",
        "isDirty": true,
        "currentLanguageId": "{LANG-1}",
        "referenceLanguageId": "{LANG-1}",
        "currentPlatformId": "{PLAT-1}",
        "defaultConversion": {
            "id": "{CONV-1}",
            "name": "Default Conversion Settings"
        },
        "platforms": [
            {
                "id": "{PLAT-1}",
                "name": "Mac",
                "baseName": "Mac",
                "baseDisplayName": "macOS",
                "soundBankPath": "/Projects/WwiseProject/GeneratedSoundBanks/Mac",
                "copiedMediaPath": "/Projects/WwiseProject/GeneratedSoundBanks/Mac"
            }
        ],
        "languages": [
            {
                "id": "{LANG-1}",
                "name": "English(US)",
                "shortId": 684519430
            }
        ],
        "directories": {
            "root": "/Projects/WwiseProject",
            "cache": "/Projects/WwiseProject/.cache",
            "originals": "/Projects/WwiseProject/Originals",
            "soundBankOutputRoot": "/Projects/WwiseProject/GeneratedSoundBanks",
            "commands": "/Projects/WwiseProject/Commands",
            "properties": "/Projects/WwiseProject/Properties"
        }
    }))
    .unwrap();
    assert_eq!(proj.name, "WwiseProject");
    assert_eq!(proj.platforms[0].base_display_name, "macOS");
    assert_eq!(proj.languages[0].short_id, 684519430);
    assert_eq!(proj.directories.root, "/Projects/WwiseProject");

    let exec = UiCommandsExecuteArgs::new("FindInProjectExplorerSyncGroup1".to_string())
        .objects(vec![ObjectRef::guid("{OBJ-1}")]);
    assert_eq!(
        serde_json::to_value(exec).unwrap(),
        json!({
            "command": "FindInProjectExplorerSyncGroup1",
            "objects": ["{OBJ-1}"]
        })
    );

    let cmds: UiCommandsGetCommandsResponse = serde_json::from_value(json!({
        "commands": ["Inspect", "FindInProjectExplorerSyncGroup1"]
    }))
    .unwrap();
    assert_eq!(cmds.commands.len(), 2);

    let open = UiProjectOpenArgs::new("/Projects/WwiseProject/WwiseProject.wproj".to_string())
        .on_migration_required(ProjectMigrationAction::Migrate)
        .bypass_save(true);
    assert_eq!(
        serde_json::to_value(open).unwrap(),
        json!({
            "path": "/Projects/WwiseProject/WwiseProject.wproj",
            "onMigrationRequired": "migrate",
            "bypassSave": true
        })
    );

    let close = UiProjectCloseArgs::new().bypass_save(true);
    assert_eq!(
        serde_json::to_value(close).unwrap(),
        json!({"bypassSave": true})
    );
    let close_resp: UiProjectCloseResponse =
        serde_json::from_value(json!({"hadProjectOpen": true})).unwrap();
    assert!(close_resp.had_project_open);
}
