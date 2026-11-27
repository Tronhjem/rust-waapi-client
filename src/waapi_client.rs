use std::fmt;

use serde_json::Value;
use tokio::sync::mpsc;

use crate::waapi_function_api::WaapiFunction;
use crate::waapi_topics_api::WaapiTopic;
pub use crate::wamp::Event;
use crate::wamp::{WampClient, WampError, raw_kwargs_str};

/// Result alias for WAAPI client operations.
pub type WaapiResult<T> = Result<T, WaapiError>;

/// Known WAAPI and WAMP error URIs returned by Wwise Authoring.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WaapiErrorKind {
    /// `"ak.wwise.already_connected"` — Remote profiler is already connected.
    AlreadyConnected,
    /// `"ak.wwise.cancelled"` — The operation was cancelled.
    Cancelled,
    /// `"ak.wwise.connection_failed"` — Remote profiler connection failed.
    ConnectionFailed,
    /// `"ak.wwise.failed"` — Generic WAAPI operation failure.
    Failed,
    /// `"ak.wwise.file_error"` — File I/O error on the Wwise side.
    FileError,
    /// `"ak.wwise.invalid_arguments"` — Invalid arguments passed to a WAAPI procedure.
    InvalidArguments,
    /// `"ak.wwise.invalid_json"` — Invalid JSON payload.
    InvalidJson,
    /// `"ak.wwise.invalid_language"` — Invalid language specified.
    InvalidLanguage,
    /// `"ak.wwise.invalid_object"` — Invalid Wwise object reference.
    InvalidObject,
    /// `"ak.wwise.invalid_options"` — Invalid options passed to a WAAPI procedure or subscription.
    InvalidOptions,
    /// `"ak.wwise.invalid_procedure_uri"` — The procedure URI is unknown.
    InvalidProcedureUri,
    /// `"ak.wwise.invalid_property"` — Invalid property or reference name.
    InvalidProperty,
    /// `"ak.wwise.invalid_reference"` — Invalid object reference assignment.
    InvalidReference,
    /// `"ak.wwise.invalid_results"` — Result validation failed.
    InvalidResults,
    /// `"ak.wwise.invalid_topic_uri"` — The subscription topic URI is unknown.
    InvalidTopicUri,
    /// `"ak.wwise.invalid_type"` — Invalid Wwise object type.
    InvalidType,
    /// `"ak.wwise.locked"` — Wwise currently holds an exclusive lock and cannot execute the call.
    Locked,
    /// `"ak.wwise.no_project_loaded"` — No Wwise project is currently loaded.
    NoProjectLoaded,
    /// `"ak.wwise.not_implemented"` — Endpoint or feature is not implemented.
    NotImplemented,
    /// `"ak.wwise.schema_validation_failed"` — Request arguments or options failed JSON Schema validation.
    SchemaValidationFailed,
    /// `"ak.wwise.time_out"` — The operation timed out inside Wwise.
    TimeOut,
    /// `"ak.wwise.unavailable"` — Procedure or topic is not available in this Wwise instance/mode.
    Unavailable,
    /// `"ak.wwise.unexpected_error"` — Unexpected internal Wwise error.
    UnexpectedError,
    /// `"ak.wwise.unknown_class_id"` — Unknown Wwise object class ID.
    UnknownClassId,
    /// `"ak.wwise.unknown_language"` — Language could not be found in the project.
    UnknownLanguage,
    /// `"ak.wwise.unknown_platform"` — Platform could not be found in the project.
    UnknownPlatform,
    /// `"ak.wwise.wwise_console"` — WwiseConsole restrictions or error.
    WwiseConsole,
    /// `"wamp.error.goodbye_and_out"` — WAMP session closed by peer.
    GoodbyeAndOut,
    /// `"wamp.error.no_such_subscription"` — Could not find the subscription ID to unsubscribe.
    NoSuchSubscription,
    /// `"wamp.error.runtime_error"` — Generic WAMP runtime error.
    RuntimeError,
    /// Any other WAAPI or WAMP error URI not in the known set.
    Other(String),
}

