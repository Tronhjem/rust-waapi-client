use super::{connect_client, master_audio_bus, sequence_lock};
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectCopyArgs, CoreObjectCopyOptions, CoreObjectCreateArgs, CoreObjectCreateResponse,
    CoreObjectDeleteArgs, CoreObjectDiffArgs, CoreObjectGetArgs, CoreObjectGetAttenuationCurveArgs,
    CoreObjectGetOptions, CoreObjectMoveArgs, CoreObjectPastePropertiesArgs,
    CoreObjectSetAttenuationCurveArgs, CoreObjectSetNameArgs, CoreObjectSetNotesArgs,
    CoreObjectSetPropertyArgs, CoreObjectSetRandomizerArgs, CoreObjectSetReferenceArgs,
};
use wwise_waapi::api::ak::wwise::core::switchContainer::{
    CoreSwitchContainerAddAssignmentArgs, CoreSwitchContainerGetAssignmentsArgs,
    CoreSwitchContainerRemoveAssignmentArgs,
};
use wwise_waapi::api::ak::wwise::core::transport::{
    CoreTransportCreateArgs, CoreTransportDestroyArgs, CoreTransportExecuteActionArgs,
    CoreTransportGetStateArgs, CoreTransportPrepareArgs, CoreTransportUseOriginalsArgs,
};
use wwise_waapi::api::ak::wwise::core::undo::{CoreUndoCancelGroupArgs, CoreUndoEndGroupArgs};
use wwise_waapi::api::ak::wwise::ui::commands::UiCommandsExecuteArgs;
use wwise_waapi::api::{
    AttenuationCurveType, AttenuationCurveUse, CurvePoint2D, CurveShape, ObjectQuery,
    ObjectReturnField, OnNameConflict, PasteMode, SwitchContainerAssignment, TransportAction,
    TransportState,
};
use wwise_waapi::{ChildObject, ObjectRef, ObjectType, Property};

const DEFAULT_ACTOR_MIXER_WU: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const DEFAULT_ATTENUATIONS_WU: &str = r"\Attenuations\Default Work Unit";
const DEFAULT_SWITCHES_WU: &str = r"\Switches\Default Work Unit";

