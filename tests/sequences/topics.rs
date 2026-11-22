use super::{connect_client, sequence_lock};
use std::time::Duration;
use tokio::time::timeout;
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectCreatedOptions, CoreObjectCreatedPayload, CoreObjectDeleteArgs,
    CoreObjectNameChangedOptions, CoreObjectNameChangedPayload, CoreObjectPreDeletedOptions,
    CoreObjectPreDeletedPayload, CoreObjectPropertyChangedOptions,
    CoreObjectPropertyChangedPayload, CoreObjectSetNameArgs, CoreObjectSetPropertyArgs,
};
use wwise_waapi::api::{ObjectReturnField, OnNameConflict};
use wwise_waapi::{ObjectRef, ObjectType, Property};

const ACTOR_MIXER_WU: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";

async fn recv<T>(rx: &mut tokio::sync::mpsc::UnboundedReceiver<T>) -> T {
    timeout(Duration::from_secs(10), rx.recv())
        .await
        .expect("timed out waiting for topic payload")
        .expect("topic channel closed")
}

#[tokio::test]
async fn typed_topic_payload_sequence() {
    let _guard = sequence_lock().await;
    let client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    let id_name = vec![ObjectReturnField::Id, ObjectReturnField::Name];

    // Unique names so a leftover object from a prior aborted run cannot collide
    // with the rename target (setName has no OnNameConflict option).
    let suffix = std::process::id();
    let mixer_name = format!("WaapiSeqTopicMixer_{suffix}");
    let mixer_renamed = format!("WaapiSeqTopicMixerRenamed_{suffix}");

    // 1. Subscribe before acting.
    let (_, mut rx_created) = client
        .wwise()
        .core()
        .object()
        .subscribe_created(Some(
            CoreObjectCreatedOptions::new().return_(id_name.clone()),
        ))
        .await
        .expect("subscribe created");
    let (_, mut rx_name) = client
        .wwise()
        .core()
        .object()
        .subscribe_name_changed(Some(
            CoreObjectNameChangedOptions::new().return_(id_name.clone()),
        ))
        .await
        .expect("subscribe name changed");
    let (_, mut rx_pre) = client
        .wwise()
        .core()
        .object()
        .subscribe_pre_deleted(Some(
            CoreObjectPreDeletedOptions::new().return_(id_name.clone()),
        ))
        .await
        .expect("subscribe pre deleted");

    // 2. Create triggers `created`.
    let created = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(ACTOR_MIXER_WU),
                ObjectType::ActorMixer,
                mixer_name.clone(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create mixer");
    let mixer_id = created.id.clone().expect("mixer id");

    let created_payload: CoreObjectCreatedPayload = recv(&mut rx_created).await;
    assert_eq!(
        created_payload.object.id.as_deref(),
        Some(mixer_id.as_str())
    );

    // Wwise also publishes an initial nameChanged ("" -> <name>) on creation;
    // drain it so the next event we read is the actual rename.
    let initial_name: CoreObjectNameChangedPayload = recv(&mut rx_name).await;
    assert_eq!(initial_name.object.id.as_deref(), Some(mixer_id.as_str()));

    // 3. Rename triggers `nameChanged`.
    client
        .wwise()
        .core()
        .object()
        .set_name(CoreObjectSetNameArgs::new(
            ObjectRef::from(mixer_id.as_str()),
            mixer_renamed.clone(),
        ))
        .await
        .expect("set name");

    let name_payload: CoreObjectNameChangedPayload = recv(&mut rx_name).await;
    assert_eq!(name_payload.object.id.as_deref(), Some(mixer_id.as_str()));
    assert_eq!(name_payload.new_name, mixer_renamed);
    assert_eq!(name_payload.old_name, mixer_name);

    // 4. Property change triggers `propertyChanged`.
    let (_, mut rx_prop) = client
        .wwise()
        .core()
        .object()
        .subscribe_property_changed(Some(
            CoreObjectPropertyChangedOptions::new(
                ObjectRef::from(mixer_id.as_str()),
                Property::Volume,
            )
            .return_(id_name.clone()),
        ))
        .await
        .expect("subscribe property changed");

    client
        .wwise()
        .core()
        .object()
        .set_property(CoreObjectSetPropertyArgs::new(
            &mixer_id,
            Property::Volume,
            -3.0,
        ))
        .await
        .expect("set property");

    let prop_payload: CoreObjectPropertyChangedPayload = recv(&mut rx_prop).await;
    assert_eq!(prop_payload.object.id.as_deref(), Some(mixer_id.as_str()));
    assert_eq!(prop_payload.property, Property::Volume);
    assert_eq!(prop_payload.new.as_f64(), Some(-3.0));

    // 5. Delete triggers `preDeleted`.
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            mixer_id.as_str(),
        )))
        .await
        .expect("delete mixer");

    let pre_payload: CoreObjectPreDeletedPayload = recv(&mut rx_pre).await;
    assert_eq!(pre_payload.object.id.as_deref(), Some(mixer_id.as_str()));

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
