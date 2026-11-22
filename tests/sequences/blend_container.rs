use super::{connect_client, sequence_lock};
use waapi_client::api::OnNameConflict;
use waapi_client::api::ak::wwise::core::blendContainer::{
    CoreBlendContainerAddAssignmentArgs, CoreBlendContainerAddTrackArgs,
    CoreBlendContainerGetAssignmentsArgs, CoreBlendContainerRemoveAssignmentArgs,
};
use waapi_client::api::ak::wwise::core::object::{CoreObjectCreateArgs, CoreObjectDeleteArgs};
use waapi_client::{ObjectRef, ObjectType};

const ACTOR_MIXER_WU: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";

#[tokio::test]
async fn blend_container_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    // 1. Create a Blend Container and a Sound child.
    let blend = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(ACTOR_MIXER_WU),
                ObjectType::BlendContainer,
                "WaapiSeqBlendContainer".to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create blend container");
    let blend_id = blend.id.clone().expect("blend container id");

    let sound = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(blend_id.as_str()),
                ObjectType::Sound,
                "WaapiSeqBlendChild".to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create blend child");
    let sound_id = sound.id.clone().expect("sound id");

    // 2. addTrack creates a Blend Track under the container.
    let track = client
        .wwise()
        .core()
        .blend_container()
        .add_track(CoreBlendContainerAddTrackArgs::new(
            ObjectRef::from(blend_id.as_str()),
            "WaapiSeqBlendTrack".to_string(),
        ))
        .await
        .expect("add blend track");
    let track_id = track.id.clone().expect("blend track id");
    assert_eq!(track.name.as_deref(), Some("WaapiSeqBlendTrack"));
    assert_eq!(track.object_type, Some(ObjectType::BlendTrack));

    // 3. addAssignment + getAssignments + removeAssignment.
    client
        .wwise()
        .core()
        .blend_container()
        .add_assignment(CoreBlendContainerAddAssignmentArgs::new(
            track_id.clone(),
            ObjectRef::from(sound_id.as_str()),
        ))
        .await
        .expect("add blend assignment");

    let assignments = client
        .wwise()
        .core()
        .blend_container()
        .get_assignments(CoreBlendContainerGetAssignmentsArgs::new(track_id.clone()))
        .await
        .expect("get blend assignments");
    assert_eq!(assignments.assignments.len(), 1);
    assert_eq!(assignments.assignments[0].child, sound_id);

    client
        .wwise()
        .core()
        .blend_container()
        .remove_assignment(CoreBlendContainerRemoveAssignmentArgs::new(
            track_id.clone(),
            ObjectRef::from(sound_id.as_str()),
        ))
        .await
        .expect("remove blend assignment");

    let assignments = client
        .wwise()
        .core()
        .blend_container()
        .get_assignments(CoreBlendContainerGetAssignmentsArgs::new(track_id.clone()))
        .await
        .expect("get blend assignments after removal");
    assert!(assignments.assignments.is_empty());

    // Cleanup.
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            blend_id.as_str(),
        )))
        .await
        .expect("delete blend container");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