impl WaapiErrorKind {
    /// Parse a WAAPI or WAMP error URI string into a [`WaapiErrorKind`].
    pub fn from_uri(uri: &str) -> Self {
        match uri {
            "ak.wwise.already_connected" => Self::AlreadyConnected,
            "ak.wwise.cancelled" => Self::Cancelled,
            "ak.wwise.connection_failed" => Self::ConnectionFailed,
            "ak.wwise.failed" => Self::Failed,
            "ak.wwise.file_error" => Self::FileError,
            "ak.wwise.invalid_arguments" => Self::InvalidArguments,
            "ak.wwise.invalid_json" => Self::InvalidJson,
            "ak.wwise.invalid_language" => Self::InvalidLanguage,
            "ak.wwise.invalid_object" => Self::InvalidObject,
            "ak.wwise.invalid_options" => Self::InvalidOptions,
            "ak.wwise.invalid_procedure_uri" => Self::InvalidProcedureUri,
            "ak.wwise.invalid_property" => Self::InvalidProperty,
            "ak.wwise.invalid_reference" => Self::InvalidReference,
            "ak.wwise.invalid_results" => Self::InvalidResults,
            "ak.wwise.invalid_topic_uri" => Self::InvalidTopicUri,
            "ak.wwise.invalid_type" => Self::InvalidType,
            "ak.wwise.locked" => Self::Locked,
            "ak.wwise.no_project_loaded" => Self::NoProjectLoaded,
            "ak.wwise.not_implemented" => Self::NotImplemented,
            "ak.wwise.schema_validation_failed" => Self::SchemaValidationFailed,
            "ak.wwise.time_out" => Self::TimeOut,
            "ak.wwise.unavailable" => Self::Unavailable,
            "ak.wwise.unexpected_error" => Self::UnexpectedError,
            "ak.wwise.unknown_class_id" => Self::UnknownClassId,
            "ak.wwise.unknown_language" => Self::UnknownLanguage,
            "ak.wwise.unknown_platform" => Self::UnknownPlatform,
            "ak.wwise.wwise_console" => Self::WwiseConsole,
            "wamp.error.goodbye_and_out" => Self::GoodbyeAndOut,
            "wamp.error.no_such_subscription" => Self::NoSuchSubscription,
            "wamp.error.runtime_error" => Self::RuntimeError,
            other => Self::Other(other.to_string()),
        }
    }

    /// Return the wire error URI string for this [`WaapiErrorKind`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::AlreadyConnected => "ak.wwise.already_connected",
            Self::Cancelled => "ak.wwise.cancelled",
            Self::ConnectionFailed => "ak.wwise.connection_failed",
            Self::Failed => "ak.wwise.failed",
            Self::FileError => "ak.wwise.file_error",
            Self::InvalidArguments => "ak.wwise.invalid_arguments",
            Self::InvalidJson => "ak.wwise.invalid_json",
            Self::InvalidLanguage => "ak.wwise.invalid_language",
            Self::InvalidObject => "ak.wwise.invalid_object",
            Self::InvalidOptions => "ak.wwise.invalid_options",
            Self::InvalidProcedureUri => "ak.wwise.invalid_procedure_uri",
            Self::InvalidProperty => "ak.wwise.invalid_property",
            Self::InvalidReference => "ak.wwise.invalid_reference",
            Self::InvalidResults => "ak.wwise.invalid_results",
            Self::InvalidTopicUri => "ak.wwise.invalid_topic_uri",
            Self::InvalidType => "ak.wwise.invalid_type",
            Self::Locked => "ak.wwise.locked",
            Self::NoProjectLoaded => "ak.wwise.no_project_loaded",
            Self::NotImplemented => "ak.wwise.not_implemented",
            Self::SchemaValidationFailed => "ak.wwise.schema_validation_failed",
            Self::TimeOut => "ak.wwise.time_out",
            Self::Unavailable => "ak.wwise.unavailable",
            Self::UnexpectedError => "ak.wwise.unexpected_error",
            Self::UnknownClassId => "ak.wwise.unknown_class_id",
            Self::UnknownLanguage => "ak.wwise.unknown_language",
            Self::UnknownPlatform => "ak.wwise.unknown_platform",
            Self::WwiseConsole => "ak.wwise.wwise_console",
            Self::GoodbyeAndOut => "wamp.error.goodbye_and_out",
            Self::NoSuchSubscription => "wamp.error.no_such_subscription",
            Self::RuntimeError => "wamp.error.runtime_error",
            Self::Other(uri) => uri.as_str(),
        }
    }
}

impl fmt::Display for WaapiErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for WaapiErrorKind {
    fn from(uri: &str) -> Self {
        Self::from_uri(uri)
    }
}

impl From<String> for WaapiErrorKind {
    fn from(uri: String) -> Self {
        Self::from_uri(&uri)
    }
}

