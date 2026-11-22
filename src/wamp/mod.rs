pub mod error;
pub mod message;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use futures_util::{SinkExt, StreamExt};
use serde_json::value::RawValue;
use serde_json::{Map, Value};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;

pub use error::{WampError, WampResult};
pub use message::{
    IncomingMessage, Message, RequestId, SubscriptionId, serialize_call, serialize_subscribe,
    serialize_unsubscribe,
};

/// Extract the raw JSON object string from an optional [`RawValue`], defaulting
/// to `"{}"` when omitted or `"null"`.
pub fn raw_kwargs_str(raw: Option<&RawValue>) -> &str {
    match raw.map(|r| r.get().trim()) {
        Some(s) if !s.is_empty() && s != "null" => s,
        _ => "{}",
    }
}

/// A raw WAMP event received from a subscribed WAAPI topic.
#[derive(Debug, Clone)]
pub struct Event {
    /// Positional WAMP event arguments.
    pub args: Vec<Value>,
    /// Keyword WAMP event payload (JSON object delivered by WAAPI topics).
    pub kwargs: Value,
}

enum SubscriptionSink {
    Event(mpsc::UnboundedSender<Event>),
    RawKwargs(mpsc::UnboundedSender<Option<Box<RawValue>>>),
}

enum Command {
    Call {
        request_id: RequestId,
        frame: String,
        tx: oneshot::Sender<WampResult<Option<Box<RawValue>>>>,
    },
    Subscribe {
        request_id: RequestId,
        frame: String,
        sink: SubscriptionSink,
        tx: oneshot::Sender<WampResult<SubscriptionId>>,
    },
    Unsubscribe {
        request_id: RequestId,
        subscription_id: SubscriptionId,
        frame: String,
        tx: oneshot::Sender<WampResult<()>>,
    },
    Shutdown,
}

#[derive(Debug)]
pub struct WampClient {
    request_id: AtomicU64,
    cmd_tx: mpsc::UnboundedSender<Command>,
    handle: Option<JoinHandle<WampResult<()>>>,
}

impl WampClient {
    pub async fn connect(url: &str) -> WampResult<Self> {
        let mut request = url.into_client_request()?;
        request.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static("wamp.2.json"),
        );

        // Set disable_nagle = true (TCP_NODELAY) to avoid 40ms Nagle + delayed-ACK latency spikes.
        let (ws_stream, _) = connect_async_with_config(request, None, true).await?;
        let (mut write_half, mut read_half) = ws_stream.split();

        // Perform the WAMP HELLO / WELCOME handshake before spawning the IO loop.
        send_msg(&mut write_half, &Message::hello("realm1")).await?;

        loop {
            match read_half.next().await {
                Some(Ok(WsMessage::Text(text))) => {
                    let msg = serde_json::from_str::<Message>(text.as_str())
                        .map_err(WampError::Deserialize)?;
                    match msg {
                        Message::Welcome(_session_id, _) => break,
                        Message::Abort(_, reason) => return Err(WampError::WebSocket(reason)),
                        other => {
                            return Err(WampError::UnexpectedMessage(format!("{other:?}")));
                        }
                    }
                }
                Some(Ok(WsMessage::Close(_))) | None => {
                    return Err(WampError::ConnectionClosed);
                }
                Some(Err(e)) => {
                    return Err(WampError::WebSocket(e.to_string()));
                }
                Some(_) => {}
            }
        }

        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<Command>();

