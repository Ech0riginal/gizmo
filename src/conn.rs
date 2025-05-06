use crate::io::{GremlinIO, Request, Response};
use crate::{GValue, GremlinError, GremlinResult};
use bytes::Bytes;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{RwLock, mpsc};
use tokio::task::JoinHandle;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use crate::io::Args;
use crate::options::Credentials;
// use crate::ws::{GremlinSocket, WSStream};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use std::collections::HashMap;
use tokio::stream::wrappers::UnboundedReceiverStream;
use tokio::time::Instant;
use uuid::Uuid;

type RequestMap = HashMap<Uuid, mpsc::UnboundedSender<GremlinResult<GValue>>>;

pub type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub trait GremlinStream: tokio::stream::Stream<Item=GremlinResult<GValue>> {}

/// Provides a stream interface for requests' response data
pub struct Connection<V> {
    socket: Arc<RwLock<GremlinSocket<V>>>,
    pending: Arc<RwLock<RequestMap>>,
    handle: JoinHandle<GremlinResult<()>>,
}

/// Handles underlying WebSocket messaging, exposing data relevant only to Gremlin
#[derive(Clone)]
pub struct GremlinSocket<V> {
    valid: Arc<RwLock<bool>>,
    tx: UnboundedSender<Bytes>,
    rx: Arc<RwLock<UnboundedReceiver<Bytes>>>,
    _v: PhantomData<V>,
}

struct Stream(UnboundedReceiver<Message>);

#[derive(Clone)]
struct Sink(UnboundedSender<Message>);
















impl<S> GremlinStream for S where S: Stream<Item=GremlinResult<GValue>> {}



impl<S> GremlinStream for S where S: Stream<Item=GremlinResult<GValue>> {}

impl<V> Drop for Connection<V> {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl<V: GremlinIO> Connection<V> {
    pub async fn send(
        &self,
        request: Request,
    ) -> GremlinResult<impl GremlinStream> {
        let (tx, rx) = mpsc::unbounded_channel();

        self.socket.write().await.send(&request)?;
        self.pending.write().await.insert(request.id, tx);

        let stream = UnboundedReceiverStream::new(rx);

        Ok(stream)
    }
}


impl<V: GremlinIO> Connection<V> {
    pub fn new(ws_stream: WSStream, credentials: Option<Credentials>) -> Self {
        let socket = Arc::new(RwLock::new(GremlinSocket::new(ws_stream)));
        let pending = Arc::new(RwLock::new(HashMap::with_capacity(64)));

        Self {
            socket: socket.clone(),
            pending: pending.clone(),
            handle: Self::listener(socket, pending, credentials),
        }
    }

    fn listener(
        socket: Arc<RwLock<GremlinSocket<V>>>,
        pending: Arc<RwLock<RequestMap>>,
        credentials: Option<Credentials>,
    ) -> JoinHandle<GremlinResult<()>> {
        tokio::spawn(async move {
            let mut last_healthcheck = Instant::now();

            loop {
                let mut safe_socket = socket.write().await;

                if last_healthcheck.elapsed().as_secs() > 7 {
                    if !safe_socket.valid().await {
                        drop(safe_socket);
                        break;
                    } else {
                        last_healthcheck = Instant::now();
                    }
                }

                match safe_socket.recv().await {
                    Ok(Some(response)) => match response.status.code {
                        200 | 206 => {
                            drop(safe_socket);

                            match pending.read().await.get(&response.id) {
                                Some(callback) => callback
                                    .send(Ok(response.result))
                                    .map_err(|_| GremlinError::Closed)?,
                                _ => {}
                            }
                        }
                        407 => match &credentials {
                            Some(c) => {
                                let request =
                                    Request::builder()
                                        .id(response.id.clone())
                                        .op("authentication")
                                        .proc("traversal")
                                        .args(Args::new().arg(
                                            "sasl",
                                            GValue::String(BASE64_STANDARD.encode(&format!(
                                                "\0{}\0{}",
                                                c.username, c.password
                                            ))),
                                        ))
                                        .build()
                                        .unwrap();
                                safe_socket.send(&request)?;
                                drop(safe_socket);
                            }
                            None => {
                                drop(safe_socket);

                                match pending.read().await.get(&response.id) {
                                    Some(callback) => callback
                                        .send(Ok(response.result))
                                        .map_err(|_| GremlinError::Closed)?,
                                    _ => {}
                                }
                            }
                        },
                        n if n > 500 => {
                            drop(safe_socket);
                            _ = pending.write().await.remove(&response.id)
                        }
                        _ => {
                            drop(safe_socket);
                        }
                    },
                    Ok(None) => {
                        drop(safe_socket);
                    }
                    Err(error) => {
                        drop(safe_socket);

                        match error {
                            GremlinError::Json(_) => {}
                            GremlinError::Websocket(_) | GremlinError::Closed => break,
                            e => panic!("Unhandled error type emitted by GremlinSocket! {:?}", e),
                        }
                    }
                }
            }

            Ok(())
        })
    }
}