/// End-to-end sequence testing `ak.wwise.core.object.{setName, setNotes, setProperty,
/// setReference, setRandomizer, copy, move, diff, pasteProperties}` and
/// `ak.wwise.core.undo.{beginGroup, endGroup, cancelGroup, undo, redo}`.
#[tokio::test]
async fn undo_and_object_helpers_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to WAAPI on 127.0.0.1:8080; is Wwise open with a project?");

    // 0. Discover the master audio bus (name differs across Wwise versions).
    let (master_bus_id, _) = master_audio_bus(&mut client).await;

    // 1. Self-setup: create a temporary root ActorMixer with SourceSound, FolderA, and FolderB.
    let create_root = CoreObjectCreateArgs::new(
        DEFAULT_ACTOR_MIXER_WU,
        ObjectType::ActorMixer,
        "WaapiPack1WorkflowRoot",
    )
    .on_name_conflict(OnNameConflict::Replace)
    .children(vec![
        ChildObject::new(ObjectType::Sound, "SourceSound"),
        ChildObject::new(ObjectType::ActorMixer, "FolderA"),
        ChildObject::new(ObjectType::ActorMixer, "FolderB")
            .children(vec![ChildObject::new(ObjectType::Sound, "TargetSound")]),
    ]);

    let created: CoreObjectCreateResponse = client
        .wwise()
        .core()
        .object()
        .create(create_root)
        .await
        .expect("create WaapiPack1WorkflowRoot failed");

    let root_id = created.id.expect("root id");
    let source_id = created.children[0].id.clone().expect("SourceSound id");
    let folder_a_id = created.children[1].id.clone().expect("FolderA id");
    let folder_b_id = created.children[2].id.clone().expect("FolderB id");
    let target_id = created.children[2].children[0]
        .id
        .clone()
        .expect("TargetSound id");

    // 2. Rename, set notes, set properties, set reference, and enable randomizer on SourceSound.
    client
        .wwise()
        .core()
        .object()
        .set_name(CoreObjectSetNameArgs::new(&source_id, "ConfiguredSound"))
        .await
        .expect("set_name failed");

    client
        .wwise()
        .core()
        .object()
        .set_notes(CoreObjectSetNotesArgs::new(
            &source_id,
            "Pack 1 helper notes",
        ))
        .await
        .expect("set_notes failed");

    client
        .wwise()
        .core()
        .object()
        .set_property(CoreObjectSetPropertyArgs::new(
            &source_id,
            Property::Volume,
            -9.0,
        ))
        .await
        .expect("set_property Volume failed");

    client
        .wwise()
        .core()
        .object()
        .set_property(CoreObjectSetPropertyArgs::new(
            &source_id,
            Property::Pitch,
            250,
        ))
        .await
        .expect("set_property Pitch failed");

    client
        .wwise()
        .core()
        .object()
        .set_reference(CoreObjectSetReferenceArgs::new(
            &source_id,
            Property::OutputBus,
            &master_bus_id,
        ))
        .await
        .expect("set_reference OutputBus failed");

    client
        .wwise()
        .core()
        .object()
        .set_randomizer(
            CoreObjectSetRandomizerArgs::new(&source_id, Property::Volume)
                .enabled(true)
                .min(-3.0)
                .max(3.0),
        )
        .await
        .expect("set_randomizer Volume failed");

    // Verify ConfiguredSound properties via object.get.
    let get_opts = CoreObjectGetOptions::new().return_fields([
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Notes,
        ObjectReturnField::Parent,
        ObjectReturnField::Property(Property::Volume),
        ObjectReturnField::Property(Property::Pitch),
    ]);
    let get_source = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(source_id.as_str())),
            get_opts.clone(),
        )
        .await
        .expect("get ConfiguredSound failed");
    let src_obj = &get_source.objects[0];
    assert_eq!(src_obj.name.as_deref(), Some("ConfiguredSound"));
    assert_eq!(src_obj.notes.as_deref(), Some("Pack 1 helper notes"));
    assert_eq!(
        src_obj.property(Property::Volume).and_then(|v| v.as_f64()),
        Some(-9.0)
    );
    assert_eq!(
        src_obj.property(Property::Pitch).and_then(|v| v.as_i64()),
        Some(250)
    );

    // 3. Copy ConfiguredSound into FolderA and move that copy into FolderB.
    let copied = client
        .wwise()
        .core()
        .object()
        .copy(
            CoreObjectCopyArgs::new(&source_id, &folder_a_id)
                .on_name_conflict(OnNameConflict::Rename),
            CoreObjectCopyOptions::new().return_fields([
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Parent,
            ]),
        )
        .await
        .expect("object.copy failed");
    let copied_id = copied.id.expect("copied object id");

    // object.copy's response only returns id/name; verify the parent via object.get.
    let get_copied = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(copied_id.as_str())),
            get_opts.clone(),
        )
        .await
        .expect("get copied object failed");
    assert_eq!(
        get_copied.objects[0]
            .parent
            .as_ref()
            .and_then(|p| p.id.as_deref()),
        Some(folder_a_id.as_str())
    );

    let moved = client
        .wwise()
        .core()
        .object()
        .move_(
            CoreObjectMoveArgs::new(&copied_id, &folder_b_id)
                .on_name_conflict(OnNameConflict::Rename),
        )
        .await
        .expect("object.move_ failed");
    assert_eq!(moved.id.as_deref(), Some(copied_id.as_str()));

    let get_moved = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(copied_id.as_str())),
            get_opts.clone(),
        )
        .await
        .expect("get moved object failed");
    assert_eq!(
        get_moved.objects[0]
            .parent
            .as_ref()
            .and_then(|p| p.id.as_deref()),
        Some(folder_b_id.as_str())
    );

    // 4. Diff ConfiguredSound vs TargetSound, then paste Volume and Pitch onto TargetSound.
    let diff_before = client
        .wwise()
        .core()
        .object()
        .diff(CoreObjectDiffArgs::new(&source_id, &target_id))
        .await
        .expect("object.diff before paste failed");
    assert!(diff_before.properties.iter().any(|p| p == "Volume"));
    assert!(diff_before.properties.iter().any(|p| p == "Pitch"));

    client
        .wwise()
        .core()
        .object()
        .paste_properties(
            CoreObjectPastePropertiesArgs::new(&source_id, vec![ObjectRef::from(&target_id)])
                .paste_mode(PasteMode::ReplaceEntire)
                .inclusion(vec![Property::Volume, Property::Pitch]),
        )
        .await
        .expect("object.paste_properties failed");

    let get_target = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(target_id.as_str())),
            get_opts.clone(),
        )
        .await
        .expect("get TargetSound after paste failed");
    assert_eq!(
        get_target.objects[0]
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-9.0)
    );
    assert_eq!(
        get_target.objects[0]
            .property(Property::Pitch)
            .and_then(|v| v.as_i64()),
        Some(250)
    );

    // 5. Undo group cancel (`begin_group` -> modify -> `cancel_group(undo=true)`).
    client
        .wwise()
        .core()
        .undo()
        .begin_group()
        .await
        .expect("undo.begin_group failed");
    client
        .wwise()
        .core()
        .object()
        .set_property(CoreObjectSetPropertyArgs::new(
            &target_id,
            Property::Volume,
            -18.0,
        ))
        .await
        .expect("set_property inside cancelled undo group failed");
    client
        .wwise()
        .core()
        .undo()
        .cancel_group(CoreUndoCancelGroupArgs::new().undo(true))
        .await
        .expect("undo.cancel_group failed");

    let after_cancel = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(target_id.as_str())),
            get_opts.clone(),
        )
        .await
        .expect("get TargetSound after cancel_group failed");
    assert_eq!(
        after_cancel.objects[0]
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-9.0)
    );

    // 6. Undo group commit (`begin_group` -> modify -> `end_group` -> `undo` -> `redo`).
    client
        .wwise()
        .core()
        .undo()
        .begin_group()
        .await
        .expect("undo.begin_group 2 failed");
    client
        .wwise()
        .core()
        .object()
        .set_property(CoreObjectSetPropertyArgs::new(
            &target_id,
            Property::Volume,
            -15.0,
        ))
        .await
        .expect("set_property inside undo group failed");
    client
        .wwise()
        .core()
        .undo()
        .end_group(CoreUndoEndGroupArgs::new("WaapiPack1 Volume Change"))
        .await
        .expect("undo.end_group failed");

    client
        .wwise()
        .core()
        .undo()
        .undo()
        .await
        .expect("undo.undo failed");

    let after_undo = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(target_id.as_str())),
            get_opts.clone(),
        )
        .await
        .expect("get TargetSound after undo failed");
    assert_eq!(
        after_undo.objects[0]
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-9.0)
    );

    client
        .wwise()
        .core()
        .undo()
        .redo()
        .await
        .expect("undo.redo failed");

    let after_redo = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(target_id.as_str())),
            get_opts,
        )
        .await
        .expect("get TargetSound after redo failed");
    assert_eq!(
        after_redo.objects[0]
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-15.0)
    );

    // 7. Cleanup.
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(root_id))
        .await
        .expect("delete WaapiPack1WorkflowRoot failed");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}

