//! Subscribe to WAAPI topics using the typed subscription API (requires a running Wwise instance).
//!
//! Run with: cargo run --example subscribe

use log::{error, info};
use tokio::time::{Duration, timeout};
use wwise_waapi::api::ak::wwise::core::object::CreatedOptions;
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let address = "127.0.0.1:8080";
    info!("Connecting to WAAPI at {address}");

    let mut client = WaapiClient::connect_to(address).await?;
    info!("Successfully connected to WAAPI");

    let options = CreatedOptions::new().return_(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Type,
    ]);

    let (subscription_id, mut events) = client.object().subscribe_created(Some(options)).await?;
    info!("Subscribed to ak.wwise.core.object.created with id {subscription_id}");

    // Print the first created-object payload within 60 seconds, then unsubscribe.
    match timeout(Duration::from_secs(60), events.recv()).await {
        Ok(Some(payload)) => info!(
            "Created object: id={:?} name={:?} type={:?}",
            payload.object.id(),
            payload.object.name(),
            payload.object.object_type()
        ),
        Ok(None) => info!("Event channel closed"),
        Err(_) => info!("No event received within 60 seconds"),
    }

    if let Err(e) = client.unsubscribe(subscription_id).await {
        error!("Unsubscribe failed: {e}");
    }

    client.shutdown().await?;
    Ok(())
}