impl serde::Serialize for WaapiErrorKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for WaapiErrorKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct KindVisitor;
        impl serde::de::Visitor<'_> for KindVisitor {
            type Value = WaapiErrorKind;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a WAAPI or WAMP error URI string")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(WaapiErrorKind::from_uri(v))
            }
        }
        deserializer.deserialize_str(KindVisitor)
    }
}

/// Structured error details parsed from `kwargs["details"]` in a WAMP `ERROR` response.
///
/// Wwise populates `kwargs["details"]` with schema validation diagnostics, offending
/// argument names/pointers/values, and nested `reasons` when a call or subscription fails.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaapiErrorDetails {
    /// Procedure or topic URI reported by Wwise in `details.procedureUri`, if present.
    #[serde(rename = "procedureUri", skip_serializing_if = "Option::is_none")]
    pub procedure_uri: Option<String>,
    /// Error type URI reported by Wwise in `details.typeUri` (e.g. `"ak.wwise.schema_validation_failed"`).
    #[serde(rename = "typeUri", skip_serializing_if = "Option::is_none")]
    pub type_uri: Option<String>,
    /// Name of the offending argument (`details.argumentName`), if present.
    #[serde(rename = "argumentName", skip_serializing_if = "Option::is_none")]
    pub argument_name: Option<String>,
    /// JSON Pointer to the offending argument (`details.argumentPointer`), if present.
    #[serde(rename = "argumentPointer", skip_serializing_if = "Option::is_none")]
    pub argument_pointer: Option<String>,
    /// Offending argument value (`details.argumentValue`), if present.
    #[serde(rename = "argumentValue", skip_serializing_if = "Option::is_none")]
    pub argument_value: Option<Value>,
    /// Expected schema type or constraint (`details.schemaExpect`), if present.
    #[serde(rename = "schemaExpect", skip_serializing_if = "Option::is_none")]
    pub schema_expect: Option<String>,
    /// JSON Pointer into the endpoint's JSON Schema (`details.schemaPointer`), if present.
    #[serde(rename = "schemaPointer", skip_serializing_if = "Option::is_none")]
    pub schema_pointer: Option<String>,
    /// Description from the violated JSON Schema node (`details.schemaDescription`), if present.
    #[serde(rename = "schemaDescription", skip_serializing_if = "Option::is_none")]
    pub schema_description: Option<String>,
    /// JSON Schema keyword that failed validation (`details.schemaKeyword`, e.g. `"required"`, `"additionalProperties"`).
    #[serde(rename = "schemaKeyword", skip_serializing_if = "Option::is_none")]
    pub schema_keyword: Option<String>,
    /// Nested validation failure reasons (`details.reasons`), if present.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<WaapiErrorDetails>,
    /// Any additional domain-specific detail fields returned in `kwargs["details"]`.
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// Structured details of a WAAPI RPC or subscription error returned over WAMP.
#[derive(Debug, Clone, PartialEq)]
pub struct WaapiCallError {
    /// Raw WAAPI error URI (e.g. `"ak.wwise.invalid_arguments"`, `"ak.wwise.locked"`).
    pub uri: String,
    /// Typed [`WaapiErrorKind`] parsed from `uri` for pattern matching.
    pub kind: WaapiErrorKind,
    /// The WAAPI procedure or topic URI that was invoked/subscribed when this error occurred, if known.
    pub procedure: Option<String>,
    /// Human-readable error message extracted from `kwargs["message"]` (or the first string argument), if present.
    pub message: Option<String>,
    /// Structured error details parsed from `kwargs["details"]`, if present.
    pub details: Option<WaapiErrorDetails>,
    /// Positional WAMP error arguments (`Arguments`).
    pub args: Vec<Value>,
    /// Keyword WAMP error arguments / details (`ArgumentsKw`).
    pub kwargs: Value,
}

