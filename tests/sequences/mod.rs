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
