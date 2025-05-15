use super::*;
use crate::io::{Args, GremlinIO, Request};
use crate::options::{ConnectionOptions, Credentials};
use crate::{GValue, GremlinError, GremlinResult};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use bb8::PooledConnection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::stream::wrappers::UnboundedReceiverStream;
use tokio::sync::{RwLock, mpsc};
use tokio::task::JoinHandle;
use tokio::time::Instant;

type RequestMap = HashMap<uuid::Uuid, mpsc::UnboundedSender<Option<GremlinResult<GValue>>>>;

/// Provides a stream interface for requests' response data
pub struct Connection<V> {
    pub(crate) socket: Arc<RwLock<GremlinSocket<V>>>,
    pub(crate) pending: Arc<RwLock<RequestMap>>,
    handle: JoinHandle<GremlinResult<()>>,
}

impl<V> Drop for Connection<V> {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl<V: GremlinIO> Connection<V> {
    pub async fn valid(&self) -> bool {
        self.socket.read().await.valid().await
    }

    pub fn healthcheck(&self) -> bool {
        self.socket.blocking_read().valid_blocking()
    }

    pub async fn send<'a>(
        self: PooledConnection<'_, ConnectionOptions<V>>,
        request: Request,
    ) -> GremlinResult<impl GremlinStream + use<'a, V>> {
        let (tx, rx) = mpsc::unbounded_channel();

        self.socket.write().await.send(&request)?;
        self.pending.write().await.insert(request.id, tx);

        let stream = UnboundedReceiverStream::new(rx);

        drop(self);

        Ok(stream)
    }
}

impl<V: GremlinIO> Connection<V> {
    pub fn new(ws_stream: WSStream, options: ConnectionOptions<V>) -> Self {
        let socket = Arc::new(RwLock::new(GremlinSocket::new(ws_stream)));
        let pending = Arc::new(RwLock::new(HashMap::with_capacity(64)));

        Self {
            socket: socket.clone(),
            pending: pending.clone(),
            handle: Self::listener(socket, pending, options.credentials),
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
                        200 => {
                            drop(safe_socket);

                            match pending.write().await.remove(&response.id) {
                                Some(callback) => {
                                    callback
                                        .send(Some(Ok(response.data)))
                                        .map_err(|_| GremlinError::Closed)?;
                                    callback.send(None).map_err(|_| GremlinError::Closed)?;
                                }
                                _ => {}
                            }
                        }
                        206 => {
                            drop(safe_socket);

                            match pending.read().await.get(&response.id) {
                                Some(callback) => callback
                                    .send(Some(Ok(response.data)))
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
                                // TODO request ids could get lost in here
                                match pending.read().await.get(&response.id) {
                                    Some(callback) => callback
                                        .send(Some(Ok(response.data)))
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
