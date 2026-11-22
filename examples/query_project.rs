//! Query the project itself and print its name, file path, and dirty state.
//!
//! Mirrors the Audiokinetic WAAPI example "Getting the project name, path, and
//! dirty state" (`ak.wwise.core.object.get`).
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example query_project

use wwise_waapi::api::generated::ak::wwise::core::object::{
    CoreObjectGetArgs, CoreObjectGetOptions,
};
use wwise_waapi::api::generated::primitives::ObjectReturnField;
use wwise_waapi::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = WaapiClient::new(None).await?;

    // The WAQL query `"\""` (a string containing a single backslash) selects the
    // project root itself, rather than any object inside it.
    let args = CoreObjectGetArgs::from_waql(r#""\""#);

    let options = CoreObjectGetOptions::new().return_fields([
        ObjectReturnField::Name,
        ObjectReturnField::FilePath,
        ObjectReturnField::WorkunitIsDirty,
    ]);

    let response = client.wwise().core().object().get(args, options).await?;

    for project in &response.objects {
        println!("Project: {:?}", project.name);
        println!("File:    {:?}", project.file_path);
        println!("Dirty:   {:?}", project.workunit_is_dirty);
    }

    client.shutdown().await?;
    Ok(())
}
