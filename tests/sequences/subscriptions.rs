use std::time::Duration;

use super::{connect_client, sequence_lock};
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectSetArgs, CoreObjectSetItem,
    CoreObjectSetOptions,
};
use wwise_waapi::api::{ObjectReturnField, ObjectReturnOptions, OnNameConflict, WwiseObject};
use wwise_waapi::{Event, ObjectRef, ObjectType, Property, WaapiTopic};

const DEFAULT_WORK_UNIT_PATH: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const SUB_ACTOR_MIXER_NAME: &str = "WaapiSubTestActorMixer";
const SUB_INITIAL_NAME: &str = "WaapiSubTestSound";
const SUB_RENAMED_NAME: &str = "WaapiSubTestSoundRenamed";

async fn recv_matching_object_event(
    rx: &mut tokio::sync::mpsc::UnboundedReceiver<Event>,
    expected_id: &str,
) -> (WwiseObject, Event) {
    tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(event) = rx.recv().await {
            if let Some(obj_val) = event.kwargs.get("object")
                && let Ok(obj) = wwise_waapi::serde_json::from_value::<WwiseObject>(obj_val.clone())
                && obj.id.as_deref() == Some(expected_id)
            {
                return (obj, event);
            }
        }
        panic!("Subscription channel closed before receiving event for {expected_id}");
    })
    .await
    .unwrap_or_else(|_| panic!("Timed out waiting for subscription event for {expected_id}"))
}

#[tokio::test]
async fn object_events_subscription_sequence() {
    let _guard = sequence_lock().await;
    let client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    // Create a dedicated parent ActorMixer (replacing any leftover one from an interrupted run)
    let mixer_created = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(DEFAULT_WORK_UNIT_PATH),
                ObjectType::ActorMixer,
                SUB_ACTOR_MIXER_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("Failed to create parent ActorMixer for subscription sequence");

    let mixer_id = mixer_created.id.expect("Created ActorMixer missing id");

    let sub_opts = ObjectReturnOptions::new().return_fields(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Type,
        ObjectReturnField::Property(Property::Volume),
    ]);

    // 1. Subscribe to object.created, object.nameChanged, and object.preDeleted
    let (created_sub_id, mut created_rx) = client
        .subscribe_typed(
            WaapiTopic::ak_wwise_core_object_created,
            Some(sub_opts.clone()),
        )
        .await
        .expect("Failed to subscribe to ak.wwise.core.object.created");

    let (renamed_sub_id, mut renamed_rx) = client
        .subscribe_typed(
            WaapiTopic::ak_wwise_core_object_nameChanged,
            Some(sub_opts.clone()),
        )
        .await
        .expect("Failed to subscribe to ak.wwise.core.object.nameChanged");

    let (pre_deleted_sub_id, mut pre_deleted_rx) = client
        .subscribe_typed(WaapiTopic::ak_wwise_core_object_preDeleted, Some(sub_opts))
        .await
        .expect("Failed to subscribe to ak.wwise.core.object.preDeleted");

    // 2. Create a Sound object under the test ActorMixer
    let created = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(mixer_id.as_str()),
                ObjectType::Sound,
                SUB_INITIAL_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace)
            .property(Property::Volume, -5.0),
        )
        .await
        .expect("Failed to create test Sound for subscription sequence");
    let sound_id = created.id.expect("Created Sound missing id");

    let (created_obj, _) = recv_matching_object_event(&mut created_rx, &sound_id).await;
    assert_eq!(created_obj.id.as_deref(), Some(sound_id.as_str()));
    assert_eq!(created_obj.object_type, Some(ObjectType::Sound));

    // Wwise also publishes an initial nameChanged event ("" -> SUB_INITIAL_NAME) upon creation
    let (initial_named_obj, initial_name_event) =
        recv_matching_object_event(&mut renamed_rx, &sound_id).await;
    assert_eq!(initial_named_obj.name.as_deref(), Some(SUB_INITIAL_NAME));
    assert_eq!(
        initial_name_event
            .kwargs
            .get("newName")
            .and_then(|v| v.as_str()),
        Some(SUB_INITIAL_NAME)
    );

    // 3. Rename the Sound object and set its volume
    client
        .wwise()
        .core()
        .object()
        .set(
            CoreObjectSetArgs::new(vec![
                CoreObjectSetItem::new(ObjectRef::from(sound_id.as_str()))
                    .name(SUB_RENAMED_NAME)
                    .property(Property::Volume, -9.0),
            ]),
            CoreObjectSetOptions::new(),
        )
        .await
        .expect("Failed to rename test Sound");

    let (renamed_obj, rename_event) = recv_matching_object_event(&mut renamed_rx, &sound_id).await;
    assert_eq!(renamed_obj.name.as_deref(), Some(SUB_RENAMED_NAME));
    assert_eq!(
        rename_event.kwargs.get("oldName").and_then(|v| v.as_str()),
        Some(SUB_INITIAL_NAME)
    );
    assert_eq!(
        rename_event.kwargs.get("newName").and_then(|v| v.as_str()),
        Some(SUB_RENAMED_NAME)
    );

    // 4. Delete the Sound object
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            sound_id.as_str(),
        )))
        .await
        .expect("Failed to delete test Sound");

    let (deleted_obj, _) = recv_matching_object_event(&mut pre_deleted_rx, &sound_id).await;
    assert_eq!(deleted_obj.name.as_deref(), Some(SUB_RENAMED_NAME));
    assert_eq!(
        deleted_obj
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-9.0)
    );

    // 5. Unsubscribe from all topics and shut down
    client
        .unsubscribe(created_sub_id)
        .await
        .expect("Failed to unsubscribe from object.created");
    client
        .unsubscribe(renamed_sub_id)
        .await
        .expect("Failed to unsubscribe from object.nameChanged");
    client
        .unsubscribe(pre_deleted_sub_id)
        .await
        .expect("Failed to unsubscribe from object.preDeleted");

    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            mixer_id.as_str(),
        )))
        .await
        .expect("Failed to delete parent ActorMixer for subscription sequence");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
