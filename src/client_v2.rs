use crate::error::GremlinError;
use crate::io::GraphSON;
use crate::message::Response;
use crate::options::ConnectionOptions;
use crate::prelude::GremlinResult;
use async_tungstenite::tokio::{
    ConnectStream,
    connect_async_with_config,
    connect_async_with_tls_connector_and_config,
};
use async_tungstenite::WebSocketStream;
use bb8::ManageConnection;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{RwLock, mpsc};
use tokio::task::yield_now;
use tokio_rustls::TlsConnector;
use tokio_stream::Stream;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tungstenite::client::{IntoClientRequest, uri_mode};
use tungstenite::stream::{Mode, NoDelay};
use tungstenite::{Connector, Message};
use uuid::Uuid;

pub struct Connection<V> {
    outbound: mpsc::UnboundedSender<Message>,
    valid: Arc<RwLock<bool>>,
    buffer: Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<GremlinResult<Response>>>>>,
    #[doc(hidden)]
    _phantom: std::marker::PhantomData<V>,
}

type WSStream = WebSocketStream<ConnectStream>;

impl<V> Connection<V> {
    pub async fn send(
        &self,
        payload: Vec<u8>,
    ) -> GremlinResult<impl Stream<Item = GremlinResult<Response>>> {
        let id = uuid::Uuid::new_v4();
        let (tx, rx) = mpsc::unbounded_channel();

        self.buffer.write().await.insert(id, tx);
        self.outbound.send(Message::binary(payload))?;

        Ok(UnboundedReceiverStream::new(rx))
    }
    
    fn new(value: WSStream) -> Self {
        let (sink, stream) = value.split();
        let (outbound, outbound_rx) = mpsc::unbounded_channel();
        let valid = Arc::new(RwLock::new(true));
        let buffer = Arc::new(RwLock::new(HashMap::with_capacity(32)));

        Self::proxy(outbound_rx, sink, valid.clone());
        Self::recv(stream, outbound.clone(), buffer.clone(), valid.clone());

        Self {
            outbound,
            valid,
            buffer,
            _phantom: Default::default(),
        }
    }

    fn proxy(
        mut rx: mpsc::UnboundedReceiver<Message>,
        mut sink: SplitSink<WSStream, Message>,
        validity: Arc<RwLock<bool>>,
    ) {
        tokio::spawn(async move {
            let close = async || {
                *validity.write().await = false;
            };
            
            loop {
                if !*validity.read().await {
                    break;
                }

                match rx.try_recv() {
                    Ok(message) => match sink.send(message).await {
                        Ok(_) => {}
                        Err(error) => {
                            tracing::warn!("{:?}", error);
                            close().await;
                        }
                    },
                    Err(recv_err) => match recv_err {
                        TryRecvError::Empty => {}
                        TryRecvError::Disconnected => close().await,
                    },
                }

                yield_now().await;
            }
        });
    }

    fn recv(
        mut stream: SplitStream<WSStream>,
        callback: mpsc::UnboundedSender<Message>,
        requests: Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<GremlinResult<Response>>>>>,
        validity: Arc<RwLock<bool>>,
    ) {
        tokio::spawn(async move {
            let close = async || {
                *validity.write().await = false;
            };
            
            loop {
                if !*validity.read().await {
                    break;
                }

                match stream.next().await {
                    None => {}
                    Some(Ok(message)) => match message {
                        Message::Text(string) => {
                            tracing::warn!("we got a string? {}", string);
                        }
                        Message::Binary(blob) => match serde_json::from_slice::<Response>(&blob) {
                            Ok(response) => {
                                tracing::trace!(
                                    request = &response.request_id.to_string(),
                                    status = &response.status.code
                                );

                                let mut guard = requests.write().await;
                                match response.status.code {
                                    206 => {
                                        let id = response.request_id.clone();
                                        let item = guard.get_mut(&id);
                                        if let Some(callback) = item {
                                            if let Err(error) = callback.send(Ok(response)) {
                                                tracing::warn!(
                                                    request = id.to_string(),
                                                    error = error.to_string()
                                                );
                                                close().await;
                                            }
                                        } else {
                                            tracing::warn!(
                                                request = id.to_string(),
                                                error = "Missing callback"
                                            );
                                        }
                                    }
                                    n if n > 500 => {
                                        guard.remove(&response.request_id);
                                        drop(guard);
                                    }
                                    _ => {
                                        drop(guard);
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::warn!("{}", e);
                            }
                        },
                        Message::Ping(msg) => {
                            if let Err(e) = callback.send(Message::Pong(msg)) {
                                tracing::warn!("{}", e);
                                close().await;
                            }
                        }
                        Message::Pong(msg) => {
                            if let Err(e) = callback.send(Message::Ping(msg)) {
                                tracing::warn!("{}", e);
                                close().await;
                            }
                        }
                        Message::Close(_) => close().await,
                        Message::Frame(_) => {} // idk
                    },
                    Some(Err(e)) => {
                        tracing::warn!("{}", e);
                        close().await;
                    }
                }
            }
        });
    }
}

impl<SD: GraphSON> ManageConnection for ConnectionOptions<SD> {
    type Connection = Connection<SD>;
    type Error = GremlinError;

    fn connect(&self) -> impl Future<Output = Result<Self::Connection, Self::Error>> + Send {
        async move {
            let websocket_url = self.websocket_url();
            let request = websocket_url
                .clone()
                .into_client_request()
                .map_err(|e| GremlinError::Generic(e.to_string()))?;

            let connector = if let Some(opts) = &self.tls_options {
                let config = opts.clone().config()?;
                let config = Arc::new(config);
                Connector::Rustls(config)
            } else {
                Connector::Plain
            };
            let url = request.uri();
            let mode = uri_mode(url).map_err(|e| GremlinError::Generic(e.to_string()))?;
            let host = request
                .uri()
                .host()
                .ok_or_else(|| GremlinError::Generic("No Hostname".into()))?;
            let port = url.port_u16().unwrap_or(match mode {
                Mode::Plain => 80,
                Mode::Tls => 443,
            });
            let mut stream = std::net::TcpStream::connect((host, port))
                .map_err(|e| GremlinError::Generic(format!("Unable to connect {e:?}")))?;
            NoDelay::set_nodelay(&mut stream, true)
                .map_err(|e| GremlinError::Generic(e.to_string()))?;

            let websocket_config = self.websocket_options.clone().map(Into::into);

            let (client, _) = match connector {
                Connector::Plain => connect_async_with_config(url, websocket_config).await,
                Connector::Rustls(config) => {
                    let connector = TlsConnector::from(config);
                    connect_async_with_tls_connector_and_config(
                        url,
                        Some(connector),
                        websocket_config,
                    )
                    .await
                }

                _ => panic!(),
            }?;

            Ok(Connection::new(client))
        }
    }

    fn is_valid(
        &self,
        conn: &mut Self::Connection,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            if !*conn.valid.read().await {
                Err(GremlinError::Generic("Connection is disconnected".into()))
            } else {
                Ok(())
            }
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !*conn.valid.blocking_read()
    }
}
