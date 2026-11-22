use super::{connect_client, sequence_lock};
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs, CoreObjectGetOptions,
    CoreObjectSetArgs, CoreObjectSetItem, CoreObjectSetOptions,
};
use wwise_waapi::api::{
    ObjectQuery, ObjectReturnField, OnNameConflict, Transform, TransformSelect,
};
use wwise_waapi::{ChildObject, ObjectRef, ObjectType, Property};

const DEFAULT_WORK_UNIT_PATH: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const ACTOR_MIXER_NAME: &str = "WaapiTestActorMixer";
const INITIAL_CONTAINER_NAME: &str = "NewSound";
const RENAMED_CONTAINER_NAME: &str = "RanamedNewSound";

#[tokio::test]
async fn actor_mixer_and_container_lifecycle_sequence() {
    let _guard = sequence_lock().await;
    let client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    let base_return_opts = CoreObjectGetOptions::new().return_fields(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Type,
        ObjectReturnField::Path,
    ]);

    // 1. Get the Default Work Unit
    let default_wu_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(DEFAULT_WORK_UNIT_PATH)),
            base_return_opts.clone(),
        )
        .await
        .expect("Failed to get Default Work Unit");

    assert_eq!(default_wu_resp.objects.len(), 1);
    let default_wu = &default_wu_resp.objects[0];
    assert_eq!(default_wu.name.as_deref(), Some("Default Work Unit"));
    assert_eq!(default_wu.object_type, Some(ObjectType::WorkUnit));
    let default_wu_id = default_wu
        .id
        .as_deref()
        .expect("Default Work Unit missing id");

    // 2. Create a new Actor Mixer under Default Work Unit and get its ID
    let create_am_args = CoreObjectCreateArgs::new(
        ObjectRef::from(default_wu_id),
        ObjectType::ActorMixer,
        ACTOR_MIXER_NAME.to_string(),
    )
    .on_name_conflict(OnNameConflict::Replace);

    let created_am = client
        .wwise()
        .core()
        .object()
        .create(create_am_args)
        .await
        .expect("Failed to create Actor Mixer");
    let actor_mixer_id = created_am.id.expect("Created Actor Mixer missing id");
    assert!(!actor_mixer_id.is_empty());

    let actor_mixer_path = format!(r"{}\{}", DEFAULT_WORK_UNIT_PATH, ACTOR_MIXER_NAME);
    let got_am_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(actor_mixer_path.as_str())),
            base_return_opts.clone(),
        )
        .await
        .expect("Failed to get created Actor Mixer");
    assert_eq!(got_am_resp.objects.len(), 1);
    assert_eq!(
        got_am_resp.objects[0].id.as_deref(),
        Some(actor_mixer_id.as_str())
    );
    // Wwise 2024 reports "ActorMixer"; Wwise 2025 renamed it to "PropertyContainer"
    // (both share class ID 0x80010). Accept either name.
    assert!(matches!(
        got_am_resp.objects[0].object_type,
        Some(ObjectType::ActorMixer) | Some(ObjectType::PropertyContainer)
    ));

    // 3. Create a RandomSequenceContainer as a child of the Actor Mixer named "NewSound"
    let create_container_args = CoreObjectCreateArgs::new(
        ObjectRef::from(actor_mixer_id.as_str()),
        ObjectType::RandomSequenceContainer,
        INITIAL_CONTAINER_NAME.to_string(),
    )
    .on_name_conflict(OnNameConflict::Replace);

    let created_container = client
        .wwise()
        .core()
        .object()
        .create(create_container_args)
        .await
        .expect("Failed to create RandomSequenceContainer");
    let container_id = created_container
        .id
        .expect("Created RandomSequenceContainer missing id");

    // 4. Get the container again through `get` with the name "NewSound"
    let initial_container_path = format!(r"{}\{}", actor_mixer_path, INITIAL_CONTAINER_NAME);
    let got_initial_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(initial_container_path.as_str())),
            base_return_opts.clone(),
        )
        .await
        .expect("Failed to get RandomSequenceContainer by initial name");

    assert_eq!(got_initial_resp.objects.len(), 1);
    let got_initial = &got_initial_resp.objects[0];
    assert_eq!(got_initial.id.as_deref(), Some(container_id.as_str()));
    assert_eq!(got_initial.name.as_deref(), Some(INITIAL_CONTAINER_NAME));
    assert_eq!(
        got_initial.object_type,
        Some(ObjectType::RandomSequenceContainer)
    );

    // 5. Set the name to "RanamedNewSound" and set the volume to -6
    let set_args = CoreObjectSetArgs::new(vec![
        CoreObjectSetItem::new(ObjectRef::from(container_id.as_str()))
            .name(RENAMED_CONTAINER_NAME.to_string())
            .property(Property::Volume, -6.0),
    ]);

    client
        .wwise()
        .core()
        .object()
        .set(set_args, CoreObjectSetOptions::new())
        .await
        .expect("Failed to rename container and set volume");

    // 6. Get the container again with the new name to check naming and volume
    let renamed_container_path = format!(r"{}\{}", actor_mixer_path, RENAMED_CONTAINER_NAME);
    let get_with_volume_opts = CoreObjectGetOptions::new().return_fields(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Type,
        ObjectReturnField::Property(Property::Volume),
    ]);

    let got_renamed_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(renamed_container_path.as_str())),
            get_with_volume_opts,
        )
        .await
        .expect("Failed to get RandomSequenceContainer by renamed name");

    assert_eq!(got_renamed_resp.objects.len(), 1);
    let got_renamed = &got_renamed_resp.objects[0];
    assert_eq!(got_renamed.id.as_deref(), Some(container_id.as_str()));
    assert_eq!(got_renamed.name.as_deref(), Some(RENAMED_CONTAINER_NAME));
    assert_eq!(
        got_renamed.object_type,
        Some(ObjectType::RandomSequenceContainer)
    );
    assert_eq!(
        got_renamed
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-6.0)
    );

    // 7. Delete the container and verify it is removed by checking the parent's children
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            container_id.as_str(),
        )))
        .await
        .expect("Failed to delete RandomSequenceContainer");

    let children_query = CoreObjectGetArgs::new(ObjectQuery::from(actor_mixer_id.as_str()))
        .transform(vec![Transform::Select {
            select: vec![TransformSelect::Children],
        }]);

    let parent_children_resp = client
        .wwise()
        .core()
        .object()
        .get(children_query, base_return_opts)
        .await
        .expect("Failed to query parent Actor Mixer children");

    assert!(
        parent_children_resp
            .objects
            .iter()
            .all(|child| child.id.as_deref() != Some(container_id.as_str())),
        "Deleted container should not be present in parent's children: {:?}",
        parent_children_resp.objects
    );
    assert!(parent_children_resp.objects.is_empty());

    // Clean up the test Actor Mixer itself so the project is left clean
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            actor_mixer_id.as_str(),
        )))
        .await
        .expect("Failed to clean up test Actor Mixer");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}

