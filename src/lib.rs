mod types;
pub mod waapi_client;
pub mod waapi_function_api;
pub mod waapi_topics_api;

pub use types::{ReturnType, WaapiArgs, WaapiOptions, WaapiValue};
pub use waapi_client::{WaapiClient, WaapiError, WaapiResponse, WaapiResult};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
