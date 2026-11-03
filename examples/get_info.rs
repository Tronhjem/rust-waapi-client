//! Minimal typed-API quickstart: connect and call `ak.wwise.core.getInfo`.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example get_info

use log::info;
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = WaapiClient::new(Some("127.0.0.1:8080")).await?;
    info!("Connected. Calling ak.wwise.core.getInfo...");

    let info = client.get_info().await?;

    info!("Wwise display name: {}", info.display_name);
    info!("Platform: {}", info.platform);
    info!("Version: {}", info.version.display_name);
    info!("Schema version: {}", info.version.schema);
    info!("Install directory: {}", info.directories.install);

    client.shutdown().await?;
    info!("Disconnected.");

    Ok(())
}