#[tokio::test]
async fn nested_hierarchy_and_batch_set_sequence() {
    let _guard = sequence_lock().await;
    let client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    const NESTED_MIXER_NAME: &str = "WaapiTestNestedMixer";
    const NESTED_CONTAINER_NAME: &str = "NestedContainer";
    const SOUND_A_NAME: &str = "NestedSoundA";
    const SOUND_B_NAME: &str = "NestedSoundB";
    const SOUND_C_NAME: &str = "NestedSoundC";

    // 1. Create a root ActorMixer with nested children in a single create call
    let create_args = CoreObjectCreateArgs::new(
        ObjectRef::from(DEFAULT_WORK_UNIT_PATH),
        ObjectType::ActorMixer,
        NESTED_MIXER_NAME.to_string(),
    )
    .on_name_conflict(OnNameConflict::Replace)
    .notes("Root test mixer")
    .property(Property::Volume, -2.0)
    .children(vec![
        ChildObject::new(ObjectType::RandomSequenceContainer, NESTED_CONTAINER_NAME)
            .notes("Nested container notes")
            .property(Property::Volume, -4.0)
            .children(vec![
                ChildObject::new(ObjectType::Sound, SOUND_A_NAME).property(Property::Pitch, 100),
                ChildObject::new(ObjectType::Sound, SOUND_B_NAME).property(Property::Lowpass, 20),
            ]),
    ]);

    let created_root = client
        .wwise()
        .core()
        .object()
        .create(create_args)
        .await
        .expect("Failed to create nested ActorMixer hierarchy");

    let root_id = created_root.id.expect("Created root ActorMixer missing id");
    assert_eq!(created_root.children.len(), 1);

    // 2. Query descendants and verify types, class_ids, notes, and initial properties
    let detail_opts = CoreObjectGetOptions::new().return_fields(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Notes,
        ObjectReturnField::Type,
        ObjectReturnField::ClassId,
        ObjectReturnField::Path,
        ObjectReturnField::Property(Property::Volume),
        ObjectReturnField::Property(Property::Pitch),
        ObjectReturnField::Property(Property::Lowpass),
        ObjectReturnField::Property(Property::Highpass),
        ObjectReturnField::Property(Property::IsLoopingEnabled),
    ]);

    let descendants_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(root_id.as_str())).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::Descendants],
                },
            ]),
            detail_opts.clone(),
        )
        .await
        .expect("Failed to query descendants of root ActorMixer");

    assert_eq!(descendants_resp.objects.len(), 3);

    let container = descendants_resp
        .objects
        .iter()
        .find(|o| o.name.as_deref() == Some(NESTED_CONTAINER_NAME))
        .expect("Missing NestedContainer in descendants");
    assert_eq!(
        container.object_type,
        Some(ObjectType::RandomSequenceContainer)
    );
    assert_eq!(
        container.class_id,
        Some(ObjectType::RandomSequenceContainer.class_id())
    );
    assert_eq!(container.notes.as_deref(), Some("Nested container notes"));
    assert_eq!(
        container
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-4.0)
    );
    let container_id = container.id.as_deref().expect("NestedContainer missing id");

    let sound_a = descendants_resp
        .objects
        .iter()
        .find(|o| o.name.as_deref() == Some(SOUND_A_NAME))
        .expect("Missing NestedSoundA in descendants");
    assert_eq!(sound_a.object_type, Some(ObjectType::Sound));
    assert_eq!(sound_a.class_id, Some(ObjectType::Sound.class_id()));
    assert_eq!(
        sound_a.property(Property::Pitch).and_then(|v| v.as_i64()),
        Some(100)
    );
    let sound_a_id = sound_a.id.as_deref().expect("NestedSoundA missing id");

    let sound_b = descendants_resp
        .objects
        .iter()
        .find(|o| o.name.as_deref() == Some(SOUND_B_NAME))
        .expect("Missing NestedSoundB in descendants");
    assert_eq!(
        sound_b.property(Property::Lowpass).and_then(|v| v.as_i64()),
        Some(20)
    );
    let sound_b_id = sound_b.id.as_deref().expect("NestedSoundB missing id");

    // 3. Batch update multiple objects and create a new child via object.set
    let set_args = CoreObjectSetArgs::new(vec![
        CoreObjectSetItem::new(ObjectRef::from(sound_a_id))
            .notes("Updated SoundA notes")
            .property(Property::Volume, -8.5)
            .property(Property::Pitch, -250)
            .property(Property::Highpass, 15)
            .property(Property::IsLoopingEnabled, true),
        CoreObjectSetItem::new(ObjectRef::from(sound_b_id))
            .property(Property::Volume, -3.0)
            .property(Property::Lowpass, 45),
        CoreObjectSetItem::new(ObjectRef::from(container_id))
            .on_name_conflict(OnNameConflict::Replace)
            .children(vec![
                ChildObject::new(ObjectType::Sound, SOUND_C_NAME).property(Property::Volume, -1.0),
            ]),
    ]);

    let set_resp = client
        .wwise()
        .core()
        .object()
        .set(
            set_args,
            CoreObjectSetOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await
        .expect("Failed to execute batch object.set");

    // Verify set response includes the created child under NestedContainer
    let container_set_entry = set_resp
        .objects
        .iter()
        .find(|entry| entry.id.as_deref() == Some(container_id))
        .expect("Missing container entry in CoreObjectSetResponse");
    assert_eq!(container_set_entry.children.len(), 1);
    assert_eq!(
        container_set_entry.children[0].name.as_deref(),
        Some(SOUND_C_NAME)
    );
    assert_eq!(
        container_set_entry.children[0].object_type,
        Some(ObjectType::Sound)
    );

    // 4. Query descendants again and verify all batch updates took effect
    let updated_descendants = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(root_id.as_str())).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::Descendants],
                },
            ]),
            detail_opts,
        )
        .await
        .expect("Failed to re-query descendants after batch set");

    assert_eq!(updated_descendants.objects.len(), 4);

    let updated_a = updated_descendants
        .objects
        .iter()
        .find(|o| o.name.as_deref() == Some(SOUND_A_NAME))
        .expect("Missing NestedSoundA after set");
    assert_eq!(updated_a.notes.as_deref(), Some("Updated SoundA notes"));
    assert_eq!(
        updated_a
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-8.5)
    );
    assert_eq!(
        updated_a.property(Property::Pitch).and_then(|v| v.as_i64()),
        Some(-250)
    );
    assert_eq!(
        updated_a
            .property(Property::Highpass)
            .and_then(|v| v.as_i64()),
        Some(15)
    );
    assert_eq!(
        updated_a
            .property(Property::IsLoopingEnabled)
            .and_then(|v| v.as_bool()),
        Some(true)
    );

    let updated_b = updated_descendants
        .objects
        .iter()
        .find(|o| o.name.as_deref() == Some(SOUND_B_NAME))
        .expect("Missing NestedSoundB after set");
    assert_eq!(
        updated_b
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-3.0)
    );
    assert_eq!(
        updated_b
            .property(Property::Lowpass)
            .and_then(|v| v.as_i64()),
        Some(45)
    );

    let updated_c = updated_descendants
        .objects
        .iter()
        .find(|o| o.name.as_deref() == Some(SOUND_C_NAME))
        .expect("Missing NestedSoundC after set");
    assert_eq!(
        updated_c
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-1.0)
    );

    // 5. Delete the root ActorMixer and verify cleanup
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(root_id.as_str())))
        .await
        .expect("Failed to delete root ActorMixer");

    let wu_children = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(DEFAULT_WORK_UNIT_PATH)).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::Children],
                },
            ]),
            CoreObjectGetOptions::new()
                .return_fields(vec![ObjectReturnField::Id, ObjectReturnField::Name]),
        )
        .await
        .expect("Failed to query Default Work Unit children");

    assert!(
        wu_children
            .objects
            .iter()
            .all(|o| o.id.as_deref() != Some(root_id.as_str())),
        "Deleted root ActorMixer should not exist under Default Work Unit"
    );

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
