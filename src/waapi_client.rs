use std::fmt;

use serde_json::Value;
use tokio::sync::mpsc;

use crate::waapi_function_api::WaapiFunction;
use crate::waapi_topics_api::WaapiTopic;
pub use crate::wamp::Event;
use crate::wamp::{WampClient, WampError};

pub type WaapiResult<T> = Result<T, WaapiError>;

#[derive(Debug)]
pub enum WaapiError {
    Connection(String),
    Serialization(serde_json::Error),
    Call(String),
    Wamp(WampError),
}

impl fmt::Display for WaapiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WaapiError::Connection(msg) => write!(f, "connection error: {msg}"),
            WaapiError::Serialization(e) => write!(f, "serialization error: {e}"),
            WaapiError::Call(msg) => write!(f, "call error: {msg}"),
            WaapiError::Wamp(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for WaapiError {}

impl From<serde_json::Error> for WaapiError {
    fn from(err: serde_json::Error) -> Self {
        WaapiError::Serialization(err)
    }
}

impl From<WampError> for WaapiError {
    fn from(err: WampError) -> Self {
        match err {
            WampError::CallError { uri, args, kwargs } => {
                let detail = format_args(&uri, &args, &kwargs);
                WaapiError::Call(detail)
            }
            other => WaapiError::Wamp(other),
        }
    }
}

fn format_args(uri: &str, args: &[Value], kwargs: &Value) -> String {
    if !args.is_empty() || !kwargs.is_null() {
        format!("{uri} args={args:?} kwargs={kwargs}")
    } else {
        uri.to_string()
    }
}

/// Client for the Wwise Authoring API (WAAPI) over WAMP/WebSocket.
pub struct WaapiClient {
    wamp: Option<WampClient>,
    called_shutdown: bool,
}

impl WaapiClient {
    /// Connect to WAAPI.
    ///
    /// `url` may be a full WebSocket URI (`ws://host:port/waapi`) or just
    /// `host:port`, in which case `ws://host:port/waapi` is used.
    /// Defaults to `ws://127.0.0.1:8080/waapi`.
    pub async fn new(url: Option<&str>) -> WaapiResult<Self> {
        let url = match url {
            Some(u) if u.starts_with("ws://") || u.starts_with("wss://") => u.to_string(),
            Some(u) => format!("ws://{u}/waapi"),
            None => "ws://127.0.0.1:8080/waapi".to_string(),
        };

        let wamp = WampClient::connect(&url)
            .await
            .map_err(|e| WaapiError::Connection(e.to_string()))?;

        Ok(WaapiClient {
            wamp: Some(wamp),
            called_shutdown: false,
        })
    }

    /// WAAPI call using a known [`WaapiFunction`] and raw JSON arguments/options.
    ///
    /// `args` is passed as the WAMP `CALL` keyword arguments (`ArgumentsKw`), and
    /// `options` is passed as the WAMP `CALL` `Options` dictionary (e.g. `{"return": ["id", "name"]}`).
    pub async fn call(
        &mut self,
        uri: WaapiFunction,
        args: Option<Value>,
        options: Option<Value>,
    ) -> WaapiResult<Value> {
        self.call_raw(uri.as_str(), args, options).await
    }

    /// Low-level WAAPI call taking a raw URI string.
    ///
    /// Prefer [`WaapiClient::call`] or the typed endpoint methods when calling
    /// known WAAPI endpoints.
    pub async fn call_raw(
        &mut self,
        uri: &str,
        args: Option<Value>,
        options: Option<Value>,
    ) -> WaapiResult<Value> {
        let result = self
            .wamp
            .as_ref()
            .ok_or_else(|| WaapiError::Connection("client shutdown".to_string()))?
            .call(uri, options, None, args)
            .await
            .map_err(WaapiError::from)?;
        Ok(result)
    }

    /// Type-safe WAAPI call.
    ///
    /// Serializes typed args/options and deserializes the response.
    pub async fn call_typed<Args, Options, Response>(
        &mut self,
        uri: WaapiFunction,
        args: Option<Args>,
        options: Option<Options>,
    ) -> WaapiResult<Response>
    where
        Args: serde::Serialize,
        Options: serde::Serialize,
        Response: serde::de::DeserializeOwned,
    {
        let args = args.map(serde_json::to_value).transpose()?;
        let options = options.map(serde_json::to_value).transpose()?;
        let value = self.call(uri, args, options).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Type-safe WAAPI call that discards the response.
    ///
    /// Use for endpoints that return no useful data.
    pub async fn call_typed_void<Args, Options>(
        &mut self,
        uri: WaapiFunction,
        args: Option<Args>,
        options: Option<Options>,
    ) -> WaapiResult<()>
    where
        Args: serde::Serialize,
        Options: serde::Serialize,
    {
        let args = args.map(serde_json::to_value).transpose()?;
        let options = options.map(serde_json::to_value).transpose()?;
        self.call(uri, args, options).await?;
        Ok(())
    }

    /// Subscribe to a known [`WaapiTopic`] with optional subscription options (e.g. `{"return": ["id", "name"]}`).
    ///
    /// Returns an unbounded receiver of [`Event`] values. Dropping the
    /// receiver does not unsubscribe on the server; call [`WaapiClient::unsubscribe`]
    /// with the returned subscription id when done.
    pub async fn subscribe(
        &mut self,
        topic: WaapiTopic,
        options: Option<Value>,
    ) -> WaapiResult<(u64, mpsc::UnboundedReceiver<Event>)> {
        self.subscribe_raw(topic.as_str(), options).await
    }

    /// Low-level WAAPI topic subscription taking a raw topic string.
    ///
    /// Prefer [`WaapiClient::subscribe`] or [`WaapiClient::subscribe_typed`] when
    /// subscribing to known WAAPI topics.
    pub async fn subscribe_raw(
        &mut self,
        topic: &str,
        options: Option<Value>,
    ) -> WaapiResult<(u64, mpsc::UnboundedReceiver<Event>)> {
        let (tx, rx) = mpsc::unbounded_channel();
        let id = self
            .wamp
            .as_ref()
            .ok_or_else(|| WaapiError::Connection("client shutdown".to_string()))?
            .subscribe(topic, options, tx)
            .await
            .map_err(WaapiError::from)?;
        Ok((id, rx))
    }

    /// Type-safe WAAPI subscription.
    ///
    /// Serializes typed options before subscribing.
    pub async fn subscribe_typed<Options>(
        &mut self,
        topic: WaapiTopic,
        options: Option<Options>,
    ) -> WaapiResult<(u64, mpsc::UnboundedReceiver<Event>)>
    where
        Options: serde::Serialize,
    {
        let options = options.map(serde_json::to_value).transpose()?;
        self.subscribe(topic, options).await
    }

    /// Unsubscribe from a previously subscribed topic.
    pub async fn unsubscribe(&mut self, subscription_id: u64) -> WaapiResult<()> {
        self.wamp
            .as_ref()
            .ok_or_else(|| WaapiError::Connection("client shutdown".to_string()))?
            .unsubscribe(subscription_id)
            .await
            .map_err(WaapiError::from)
    }

    /// Gracefully close the WAMP session.
    pub async fn shutdown(mut self) -> WaapiResult<()> {
        self.called_shutdown = true;
        match self.wamp.take() {
            Some(wamp) => wamp.shutdown().await.map_err(WaapiError::from),
            None => Ok(()),
        }
    }
}

impl Drop for WaapiClient {
    fn drop(&mut self) {
        if !self.called_shutdown {
            log::warn!("WaapiClient dropped without calling shutdown()");
        }
    }
}
