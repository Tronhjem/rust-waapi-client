use std::fmt;

use serde_json::Value;
use tokio::sync::mpsc;

use crate::waapi_function_api::WaapiFunction;
use crate::waapi_topics_api::WaapiTopic;
pub use crate::wamp::Event;
use crate::wamp::{WampClient, WampError, raw_kwargs_str};

/// Result alias for WAAPI client operations.
pub type WaapiResult<T> = Result<T, WaapiError>;

/// Structured details of a WAAPI RPC or subscription error returned over WAMP.
#[derive(Debug, Clone, PartialEq)]
pub struct WaapiCallError {
    /// WAAPI error URI (e.g. `"ak.wwise.invalid_arguments"`, `"ak.wwise.locked"`).
    pub uri: String,
    /// Human-readable error message extracted from `kwargs["message"]` (or the first string argument), if present.
    pub message: Option<String>,
    /// Positional WAMP error arguments (`Arguments`).
    pub args: Vec<Value>,
    /// Keyword WAMP error arguments / details (`ArgumentsKw`).
    pub kwargs: Value,
}

impl WaapiCallError {
    /// Construct a [`WaapiCallError`] from the raw WAMP `ERROR` fields.
    pub fn new(uri: impl Into<String>, args: Vec<Value>, kwargs: Value) -> Self {
        let message = kwargs
            .get("message")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned)
            .or_else(|| args.first().and_then(Value::as_str).map(ToOwned::to_owned));
        Self {
            uri: uri.into(),
            message,
            args,
            kwargs,
        }
    }

    /// Returns the WAAPI error URI (e.g. `"ak.wwise.invalid_arguments"`).
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the human-readable WAAPI error message, if present.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the structured error details (`kwargs.details` if present, otherwise `&self.kwargs`).
    pub fn details(&self) -> &Value {
        self.kwargs.get("details").unwrap_or(&self.kwargs)
    }
}

impl fmt::Display for WaapiCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(msg) = &self.message {
            write!(f, "{}: {msg}", self.uri)?;
            if let Some(details) = self.kwargs.get("details")
                && !details.is_null()
            {
                write!(f, " ({details})")?;
            }
            Ok(())
        } else if !self.args.is_empty() || !self.kwargs.is_null() {
            write!(
                f,
                "{} args={:?} kwargs={}",
                self.uri, self.args, self.kwargs
            )
        } else {
            write!(f, "{}", self.uri)
        }
    }
}

impl std::error::Error for WaapiCallError {}

/// Errors returned by [`WaapiClient`] operations.
#[derive(Debug)]
pub enum WaapiError {
    /// Failed to establish the WebSocket/WAMP connection to WAAPI.
    Connection(String),
    /// Operation attempted on a [`WaapiClient`] that has already been shut down.
    ClientShutdown,
    /// Failed to serialize request arguments/options or deserialize a response.
    Serialization(serde_json::Error),
    /// WAAPI returned a structured WAMP call/subscribe/unsubscribe error.
    Call(WaapiCallError),
    /// Low-level WAMP transport error.
    Wamp(WampError),
}

impl WaapiError {
    /// If this is a [`WaapiError::Call`], returns the underlying [`WaapiCallError`].
    pub fn as_call_error(&self) -> Option<&WaapiCallError> {
        match self {
            WaapiError::Call(err) => Some(err),
            _ => None,
        }
    }

    /// If this is a [`WaapiError::Call`], returns the WAAPI error URI (e.g. `"ak.wwise.invalid_arguments"`).
    pub fn error_uri(&self) -> Option<&str> {
        self.as_call_error().map(WaapiCallError::uri)
    }

    /// If this is a [`WaapiError::Call`], returns the human-readable WAAPI error message.
    pub fn waapi_message(&self) -> Option<&str> {
        self.as_call_error().and_then(WaapiCallError::message)
    }
}

impl fmt::Display for WaapiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WaapiError::Connection(msg) => write!(f, "connection error: {msg}"),
            WaapiError::ClientShutdown => write!(f, "connection error: client already shut down"),
            WaapiError::Serialization(e) => write!(f, "serialization error: {e}"),
            WaapiError::Call(err) => write!(f, "call error: {err}"),
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
                WaapiError::Call(WaapiCallError::new(uri, args, kwargs))
            }
            other => WaapiError::Wamp(other),
        }
    }
}

/// Client for the Wwise Authoring API (WAAPI) over WAMP/WebSocket.
pub struct WaapiClient {
    wamp: Option<WampClient>,
    called_shutdown: bool,
}

impl WaapiClient {
    /// Connect to WAAPI on the default local endpoint (`ws://127.0.0.1:8080/waapi`).
    pub async fn connect() -> WaapiResult<Self> {
        Self::new(None).await
    }