impl WaapiCallError {
    /// Construct a [`WaapiCallError`] from the raw WAMP `ERROR` fields.
    pub fn new(uri: impl Into<String>, args: Vec<Value>, kwargs: Value) -> Self {
        let uri = uri.into();
        let kind = WaapiErrorKind::from_uri(&uri);
        let message = kwargs
            .get("message")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned)
            .or_else(|| args.first().and_then(Value::as_str).map(ToOwned::to_owned));
        let details = kwargs
            .get("details")
            .filter(|v| v.is_object())
            .and_then(|v| serde_json::from_value::<WaapiErrorDetails>(v.clone()).ok());
        let procedure = details.as_ref().and_then(|d| d.procedure_uri.clone());
        Self {
            uri,
            kind,
            procedure,
            message,
            details,
            args,
            kwargs,
        }
    }

    /// Attach the WAAPI procedure or topic URI that triggered this error.
    pub fn with_procedure(mut self, procedure: impl Into<String>) -> Self {
        self.procedure = Some(procedure.into());
        self
    }

    /// Returns the WAAPI error URI (e.g. `"ak.wwise.invalid_arguments"`).
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the typed [`WaapiErrorKind`] parsed from the error URI.
    pub fn kind(&self) -> &WaapiErrorKind {
        &self.kind
    }

    /// Returns the WAAPI procedure or topic URI that failed, if known.
    pub fn procedure(&self) -> Option<&str> {
        self.procedure.as_deref()
    }

    /// Returns the human-readable WAAPI error message, if present.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the typed [`WaapiErrorDetails`] parsed from `kwargs["details"]`, if present.
    pub fn structured_details(&self) -> Option<&WaapiErrorDetails> {
        self.details.as_ref()
    }

    /// Returns the raw JSON error details (`kwargs["details"]` if present, otherwise `&self.kwargs`).
    pub fn details(&self) -> &Value {
        self.kwargs.get("details").unwrap_or(&self.kwargs)
    }
}

impl fmt::Display for WaapiCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(proc_uri) = &self.procedure {
            write!(f, "{} (calling {proc_uri})", self.uri)?;
        } else {
            write!(f, "{}", self.uri)?;
        }
        if let Some(msg) = &self.message {
            write!(f, ": {msg}")?;
            if let Some(details) = self.kwargs.get("details")
                && !details.is_null()
            {
                write!(f, " ({details})")?;
            }
            Ok(())
        } else if !self.args.is_empty() || !self.kwargs.is_null() {
            write!(f, " args={:?} kwargs={}", self.args, self.kwargs)
        } else {
            Ok(())
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
    Call(Box<WaapiCallError>),
    /// Low-level WAMP transport error.
    Wamp(WampError),
}

impl WaapiError {
    /// Attach the WAAPI procedure or topic URI if this error is a [`WaapiError::Call`].
    pub(crate) fn with_procedure(mut self, procedure: &str) -> Self {
        if let WaapiError::Call(err) = &mut self
            && err.procedure.is_none()
        {
            err.procedure = Some(procedure.to_string());
        }
        self
    }

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

    /// If this is a [`WaapiError::Call`], returns the typed [`WaapiErrorKind`].
    pub fn error_kind(&self) -> Option<&WaapiErrorKind> {
        self.as_call_error().map(WaapiCallError::kind)
    }

    /// If this is a [`WaapiError::Call`], returns the procedure or topic URI that failed.
    pub fn procedure(&self) -> Option<&str> {
        self.as_call_error().and_then(WaapiCallError::procedure)
    }

    /// If this is a [`WaapiError::Call`], returns the human-readable WAAPI error message.
    pub fn waapi_message(&self) -> Option<&str> {
        self.as_call_error().and_then(WaapiCallError::message)
    }

    /// If this is a [`WaapiError::Call`], returns the structured [`WaapiErrorDetails`] parsed from `kwargs["details"]`.
    pub fn error_details(&self) -> Option<&WaapiErrorDetails> {
        self.as_call_error()
            .and_then(WaapiCallError::structured_details)
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

impl std::error::Error for WaapiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WaapiError::Serialization(e) => Some(e),
            WaapiError::Call(e) => Some(e),
            WaapiError::Wamp(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for WaapiError {
    fn from(err: serde_json::Error) -> Self {
        WaapiError::Serialization(err)
    }
}

impl From<WaapiCallError> for WaapiError {
    fn from(err: WaapiCallError) -> Self {
        WaapiError::Call(Box::new(err))
    }
}

impl From<WampError> for WaapiError {
    fn from(err: WampError) -> Self {
        match err {
            WampError::CallError { uri, args, kwargs } => {
                WaapiError::Call(Box::new(WaapiCallError::new(uri, args, kwargs)))
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
            .map_err(|e| WaapiError::from(e).with_procedure(uri))?;
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
            .map_err(|e| WaapiError::from(e).with_procedure(uri.as_str()))?;
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
            .map_err(|e| WaapiError::from(e).with_procedure(uri.as_str()))?;
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
            .map_err(|e| WaapiError::from(e).with_procedure(topic))?;
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
            .map_err(|e| WaapiError::from(e).with_procedure(topic.as_str()))?;
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
            .map_err(|e| WaapiError::from(e).with_procedure(topic.as_str()))?;
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