        let handle = tokio::spawn(async move {
            let mut pending_calls: HashMap<
                RequestId,
                oneshot::Sender<WampResult<Option<Box<RawValue>>>>,
            > = HashMap::new();
            let mut pending_subscribes: HashMap<
                RequestId,
                (
                    SubscriptionSink,
                    oneshot::Sender<WampResult<SubscriptionId>>,
                ),
            > = HashMap::new();
            let mut pending_unsubscribes: HashMap<
                RequestId,
                (SubscriptionId, oneshot::Sender<WampResult<()>>),
            > = HashMap::new();
            let mut subscriptions: HashMap<SubscriptionId, SubscriptionSink> = HashMap::new();

            loop {
                tokio::select! {
                    incoming = read_half.next() => {
                        match incoming {
                            Some(Ok(WsMessage::Text(text))) => {
                                let msg = serde_json::from_str::<IncomingMessage>(text.as_str())
                                    .map_err(WampError::Deserialize)?;

                                match msg {
                                    IncomingMessage::Result(request_id, kwargs) => {
                                        if let Some(tx) = pending_calls.remove(&request_id) {
                                            let _ = tx.send(Ok(kwargs));
                                        }
                                    }
                                    IncomingMessage::Subscribed(request_id, subscription_id) => {
                                        if let Some((sink, tx)) = pending_subscribes.remove(&request_id) {
                                            subscriptions.insert(subscription_id, sink);
                                            let _ = tx.send(Ok(subscription_id));
                                        }
                                    }
                                    IncomingMessage::Unsubscribed(request_id) => {
                                        if let Some((subscription_id, tx)) = pending_unsubscribes.remove(&request_id) {
                                            subscriptions.remove(&subscription_id);
                                            let _ = tx.send(Ok(()));
                                        }
                                    }
                                    IncomingMessage::Error(request_id, error, args, kwargs) => {
                                        let err = WampError::CallError {
                                            uri: error,
                                            args,
                                            kwargs,
                                        };
                                        if let Some(tx) = pending_calls.remove(&request_id) {
                                            let _ = tx.send(Err(err));
                                        } else if let Some((_, tx)) = pending_subscribes.remove(&request_id) {
                                            let _ = tx.send(Err(err));
                                        } else if let Some((_, tx)) = pending_unsubscribes.remove(&request_id) {
                                            let _ = tx.send(Err(err));
                                        }
                                    }
                                    IncomingMessage::Event(subscription_id, args, kwargs) => {
                                        let closed = match subscriptions.get(&subscription_id) {
                                            Some(SubscriptionSink::RawKwargs(tx)) => {
                                                tx.send(kwargs).is_err()
                                            }
                                            Some(SubscriptionSink::Event(tx)) => {
                                                let kw_val = serde_json::from_str::<Value>(
                                                    raw_kwargs_str(kwargs.as_deref()),
                                                )
                                                .unwrap_or_else(|_| Value::Object(Map::new()));
                                                tx.send(Event { args, kwargs: kw_val }).is_err()
                                            }
                                            None => false,
                                        };
                                        if closed {
                                            subscriptions.remove(&subscription_id);
                                        }
                                    }
                                    IncomingMessage::Goodbye => {
                                        let reply = Message::Goodbye(
                                            Value::Object(Map::new()),
                                            "wamp.error.goodbye_and_out".to_string(),
                                        );
                                        let _ = send_msg(&mut write_half, &reply).await;
                                        let _ = write_half.close().await;
                                        return Ok(());
                                    }
                                    IncomingMessage::Abort(reason) => {
                                        return Err(WampError::WebSocket(reason));
                                    }
                                    IncomingMessage::Other => {}
                                }
                            }
                            Some(Ok(WsMessage::Close(_))) | None => {
                                return Err(WampError::ConnectionClosed);
                            }
                            Some(Err(e)) => {
                                return Err(WampError::WebSocket(e.to_string()));
                            }
                            Some(_) => {}
                        }
                    }
                    first_cmd = cmd_rx.recv() => {
                        let mut current = first_cmd;
                        let mut shutdown_requested = false;

                        loop {
                            match current {
                                Some(Command::Call { request_id, frame, tx }) => {
                                    if let Err(e) = write_half.feed(WsMessage::Text(frame.into())).await {
                                        let _ = tx.send(Err(WampError::WebSocket(e.to_string())));
                                        return Err(WampError::ConnectionClosed);
                                    }
                                    pending_calls.insert(request_id, tx);
                                }
                                Some(Command::Subscribe { request_id, frame, sink, tx }) => {
                                    if let Err(e) = write_half.feed(WsMessage::Text(frame.into())).await {
                                        let _ = tx.send(Err(WampError::WebSocket(e.to_string())));
                                        return Err(WampError::ConnectionClosed);
                                    }
                                    pending_subscribes.insert(request_id, (sink, tx));
                                }
                                Some(Command::Unsubscribe { request_id, subscription_id, frame, tx }) => {
                                    if let Err(e) = write_half.feed(WsMessage::Text(frame.into())).await {
                                        let _ = tx.send(Err(WampError::WebSocket(e.to_string())));
                                        return Err(WampError::ConnectionClosed);
                                    }
                                    pending_unsubscribes.insert(request_id, (subscription_id, tx));
                                }
                                Some(Command::Shutdown) | None => {
                                    shutdown_requested = true;
                                    break;
                                }
                            }

                            match cmd_rx.try_recv() {
                                Ok(next) => current = Some(next),
                                Err(_) => break,
                            }
                        }

                        if write_half.flush().await.is_err() {
                            return Err(WampError::ConnectionClosed);
                        }

                        if shutdown_requested {
                            let goodbye = Message::Goodbye(
                                Value::Object(Map::new()),
                                "wamp.error.close_realm".to_string(),
                            );
                            let _ = send_msg(&mut write_half, &goodbye).await;
                            let _ = write_half.close().await;
                            return Ok(());
                        }
                    }
                }
            }
        });