    /// Connect to WAAPI at a custom WebSocket URL (`ws://host:port/waapi`) or `host:port`.
    pub async fn connect_to(url: &str) -> WaapiResult<Self> {
        Self::new(Some(url)).await
    }

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
        &self,
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
        &self,
        uri: &str,
        args: Option<Value>,
        options: Option<Value>,
    ) -> WaapiResult<Value> {
        let result = self
            .wamp
            .as_ref()
            .ok_or(WaapiError::ClientShutdown)?
            .call(uri, options, None, args)
            .await
            .map_err(WaapiError::from)?;
        Ok(result)
    }

    /// Type-safe WAAPI call.
    ///
    /// Serializes typed args/options directly into the outgoing WAMP frame and
    /// deserializes the response directly from the incoming JSON payload without
    /// intermediate `serde_json::Value` allocations.
    pub async fn call_typed<Args, Options, Response>(
        &self,
        uri: WaapiFunction,
        args: Option<Args>,
        options: Option<Options>,
    ) -> WaapiResult<Response>
    where
        Args: serde::Serialize,
        Options: serde::Serialize,
        Response: serde::de::DeserializeOwned,
    {
        let raw = self
            .wamp
            .as_ref()
            .ok_or(WaapiError::ClientShutdown)?
            .call_typed(uri.as_str(), options.as_ref(), args.as_ref())
            .await
            .map_err(WaapiError::from)?;
        Ok(serde_json::from_str::<Response>(raw_kwargs_str(
            raw.as_deref(),
        ))?)
    }

    /// Type-safe WAAPI call that discards the response.
    ///
    /// Use for endpoints that return no useful data.
    pub async fn call_typed_void<Args, Options>(
        &self,
        uri: WaapiFunction,
        args: Option<Args>,
        options: Option<Options>,
    ) -> WaapiResult<()>
    where
        Args: serde::Serialize,
        Options: serde::Serialize,
    {
        self.wamp
            .as_ref()
            .ok_or(WaapiError::ClientShutdown)?
            .call_typed(uri.as_str(), options.as_ref(), args.as_ref())
            .await
            .map_err(WaapiError::from)?;
        Ok(())
    }

    /// Subscribe to a known [`WaapiTopic`] with optional subscription options (e.g. `{"return": ["id", "name"]}`).
    ///
    /// Returns an unbounded receiver of [`Event`] values. Dropping the
    /// receiver does not unsubscribe on the server; call [`WaapiClient::unsubscribe`]
    /// with the returned subscription id when done.
    pub async fn subscribe(
        &self,
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
        &self,
        topic: &str,
        options: Option<Value>,
    ) -> WaapiResult<(u64, mpsc::UnboundedReceiver<Event>)> {
        let (tx, rx) = mpsc::unbounded_channel();
        let id = self
            .wamp
            .as_ref()
            .ok_or(WaapiError::ClientShutdown)?
            .subscribe(topic, options, tx)
            .await
            .map_err(WaapiError::from)?;
        Ok((id, rx))
    }

    /// Type-safe WAAPI subscription.
    ///
    /// Serializes typed options directly into the outgoing WAMP frame before subscribing.
    pub async fn subscribe_typed<Options>(
        &self,
        topic: WaapiTopic,
        options: Option<Options>,
    ) -> WaapiResult<(u64, mpsc::UnboundedReceiver<Event>)>
    where
        Options: serde::Serialize,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        let id = self
            .wamp
            .as_ref()
            .ok_or(WaapiError::ClientShutdown)?
            .subscribe_with_options(topic.as_str(), options.as_ref(), tx)
            .await
            .map_err(WaapiError::from)?;
        Ok((id, rx))
    }

    /// Type-safe WAAPI subscription that deserializes each event into a typed
    /// payload.
    ///
    /// The WAAPI topic payload (the single object in `event.kwargs`) is
    /// deserialized directly from the raw JSON payload into `Payload` without an
    /// intermediate `serde_json::Value` tree. Events whose payload fails to
    /// deserialize are logged and skipped rather than propagated.
    pub async fn subscribe_payload<Options, Payload>(
        &self,
        topic: WaapiTopic,
        options: Option<Options>,
    ) -> WaapiResult<(u64, mpsc::UnboundedReceiver<Payload>)>
    where
        Options: serde::Serialize,
        Payload: serde::de::DeserializeOwned + Send + 'static,
    {
        let (raw_tx, mut raw_rx) = mpsc::unbounded_channel();
        let id = self
            .wamp
            .as_ref()
            .ok_or(WaapiError::ClientShutdown)?
            .subscribe_raw_payload(topic.as_str(), options.as_ref(), raw_tx)
            .await
            .map_err(WaapiError::from)?;
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(raw_kwargs) = raw_rx.recv().await {
                match serde_json::from_str::<Payload>(raw_kwargs_str(raw_kwargs.as_deref())) {
                    Ok(payload) => {
                        if tx.send(payload).is_err() {
                            break;
                        }
                    }
                    Err(err) => {
                        log::warn!("failed to deserialize WAAPI topic payload: {err}");
                    }
                }
            }
        });
        Ok((id, rx))
    }

    /// Unsubscribe from a previously subscribed topic.
    pub async fn unsubscribe(&self, subscription_id: u64) -> WaapiResult<()> {
        self.wamp
            .as_ref()
            .ok_or(WaapiError::ClientShutdown)?
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
