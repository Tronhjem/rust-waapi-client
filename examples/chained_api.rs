//! Demonstrates the chained namespace API mirroring WAAPI URI paths.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example chained_api

use log::info;
use wwise_waapi::api::ak::wwise::core::object::{CoreObjectGetArgs, CoreObjectGetOptions};
use wwise_waapi::api::ak::wwise::ui::UiGetSelectedObjectsOptions;
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = WaapiClient::connect().await?;

    let info = client.ak().wwise().core().get_info().await?;
    info!(
        "Connected to {} ({})",
        info.display_name, info.version.display_name
    );

    let objects = client
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::path(r"\Containers")),
            CoreObjectGetOptions::new().return_fields([
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await?;
    info!("object.get returned {} objects", objects.objects.len());

    let selected = client
        .ui()
        .get_selected_objects(
            UiGetSelectedObjectsOptions::new()
                .return_fields([ObjectReturnField::Name, ObjectReturnField::Type]),
        )
        .await?;
    info!("{} objects selected in the UI", selected.objects.len());

    client.shutdown().await?;
    Ok(())
}