        Ok(WampClient {
            request_id: AtomicU64::new(1),
            cmd_tx,
            handle: Some(handle),
        })
    }

    fn next_id(&self) -> RequestId {
        self.request_id.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn call_typed<Options: serde::Serialize, Kwargs: serde::Serialize>(
        &self,
        procedure: &str,
        options: Option<&Options>,
        kwargs: Option<&Kwargs>,
    ) -> WampResult<Option<Box<RawValue>>> {
        let request_id = self.next_id();
        let frame =
            serialize_call(request_id, options, procedure, kwargs).map_err(WampError::Serialize)?;

        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Call {
                request_id,
                frame,
                tx,
            })
            .map_err(|_| WampError::ConnectionClosed)?;

        rx.await.map_err(|_| WampError::ConnectionClosed)?
    }

    pub async fn call(
        &self,
        procedure: &str,
        options: Option<Value>,
        args: Option<Vec<Value>>,
        kwargs: Option<Value>,
    ) -> WampResult<Value> {
        let options = options.filter(|v| !v.is_null());
        let kwargs = kwargs.filter(|v| !v.is_null());

        let raw = match args.filter(|a| !a.is_empty()) {
            None => {
                self.call_typed(procedure, options.as_ref(), kwargs.as_ref())
                    .await?
            }
            Some(positional_args) => {
                let request_id = self.next_id();
                let msg = Message::Call(
                    request_id,
                    options.unwrap_or_else(|| Value::Object(Map::new())),
                    procedure.to_string(),
                    positional_args,
                    kwargs.unwrap_or(Value::Null),
                );
                let frame = serde_json::to_string(&msg).map_err(WampError::Serialize)?;
                let (tx, rx) = oneshot::channel();
                self.cmd_tx
                    .send(Command::Call {
                        request_id,
                        frame,
                        tx,
                    })
                    .map_err(|_| WampError::ConnectionClosed)?;
                rx.await.map_err(|_| WampError::ConnectionClosed)??
            }
        };

        serde_json::from_str::<Value>(raw_kwargs_str(raw.as_deref()))
            .map_err(WampError::Deserialize)
    }

    pub async fn subscribe_with_options<Options: serde::Serialize>(
        &self,
        topic: &str,
        options: Option<&Options>,
        event_tx: mpsc::UnboundedSender<Event>,
    ) -> WampResult<SubscriptionId> {
        let request_id = self.next_id();
        let frame =
            serialize_subscribe(request_id, options, topic).map_err(WampError::Serialize)?;

        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Subscribe {
                request_id,
                frame,
                sink: SubscriptionSink::Event(event_tx),
                tx,
            })
            .map_err(|_| WampError::ConnectionClosed)?;

        rx.await.map_err(|_| WampError::ConnectionClosed)?
    }

    pub async fn subscribe_raw_payload<Options: serde::Serialize>(
        &self,
        topic: &str,
        options: Option<&Options>,
        raw_tx: mpsc::UnboundedSender<Option<Box<RawValue>>>,
    ) -> WampResult<SubscriptionId> {
        let request_id = self.next_id();
        let frame =
            serialize_subscribe(request_id, options, topic).map_err(WampError::Serialize)?;

        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Subscribe {
                request_id,
                frame,
                sink: SubscriptionSink::RawKwargs(raw_tx),
                tx,
            })
            .map_err(|_| WampError::ConnectionClosed)?;

        rx.await.map_err(|_| WampError::ConnectionClosed)?
    }

    pub async fn subscribe(
        &self,
        topic: &str,
        options: Option<Value>,
        event_tx: mpsc::UnboundedSender<Event>,
    ) -> WampResult<SubscriptionId> {
        let options = options.filter(|v| !v.is_null());
        self.subscribe_with_options(topic, options.as_ref(), event_tx)
            .await
    }

    pub async fn unsubscribe(&self, subscription_id: SubscriptionId) -> WampResult<()> {
        let request_id = self.next_id();
        let frame =
            serialize_unsubscribe(request_id, subscription_id).map_err(WampError::Serialize)?;

        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Unsubscribe {
                request_id,
                subscription_id,
                frame,
                tx,
            })
            .map_err(|_| WampError::ConnectionClosed)?;

        rx.await.map_err(|_| WampError::ConnectionClosed)?
    }

    pub async fn shutdown(mut self) -> WampResult<()> {
        let _ = self.cmd_tx.send(Command::Shutdown);
        if let Some(handle) = self.handle.take() {
            handle
                .await
                .map_err(|e| WampError::WebSocket(e.to_string()))?
        } else {
            Ok(())
        }
    }
}

