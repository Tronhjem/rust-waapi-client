pub mod error;
pub mod message;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use futures_util::{SinkExt, StreamExt};
use serde_json::{Map, Value};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;

pub use error::{WampError, WampResult};
pub use message::{Message, RequestId, SubscriptionId};

#[derive(Debug, Clone)]
pub struct Event {
    pub args: Vec<Value>,
    pub kwargs: Value,
}

enum Command {
    Call {
        request_id: RequestId,
        options: Value,
        procedure: String,
        args: Vec<Value>,
        kwargs: Value,
        tx: oneshot::Sender<WampResult<Value>>,
    },
    Subscribe {
        request_id: RequestId,
        options: Value,
        topic: String,
        event_tx: mpsc::UnboundedSender<Event>,
        tx: oneshot::Sender<WampResult<SubscriptionId>>,
    },
    Unsubscribe {
        request_id: RequestId,
        subscription_id: SubscriptionId,
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

        let (ws_stream, _) = connect_async(request).await?;
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
            let mut pending_calls: HashMap<RequestId, oneshot::Sender<WampResult<Value>>> =
                HashMap::new();
            let mut pending_subscribes: HashMap<
                RequestId,
                (
                    mpsc::UnboundedSender<Event>,
                    oneshot::Sender<WampResult<SubscriptionId>>,
                ),
            > = HashMap::new();
            let mut pending_unsubscribes: HashMap<
                RequestId,
                (SubscriptionId, oneshot::Sender<WampResult<()>>),
            > = HashMap::new();
            let mut subscriptions: HashMap<SubscriptionId, mpsc::UnboundedSender<Event>> =
                HashMap::new();

            loop {
                tokio::select! {
                    incoming = read_half.next() => {
                        match incoming {
                            Some(Ok(WsMessage::Text(text))) => {
                                let msg = serde_json::from_str::<Message>(text.as_str())
                                    .map_err(WampError::Deserialize)?;

                                match msg {
                                    Message::Result(request_id, _, _, kwargs) => {
                                        if let Some(tx) = pending_calls.remove(&request_id) {
                                            let _ = tx.send(Ok(kwargs));
                                        }
                                    }
                                    Message::Subscribed(request_id, subscription_id) => {
                                        if let Some((event_tx, tx)) = pending_subscribes.remove(&request_id) {
                                            subscriptions.insert(subscription_id, event_tx);
                                            let _ = tx.send(Ok(subscription_id));
                                        }
                                    }
                                    Message::Unsubscribed(request_id) => {
                                        if let Some((subscription_id, tx)) = pending_unsubscribes.remove(&request_id) {
                                            subscriptions.remove(&subscription_id);
                                            let _ = tx.send(Ok(()));
                                        }
                                    }
                                    Message::Error(_, request_id, _, error, args, kwargs) => {
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
                                    Message::Event(subscription_id, _, _, args, kwargs) => {
                                        let closed = if let Some(tx) = subscriptions.get(&subscription_id) {
                                            tx.send(Event { args, kwargs }).is_err()
                                        } else {
                                            false
                                        };
                                        if closed {
                                            subscriptions.remove(&subscription_id);
                                        }
                                    }
                                    Message::Goodbye(_, _) => {
                                        let reply = Message::Goodbye(
                                            Value::Object(Map::new()),
                                            "wamp.error.goodbye_and_out".to_string(),
                                        );
                                        let _ = send_msg(&mut write_half, &reply).await;
                                        let _ = write_half.close().await;
                                        return Ok(());
                                    }
                                    Message::Abort(_, reason) => {
                                        return Err(WampError::WebSocket(reason));
                                    }
                                    _ => {}
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
                    cmd = cmd_rx.recv() => {
                        match cmd {
                            Some(Command::Call { request_id, options, procedure, args, kwargs, tx }) => {
                                let msg = Message::Call(request_id, options, procedure, args, kwargs);
                                if let Err(e) = send_msg(&mut write_half, &msg).await {
                                    let _ = tx.send(Err(e));
                                    return Err(WampError::ConnectionClosed);
                                }
                                pending_calls.insert(request_id, tx);
                            }
                            Some(Command::Subscribe { request_id, options, topic, event_tx, tx }) => {
                                let msg = Message::Subscribe(request_id, options, topic);
                                if let Err(e) = send_msg(&mut write_half, &msg).await {
                                    let _ = tx.send(Err(e));
                                    return Err(WampError::ConnectionClosed);
                                }
                                pending_subscribes.insert(request_id, (event_tx, tx));
                            }
                            Some(Command::Unsubscribe { request_id, subscription_id, tx }) => {
                                let msg = Message::Unsubscribe(request_id, subscription_id);
                                if let Err(e) = send_msg(&mut write_half, &msg).await {
                                    let _ = tx.send(Err(e));
                                    return Err(WampError::ConnectionClosed);
                                }
                                pending_unsubscribes.insert(request_id, (subscription_id, tx));
                            }
                            Some(Command::Shutdown) | None => {
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

    pub async fn call(
        &self,
        procedure: &str,
        options: Option<Value>,
        args: Option<Vec<Value>>,
        kwargs: Option<Value>,
    ) -> WampResult<Value> {
        let request_id = self.next_id();
        let options = options.unwrap_or_else(|| Value::Object(Map::new()));
        let args = args.unwrap_or_default();
        let kwargs = kwargs.unwrap_or(Value::Null);

        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Call {
                request_id,
                options,
                procedure: procedure.to_string(),
                args,
                kwargs,
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
        let request_id = self.next_id();
        let options = options.unwrap_or_else(|| Value::Object(Map::new()));

        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Subscribe {
                request_id,
                options,
                topic: topic.to_string(),
                event_tx,
                tx,
            })
            .map_err(|_| WampError::ConnectionClosed)?;

        rx.await.map_err(|_| WampError::ConnectionClosed)?
    }

    pub async fn unsubscribe(&self, subscription_id: SubscriptionId) -> WampResult<()> {
        let request_id = self.next_id();

        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Unsubscribe {
                request_id,
                subscription_id,
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

        let mut client = crate::WaapiClient::new(Some(&addr.to_string()))
            .await
            .unwrap();

        let res = client
            .call(
                "ak.wwise.core.object.get",
                Some(json!({ "from": { "ofType": ["Sound"] } })),
                Some(json!({ "return": ["id", "name"] })),
            )
            .await
            .unwrap();
        assert_eq!(res["objects"][0]["name"], "Beep");

        let (sub_id, mut events) = client
            .subscribe(
                "ak.wwise.core.object.created",
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

        let mut client = crate::WaapiClient::new(Some(&addr.to_string()))
            .await
            .unwrap();
        let err = client
            .call("ak.wwise.core.getInfo", None, None)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            crate::WaapiError::Wamp(WampError::ConnectionClosed)
        ));
        let _ = client.shutdown().await;
    }
}
