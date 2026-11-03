//! Demonstrates the chained namespace API mirroring WAAPI URI paths.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example chained_api

use log::info;
use waapi_client::api::generated::ak::wwise::core::object::{
    WwiseCoreObjectGetArgs, WwiseCoreObjectGetOptions,
};
use waapi_client::api::generated::primitives::{ObjectQuery, ObjectReturnField};
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = WaapiClient::new(Some("127.0.0.1:8080")).await?;

    let info = client.ak().wwise().core().get_info().await?;
    info!(
        "Connected to {} ({})",
        info.display_name, info.version.display_name
    );

    let objects = client
        .wwise()
        .core()
        .object()
        .get(
            WwiseCoreObjectGetArgs::new(ObjectQuery::from(r"\Actor-Mixer Hierarchy")),
            WwiseCoreObjectGetOptions::new().return_fields([
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await?;
    info!("object.get returned {} objects", objects.objects.len());

    let selected = client
        .wwise()
        .ui()
        .get_selected_objects(WwiseCoreObjectGetOptions::new().return_fields(["name", "type"]))
        .await?;
    info!("{} objects selected in the UI", selected.objects.len());

    client.shutdown().await?;
    Ok(())
}