/// End-to-end sequence testing `ak.wwise.core.object.{setAttenuationCurve, getAttenuationCurve}`
/// and `ak.wwise.core.switchContainer.{addAssignment, getAssignments, removeAssignment}`.
#[tokio::test]
async fn attenuation_curves_and_switch_container_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to WAAPI on 127.0.0.1:8080; is Wwise open with a project?");

    // 1. Self-setup: create an Attenuation ShareSet, a SwitchGroup with two Switches,
    // and a SwitchContainer with two child Sounds.
    let atten_created = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                DEFAULT_ATTENUATIONS_WU,
                ObjectType::Attenuation,
                "WaapiPack1Attenuation",
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create WaapiPack1Attenuation failed");
    let atten_id = atten_created.id.expect("attenuation id");

    let switch_group_created = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                DEFAULT_SWITCHES_WU,
                ObjectType::SwitchGroup,
                "WaapiPack1SwitchGroup",
            )
            .on_name_conflict(OnNameConflict::Replace)
            .children(vec![
                ChildObject::new(ObjectType::Switch, "SurfaceWood"),
                ChildObject::new(ObjectType::Switch, "SurfaceStone"),
            ]),
        )
        .await
        .expect("create WaapiPack1SwitchGroup failed");
    let switch_group_id = switch_group_created.id.expect("switch group id");
    let switch_wood_id = switch_group_created.children[0]
        .id
        .clone()
        .expect("SurfaceWood id");
    let switch_stone_id = switch_group_created.children[1]
        .id
        .clone()
        .expect("SurfaceStone id");

    let switch_root_created = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                DEFAULT_ACTOR_MIXER_WU,
                ObjectType::ActorMixer,
                "WaapiPack1SwitchRoot",
            )
            .on_name_conflict(OnNameConflict::Replace)
            .children(vec![
                ChildObject::new(ObjectType::SwitchContainer, "FootstepSwitchContainer").children(
                    vec![
                        ChildObject::new(ObjectType::Sound, "FootstepWood"),
                        ChildObject::new(ObjectType::Sound, "FootstepStone"),
                    ],
                ),
            ]),
        )
        .await
        .expect("create WaapiPack1SwitchRoot failed");
    let switch_root_id = switch_root_created.id.expect("switch root id");
    let switch_container_id = switch_root_created.children[0]
        .id
        .clone()
        .expect("FootstepSwitchContainer id");
    let sound_wood_id = switch_root_created.children[0].children[0]
        .id
        .clone()
        .expect("FootstepWood id");
    let sound_stone_id = switch_root_created.children[0].children[1]
        .id
        .clone()
        .expect("FootstepStone id");

    // 2. Set and read back a custom VolumeDryUsage curve on WaapiPack1Attenuation.
    // Note: Wwise normalizes the last point's shape (there is no segment after it),
    // so the constant segment is placed between the first two points.
    let points = vec![
        CurvePoint2D::new(0.0, 0.0, CurveShape::Linear),
        CurvePoint2D::new(50.0, -6.0, CurveShape::Constant),
        CurvePoint2D::new(100.0, -200.0, CurveShape::Linear),
    ];
    client
        .wwise()
        .core()
        .object()
        .set_attenuation_curve(CoreObjectSetAttenuationCurveArgs::new(
            &atten_id,
            AttenuationCurveType::VolumeDryUsage,
            AttenuationCurveUse::Custom,
            points.clone(),
        ))
        .await
        .expect("set_attenuation_curve failed");

    let curve_resp = client
        .wwise()
        .core()
        .object()
        .get_attenuation_curve(CoreObjectGetAttenuationCurveArgs::new(
            &atten_id,
            AttenuationCurveType::VolumeDryUsage,
        ))
        .await
        .expect("get_attenuation_curve failed");
    assert_eq!(curve_resp.curve_type, AttenuationCurveType::VolumeDryUsage);
    assert_eq!(curve_resp.use_, AttenuationCurveUse::Custom);
    assert_eq!(curve_resp.points, points);

    // 3. Wire SwitchGroupOrStateGroup onto FootstepSwitchContainer and manage switch assignments.
    client
        .wwise()
        .core()
        .object()
        .set_reference(CoreObjectSetReferenceArgs::new(
            &switch_container_id,
            Property::SwitchGroupOrStateGroup,
            &switch_group_id,
        ))
        .await
        .expect("set_reference SwitchGroupOrStateGroup failed");

    client
        .wwise()
        .core()
        .switch_container()
        .add_assignment(CoreSwitchContainerAddAssignmentArgs::new(
            &sound_wood_id,
            &switch_wood_id,
        ))
        .await
        .expect("add_assignment wood failed");

    client
        .wwise()
        .core()
        .switch_container()
        .add_assignment(CoreSwitchContainerAddAssignmentArgs::new(
            &sound_stone_id,
            &switch_stone_id,
        ))
        .await
        .expect("add_assignment stone failed");

    let assignments_resp = client
        .wwise()
        .core()
        .switch_container()
        .get_assignments(CoreSwitchContainerGetAssignmentsArgs::new(
            &switch_container_id,
        ))
        .await
        .expect("get_assignments failed");
    assert_eq!(assignments_resp.assignments.len(), 2);
    assert!(
        assignments_resp
            .assignments
            .contains(&SwitchContainerAssignment {
                child: sound_wood_id.clone(),
                state_or_switch: switch_wood_id.clone(),
            })
    );
    assert!(
        assignments_resp
            .assignments
            .contains(&SwitchContainerAssignment {
                child: sound_stone_id.clone(),
                state_or_switch: switch_stone_id.clone(),
            })
    );

    client
        .wwise()
        .core()
        .switch_container()
        .remove_assignment(CoreSwitchContainerRemoveAssignmentArgs::new(
            &sound_wood_id,
            &switch_wood_id,
        ))
        .await
        .expect("remove_assignment wood failed");

    let assignments_after = client
        .wwise()
        .core()
        .switch_container()
        .get_assignments(CoreSwitchContainerGetAssignmentsArgs::new(
            switch_container_id,
        ))
        .await
        .expect("get_assignments after remove failed");
    assert_eq!(
        assignments_after.assignments,
        vec![SwitchContainerAssignment {
            child: sound_stone_id,
            state_or_switch: switch_stone_id,
        }]
    );

    // 4. Cleanup.
    for id in [switch_root_id, switch_group_id, atten_id] {
        client
            .wwise()
            .core()
            .object()
            .delete(CoreObjectDeleteArgs::new(id))
            .await
            .expect("cleanup delete failed");
    }

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}

