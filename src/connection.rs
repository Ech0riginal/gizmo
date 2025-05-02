use crate::prelude::{ConnectionOptions, GraphSON, GremlinError, GremlinResult};

use crate::message::Response;

use async_tungstenite::WebSocketStream;
use async_tungstenite::tungstenite::protocol::{Message, WebSocketConfig};
use async_tungstenite::{self, stream};
use futures::{
    SinkExt, StreamExt,
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};

use async_tungstenite::tokio::TokioAdapter;
use async_tungstenite::tokio::{
    connect_async_with_config, connect_async_with_tls_connector_and_config,
};
use bytes::Bytes;
use futures::channel::mpsc::{Receiver, Sender, channel};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::task::{self};
use tokio_rustls::TlsConnector;
use tungstenite::{
    Connector,
    client::{IntoClientRequest, uri_mode},
    stream::{Mode, NoDelay},
};
use uuid::Uuid;

type WSStream = WebSocketStream<
    stream::Stream<
        TokioAdapter<TcpStream>,
        TokioAdapter<tokio_rustls::client::TlsStream<TcpStream>>,
    >,
>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Cmd {
    Msg((Sender<GremlinResult<Response>>, Uuid, Vec<u8>)),
    Pong(Vec<u8>),
    Shutdown,
}

pub(crate) struct Conn {
    sender: Sender<Cmd>,
    valid: bool,
}

impl std::fmt::Debug for Conn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Conn")
    }
}

impl Conn {
    pub async fn connect<SD: GraphSON, T>(options: T) -> GremlinResult<Conn>
    where
        T: Into<ConnectionOptions<SD>>,
    {
        let options = options.into();
        let websocket_url = options.websocket_url();
        let request = websocket_url
            .clone()
            .into_client_request()
            .map_err(|e| GremlinError::Generic(e.to_string()))?;

        let connector = if let Some(opts) = options.tls_options {
            let config = opts.config()?;
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

        let websocket_config = options.websocket_options.map(Into::into);

        let (client, _) = match connector {
            Connector::Plain => connect_async_with_config(url, websocket_config).await,
            Connector::Rustls(config) => {
                let connector = TlsConnector::from(config);
                connect_async_with_tls_connector_and_config(url, Some(connector), websocket_config)
                    .await
            }
            _ => panic!(),
        }?;

        let (sink, stream) = client.split();
        let (sender, receiver) = channel(20);
        let requests = Arc::new(Mutex::new(HashMap::new()));

        sender_loop(sink, requests.clone(), receiver);

        receiver_loop(stream, requests.clone(), sender.clone());

        Ok(Conn {
            sender,
            valid: true,
        })
    }

    pub async fn send(
        &mut self,
        id: Uuid,
        payload: Vec<u8>,
    ) -> GremlinResult<(Response, Receiver<GremlinResult<Response>>)> {
        let (sender, mut receiver) = channel(1);

        self.sender
            .send(Cmd::Msg((sender, id, payload)))
            .await
            .map_err(|e| {
                self.valid = false;
                e
            })?;

        receiver
            .next()
            .await
            .expect("It should contain the response")
            .map(|r| (r, receiver))
            .map_err(|e| {
                //If there's been an websocket layer error, mark the connection as invalid
                match e {
                    GremlinError::WebsocketClone(_) | GremlinError::WebSocketPool(_) => {
                        self.valid = false;
                    }
                    _ => {}
                }
                e
            })
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        send_shutdown(self);
    }
}

fn send_shutdown(conn: &mut Conn) {
    conn.sender.close_channel();
}

fn sender_loop(
    mut sink: SplitSink<WSStream, Message>,
    requests: Arc<Mutex<HashMap<Uuid, Sender<GremlinResult<Response>>>>>,
    mut receiver: Receiver<Cmd>,
) {
    task::spawn(async move {
        loop {
            match receiver.next().await {
                Some(item) => match item {
                    Cmd::Msg(msg) => {
                        let mut guard = requests.lock().await;

                        guard.insert(msg.1, msg.0);

                        let result = sink.send(Message::Binary(msg.2.into())).await;

                        if let Err(e) = result {
                            let mut sender = guard.remove(&msg.1).unwrap();
                            sender
                                .send(Err(GremlinError::from(e)))
                                .await
                                .expect("Failed to send error");
                        }

                        drop(guard);
                    }
                    Cmd::Pong(data) => {
                        sink.send(Message::Pong(data.into()))
                            .await
                            .expect("Failed to send pong message.");
                    }
                    Cmd::Shutdown => {
                        let mut guard = requests.lock().await;
                        guard.clear();
                    }
                },
                None => {
                    break;
                }
            }
        }
        let _ = sink.close().await;
    });
}

fn receiver_loop(
    mut stream: SplitStream<WSStream>,
    requests: Arc<Mutex<HashMap<Uuid, Sender<GremlinResult<Response>>>>>,
    mut sender: Sender<Cmd>,
) {
    task::spawn(async move {
        // let span = tracing::span!(tracing::Level::DEBUG, "rx");
        // let _enter = span.enter();

        loop {
            match stream.next().await {
                Some(Err(error)) => {
                    let err_str = error.to_string();
                    let mut guard = requests.lock().await;
                    for s in guard.values_mut() {
                        let error = Err(GremlinError::WebsocketClone(err_str.clone()));
                        match s.send(error).await {
                            Ok(_r) => {}
                            Err(_e) => {}
                        }
                    }
                    guard.clear();
                }
                Some(Ok(item)) => match item {
                    Message::Binary(data) => {
                        let response: Response = serde_json::from_slice(&data).unwrap();

                        tracing::trace!(
                            request = &response.request_id.to_string(),
                            status = &response.status.code
                        );

                        let mut guard = requests.lock().await;
                        if response.status.code != 206 {
                            let item = guard.remove(&response.request_id);
                            drop(guard);
                            if let Some(mut s) = item {
                                match s.send(Ok(response)).await {
                                    Ok(_r) => {}
                                    Err(_e) => {}
                                };
                            }
                        } else {
                            let item = guard.get_mut(&response.request_id);
                            if let Some(s) = item {
                                match s.send(Ok(response)).await {
                                    Ok(_r) => {}
                                    Err(_e) => {}
                                };
                            }
                            drop(guard);
                        }
                    }
                    Message::Ping(data) => {
                        let _ = sender.send(Cmd::Pong(data.into())).await;
                    }
                    _ => {}
                },
                None => {
                    break;
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::V3g;

    #[tokio::test]
    async fn it_should_connect() {
        Conn::connect::<V3g, _>(("localhost", 8182u16))
            .await
            .unwrap();
    }
}
