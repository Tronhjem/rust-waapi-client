pub mod api;
mod types;
pub mod waapi_client;

pub use types::{ReturnType, WaapiArgs, WaapiOptions, WaapiValue};
pub use waapi_client::{WaapiClient, WaapiError, WaapiResponse, WaapiResult};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