/// End-to-end sequence testing `ak.wwise.core.getProjectInfo`,
/// `ak.wwise.ui.commands.{getCommands, execute}`, and
/// `ak.wwise.core.transport.{useOriginals, prepare, create, getList, getState, executeAction, destroy}`.
#[tokio::test]
async fn transport_project_info_and_ui_commands_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to WAAPI on 127.0.0.1:8080; is Wwise open with a project?");

    // 1. Query project info.
    let proj_info = client
        .wwise()
        .core()
        .get_project_info()
        .await
        .expect("get_project_info failed");
    assert!(!proj_info.id.is_empty());
    assert!(!proj_info.name.is_empty());
    assert!(proj_info.path.ends_with(".wproj"));
    assert!(!proj_info.platforms.is_empty());
    assert!(!proj_info.languages.is_empty());
    assert!(!proj_info.directories.root.is_empty());

    // 2. Query UI commands list.
    let commands_resp = client
        .wwise()
        .ui()
        .commands()
        .get_commands()
        .await
        .expect("ui.commands.get_commands failed");
    assert!(!commands_resp.commands.is_empty());

    // 3. Self-setup: create a temporary Sound object for Transport and UI command execution.
    let sound_created = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                DEFAULT_ACTOR_MIXER_WU,
                ObjectType::Sound,
                "WaapiPack1TransportSound",
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create WaapiPack1TransportSound failed");
    let sound_id = sound_created.id.expect("transport sound id");

    // 4. Execute a non-destructive UI command targeting the created sound.
    client
        .wwise()
        .ui()
        .commands()
        .execute(
            UiCommandsExecuteArgs::new("FindInProjectExplorerSyncGroup1")
                .objects(vec![ObjectRef::from(&sound_id)]),
        )
        .await
        .expect("ui.commands.execute failed");

    // 5. Exercise Transport lifecycle on the created sound.
    client
        .wwise()
        .core()
        .transport()
        .use_originals(CoreTransportUseOriginalsArgs::new(true))
        .await
        .expect("transport.use_originals(true) failed");

    client
        .wwise()
        .core()
        .transport()
        .prepare(CoreTransportPrepareArgs::new(&sound_id))
        .await
        .expect("transport.prepare failed");

    let transport_created = client
        .wwise()
        .core()
        .transport()
        .create(CoreTransportCreateArgs::new(&sound_id))
        .await
        .expect("transport.create failed");
    let transport_id = transport_created.transport;
    assert!(transport_id > 0);

    let list_resp = client
        .wwise()
        .core()
        .transport()
        .get_list()
        .await
        .expect("transport.get_list failed");
    assert!(
        list_resp
            .list
            .iter()
            .any(|t| t.transport == transport_id && t.object == sound_id)
    );

    let state_resp = client
        .wwise()
        .core()
        .transport()
        .get_state(CoreTransportGetStateArgs::new(transport_id))
        .await
        .expect("transport.get_state failed");
    assert_eq!(state_resp.state, TransportState::Stopped);

    client
        .wwise()
        .core()
        .transport()
        .execute_action(
            CoreTransportExecuteActionArgs::new(TransportAction::Stop).transport(transport_id),
        )
        .await
        .expect("transport.execute_action(Stop) failed");

    client
        .wwise()
        .core()
        .transport()
        .destroy(CoreTransportDestroyArgs::new(transport_id))
        .await
        .expect("transport.destroy failed");

    let list_after = client
        .wwise()
        .core()
        .transport()
        .get_list()
        .await
        .expect("transport.get_list after destroy failed");
    assert!(list_after.list.iter().all(|t| t.transport != transport_id));

    client
        .wwise()
        .core()
        .transport()
        .use_originals(CoreTransportUseOriginalsArgs::new(false))
        .await
        .expect("transport.use_originals(false) failed");

    // 6. Cleanup.
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(sound_id))
        .await
        .expect("delete WaapiPack1TransportSound failed");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
