use log::info;
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let address = "127.0.0.1:8080";
    info!("Connecting to Wwise WAAPI at {}", address);

    let mut client = WaapiClient::new(Some(address)).await?;
    info!("Connected. Calling ak.wwise.core.getInfo...");

    let info = client.get_info(()).await?;

    info!("Wwise display name: {}", info.display_name);
    info!("Platform: {}", info.platform);
    if let Some(version) = info.version.display_name {
        info!("Version: {}", version);
    }
    if let Some(schema) = info.version.schema {
        info!("Schema version: {}", schema);
    }
    if let Some(install) = info.directories.install {
        info!("Install directory: {}", install);
    }

    client.shutdown().await?;
    info!("Disconnected.");

    Ok(())
}