impl Drop for WampClient {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            log::warn!("WampClient dropped without calling shutdown()");
            handle.abort();
        }
    }
}

async fn send_msg<S>(sink: &mut S, msg: &Message) -> WampResult<()>
where
    S: futures_util::Sink<WsMessage, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let text = serde_json::to_string(msg).map_err(WampError::Serialize)?;
    sink.send(WsMessage::Text(text.into()))
        .await
        .map_err(|e| WampError::WebSocket(e.to_string()))
}

#[cfg(test)]
#[allow(clippy::result_large_err)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_hdr_async;
    use tokio_tungstenite::tungstenite::handshake::server::{Request as ServerRequest, Response};

    #[test]
    fn call_message_omits_trailing_empty_fields() {
        let msg_no_args = Message::Call(
            1,
            json!({}),
            "ak.wwise.core.getInfo".to_string(),
            vec![],
            Value::Null,
        );
        let v: Value = serde_json::to_value(&msg_no_args).unwrap();
        assert_eq!(v, json!([48, 1, {}, "ak.wwise.core.getInfo"]));

        let msg_with_kwargs = Message::Call(
            2,
            json!({ "return": ["id", "name"] }),
            "ak.wwise.core.object.get".to_string(),
            vec![],
            json!({ "from": { "ofType": ["Sound"] } }),
        );
        let v: Value = serde_json::to_value(&msg_with_kwargs).unwrap();
        assert_eq!(
            v,
            json!([
                48,
                2,
                { "return": ["id", "name"] },
                "ak.wwise.core.object.get",
                [],
                { "from": { "ofType": ["Sound"] } }
            ])
        );
    }

    #[tokio::test]
    async fn wamp_session_call_subscribe_and_shutdown_roundtrip() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let ws = accept_hdr_async(stream, |req: &ServerRequest, mut resp: Response| {
                assert_eq!(
                    req.headers()
                        .get("Sec-WebSocket-Protocol")
                        .and_then(|v| v.to_str().ok()),
                    Some("wamp.2.json")
                );
                resp.headers_mut()
                    .insert("Sec-WebSocket-Protocol", "wamp.2.json".parse().unwrap());
                Ok(resp)
            })
            .await
            .unwrap();

            let (mut write, mut read) = ws.split();

            // 1. Expect HELLO
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let hello: Message = serde_json::from_str(text.as_str()).unwrap();
            match hello {
                Message::Hello(realm, details) => {
                    assert_eq!(realm, "realm1");
                    assert!(details["roles"]["caller"].is_object());
                    assert!(details["roles"]["subscriber"].is_object());
                }
                other => panic!("unexpected message: {other:?}"),
            }

            // Send WELCOME
            send_msg(&mut write, &Message::Welcome(100, json!({})))
                .await
                .unwrap();

            // 2. Expect CALL with separate options and kwargs
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let call: Message = serde_json::from_str(text.as_str()).unwrap();
            match call {
                Message::Call(req_id, options, proc_uri, args, kwargs) => {
                    assert_eq!(proc_uri, "ak.wwise.core.object.get");
                    assert_eq!(options, json!({ "return": ["id", "name"] }));
                    assert!(args.is_empty());
                    assert_eq!(kwargs, json!({ "from": { "ofType": ["Sound"] } }));

                    let res = Message::Result(
                        req_id,
                        json!({}),
                        vec![],
                        json!({ "objects": [{ "id": "{1234}", "name": "Beep" }] }),
                    );
                    send_msg(&mut write, &res).await.unwrap();
                }
                other => panic!("unexpected message: {other:?}"),
            }

            // 3. Expect SUBSCRIBE with options, then immediately send SUBSCRIBED + EVENT
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let sub: Message = serde_json::from_str(text.as_str()).unwrap();
            match sub {
                Message::Subscribe(req_id, options, topic) => {
                    assert_eq!(topic, "ak.wwise.core.object.created");
                    assert_eq!(options, json!({ "return": ["id"] }));

                    send_msg(&mut write, &Message::Subscribed(req_id, 555))
                        .await
                        .unwrap();
                    send_msg(
                        &mut write,
                        &Message::Event(555, 1, json!({}), vec![], json!({ "object": "{1234}" })),
                    )
                    .await
                    .unwrap();
                }
                other => panic!("unexpected message: {other:?}"),
            }

            // 4. Expect UNSUBSCRIBE
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let unsub: Message = serde_json::from_str(text.as_str()).unwrap();
            match unsub {
                Message::Unsubscribe(req_id, sub_id) => {
                    assert_eq!(sub_id, 555);
                    send_msg(&mut write, &Message::Unsubscribed(req_id))
                        .await
                        .unwrap();
                }
                other => panic!("unexpected message: {other:?}"),
            }

            // 5. Expect GOODBYE on shutdown
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let goodbye: Message = serde_json::from_str(text.as_str()).unwrap();
            assert!(matches!(goodbye, Message::Goodbye(_, _)));
        });

        let client = crate::WaapiClient::new(Some(&addr.to_string()))
            .await
            .unwrap();

        let res = client
            .call(
                crate::WaapiFunction::ak_wwise_core_object_get,
                Some(json!({ "from": { "ofType": ["Sound"] } })),
                Some(json!({ "return": ["id", "name"] })),
            )
            .await
            .unwrap();
        assert_eq!(res["objects"][0]["name"], "Beep");

        let (sub_id, mut events) = client
            .subscribe(
                crate::WaapiTopic::ak_wwise_core_object_created,
                Some(json!({ "return": ["id"] })),
            )
            .await
            .unwrap();
        assert_eq!(sub_id, 555);

        let ev = events.recv().await.unwrap();
        assert_eq!(ev.kwargs["object"], "{1234}");

        client.unsubscribe(sub_id).await.unwrap();
        client.shutdown().await.unwrap();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn server_disconnect_wakes_pending_call() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let ws = accept_hdr_async(stream, |_req: &ServerRequest, mut resp: Response| {
                resp.headers_mut()
                    .insert("Sec-WebSocket-Protocol", "wamp.2.json".parse().unwrap());
                Ok(resp)
            })
            .await
            .unwrap();
            let (mut write, mut read) = ws.split();

            // Complete handshake
            let _ = read.next().await;
            send_msg(&mut write, &Message::Welcome(1, json!({})))
                .await
                .unwrap();

            // Read the CALL and immediately drop the connection without replying
            let _ = read.next().await;
        });

        let client = crate::WaapiClient::new(Some(&addr.to_string()))
            .await
            .unwrap();
        let err = client
            .call(crate::WaapiFunction::ak_wwise_core_getInfo, None, None)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            crate::WaapiError::Wamp(WampError::ConnectionClosed)
        ));
        let _ = client.shutdown().await;
    }

    #[tokio::test]
    async fn call_error_returns_waapi_error_and_recovers_session() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let ws = accept_hdr_async(stream, |_req: &ServerRequest, mut resp: Response| {
                resp.headers_mut()
                    .insert("Sec-WebSocket-Protocol", "wamp.2.json".parse().unwrap());
                Ok(resp)
            })
            .await
            .unwrap();
            let (mut write, mut read) = ws.split();

            // 1. Handshake
            let _ = read.next().await;
            send_msg(&mut write, &Message::Welcome(1, json!({})))
                .await
                .unwrap();

            // 2. First CALL fails with WAMP ERROR (48 = CALL)
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let first_call: Message = serde_json::from_str(text.as_str()).unwrap();
            let Message::Call(req_id, _, proc_uri, _, _) = first_call else {
                panic!("expected CALL message");
            };
            assert_eq!(proc_uri, "ak.wwise.core.object.create");

            let err_msg = Message::Error(
                48,
                req_id,
                json!({}),
                "ak.wwise.invalid_arguments".to_string(),
                vec![],
                json!({ "message": "'parent' already has child with same name." }),
            );
            send_msg(&mut write, &err_msg).await.unwrap();

            // 3. Second CALL on the same session succeeds
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let second_call: Message = serde_json::from_str(text.as_str()).unwrap();
            let Message::Call(req_id2, _, proc_uri2, _, _) = second_call else {
                panic!("expected second CALL message");
            };
            assert_eq!(proc_uri2, "ak.wwise.core.object.get");

            let ok_msg = Message::Result(
                req_id2,
                json!({}),
                vec![],
                json!({ "return": [{ "id": "{1234}", "name": "HealthySound" }] }),
            );
            send_msg(&mut write, &ok_msg).await.unwrap();

            // 4. Expect GOODBYE
            let WsMessage::Text(text) = read.next().await.unwrap().unwrap() else {
                panic!("expected text frame");
            };
            let goodbye: Message = serde_json::from_str(text.as_str()).unwrap();
            assert!(matches!(goodbye, Message::Goodbye(_, _)));
        });

        let client = crate::WaapiClient::new(Some(&addr.to_string()))
            .await
            .unwrap();

        let err = client
            .wwise()
            .core()
            .object()
            .create(
                crate::api::ak::wwise::core::object::CoreObjectCreateArgs::new(
                    crate::ObjectRef::from(r"\Actor-Mixer Hierarchy\Default Work Unit"),
                    crate::ObjectType::Sound,
                    "DuplicateSound".to_string(),
                )
                .on_name_conflict(crate::api::OnNameConflict::Fail),
            )
            .await
            .expect_err("Expected WaapiError::Call from WAMP ERROR response");
        assert!(matches!(err, crate::WaapiError::Call(_)));
        assert_eq!(err.error_uri(), Some("ak.wwise.invalid_arguments"));
        assert_eq!(
            err.waapi_message(),
            Some("'parent' already has child with same name.")
        );

        // Verify the same session remains healthy for subsequent typed calls
        let got = client
            .wwise()
            .core()
            .object()
            .get(
                crate::api::ak::wwise::core::object::CoreObjectGetArgs::new(
                    crate::api::ObjectQuery::from("{1234}"),
                ),
                crate::api::ak::wwise::core::object::CoreObjectGetOptions::new(),
            )
            .await
            .expect("Session should recover cleanly after WaapiError::Call");
        assert_eq!(got.objects.len(), 1);
        assert_eq!(got.objects[0].name.as_deref(), Some("HealthySound"));

        client.shutdown().await.unwrap();
        server.await.unwrap();
    }
}
