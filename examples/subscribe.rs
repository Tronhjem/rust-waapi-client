// use std::collections::HashMap;
//
// use log::{debug, error, info};
// use waapi_client::{ReturnType, WaapiArgs, WaapiClient, WaapiOptions, WaapiValue};
// use waapi_client::{waapi_function_api, waapi_topics_api};
//
// #[tokio::main]
// async fn main() {
//     WaapiClient::initialize_logger(Some("info"));
//
//     let test_address: &str = "127.0.0.1:8080";
//     info!("Connecting to WAAPI at {}", test_address);
//
//     let waapi_client = WaapiClient::new(Some(test_address));
//     if let Ok(mut client) = waapi_client {
//         info!("Successfully connected to WAAPI");
//         client
//             .subscribe(
//                 waapi_topics_api::ak::wwise::core::object::created,
//                 HashMap::new(),
//             )
//             .await
//             .unwrap();
//
//         loop {}
//         client.shutdown().await.unwrap();
//     }
// }
//
fn main() {
    todo!();
}
