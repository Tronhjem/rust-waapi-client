use waapi_client::{WaapiClient, WaapiResult};

pub mod audio_import;
pub mod authoring_workflow;
pub mod blend_container;
pub mod info_ui_and_soundengine;
pub mod object_introspection;
pub mod object_lifecycle;
pub mod queries_and_transforms;
pub mod references_and_conflicts;
pub mod singles;
pub mod soundbank_and_audio;
pub mod soundengine_extended;
pub mod subscriptions;
pub mod topics;

pub async fn connect_client() -> WaapiResult<WaapiClient> {
    let _ = env_logger::builder().is_test(true).try_init();
    WaapiClient::new(None).await
}

/// Discovers the master audio bus (the primary `Bus` under the Master-Mixer
/// hierarchy's default work unit). Wwise 2024 names it "Master Audio Bus";
/// Wwise 2025 renames it to "Main Audio Bus". Returns `(id, name)`.
pub async fn master_audio_bus(client: &mut WaapiClient) -> (String, String) {
    use waapi_client::api::ak::wwise::core::object::{CoreObjectGetArgs, CoreObjectGetOptions};
    use waapi_client::api::{
        ObjectQuery, ObjectReturnField, ObjectType, Transform, TransformSelect,
    };

    let resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(
                r"\Master-Mixer Hierarchy\Default Work Unit",
            ))
            .transform(vec![Transform::Select {
                select: vec![TransformSelect::Children],
            }]),
            CoreObjectGetOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await
        .expect("Failed to discover master audio bus");

    let bus = resp
        .objects
        .into_iter()
        .find(|o| o.object_type == Some(ObjectType::Bus))
        .expect("No Bus found under the Master-Mixer hierarchy");

    let id = bus.id.expect("Bus missing id");
    let name = bus.name.unwrap_or_else(|| "Master Audio Bus".to_string());
    (id, name)
}

/// Global lock serializing live Wwise sequence tests.
///
/// Wwise keeps a single global undo stack and a single transport list, so
/// sequence tests must not run concurrently (`undo.undo()` would otherwise
/// revert operations belonging to another test).
pub async fn sequence_lock() -> tokio::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await
}
