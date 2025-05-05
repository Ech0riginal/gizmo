use crate::{Gremlin, GremlinResult, GremlinError};
use crate::prelude::{ConnectionOptions};
use crate::message::{Message, Response};
use crate::structure::GValue;
use async_tungstenite::WebSocketStream;
use async_tungstenite::tokio::{ConnectStream, connect_async_with_config, connect_async_with_tls_connector_and_config, TokioAdapter};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use bb8::{ManageConnection};
use bytes::Bytes;
use futures::stream::{SplitSink, SplitStream};
use futures::{stream, SinkExt, StreamExt};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use tokio::task::{self};
use tokio::rustls::TlsConnector;
use tokio::tracing;
use tungstenite::{
    Connector,
    client::{IntoClientRequest, uri_mode},
    stream::{Mode, NoDelay},
};
use std::task::Poll;
use serde::Serialize;
use serde_json::Value;
use tokio::pin;
use tokio::stream::Stream;
use tokio::stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{RwLock, mpsc};
use tokio::task::yield_now;
use uuid::Uuid;
use crate::options::Credentials;

type WSStream = WebSocketStream<
    stream::Stream<
        TokioAdapter<TcpStream>,
        TokioAdapter<tokio::rustls::client::TlsStream<TcpStream>>,
    >,
>;
pub struct Connection {
    outbound: mpsc::UnboundedSender<Message>,
    valid: Arc<RwLock<bool>>,
    buffer: Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<GremlinResult<Response>>>>>,
}

#[pin_project::pin_project]
pub struct GremlinStream<V> {
    #[doc(hidden)]
    _v: PhantomData<V>,
    #[pin]
    inner: UnboundedReceiverStream<GremlinResult<Response>>,
}

impl<V: Gremlin> Stream for GremlinStream<V> {
    type Item = GremlinResult<GValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        
        match this.inner.poll_next(cx) {
            Poll::Ready(Some(Ok(response))) => {
                match serde_json::to_value(&response.result.data) {
                    Ok(value) => {
                        let result = V::deserialize(&value);
                        Poll::Ready(Some(result))
                    }
                    Err(e) => {
                        Poll::Ready(Some(Err(GremlinError::from(e))))
                    },
                }
            }
            Poll::Ready(Some(Err(e))) => {
                Poll::Ready(Some(Err(e)))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

// pub trait GremlinStream: Stream<Item = GremlinResult<Response>> {}
// impl<S> GremlinStream for S where S: Stream<Item = GremlinResult<Response>> {}

struct Context {
    callback: UnboundedSender<Message>,
    requests: Arc<RwLock<HashMap<Uuid, UnboundedSender<GremlinResult<Response>>>>>,
    validity: Arc<RwLock<bool>>,
    credentials: Option<Credentials>,
}

type WSStream = WebSocketStream<ConnectStream>;

impl<SD: Gremlin> ManageConnection for ConnectionOptions<SD> {
    type Connection = Connection;
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

            Ok(Connection::new(client, &self))
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

impl Connection {
    fn new<V: Gremlin>(value: WSStream, options: &ConnectionOptions<V>) -> Self {
        let (sink, stream) = value.split();
        let (outbound, outbound_rx) = mpsc::unbounded_channel();
        let valid = Arc::new(RwLock::new(true));
        let buffer = Arc::new(RwLock::new(HashMap::with_capacity(32)));
        let ctx = Context {
            callback: outbound.clone(),
            requests: buffer.clone(),
            validity: valid.clone(),
            credentials: options.credentials.clone(),
        };

        Self::proxy::<V>(outbound_rx, sink, valid.clone());

        Self::recv::<V>(ctx, stream);

        Self {
            outbound,
            valid,
            buffer,
        }
    }

    fn proxy<V: Gremlin>(
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

    fn recv<V: Gremlin>(ctx: Context, stream: SplitStream<WSStream>) {
        tokio::spawn(async move {
            pin!(stream);

            loop {
                if !*ctx.validity.read().await {
                    break;
                }

                match stream.next().await {
                    Some(Ok(message)) => Self::message_handler::<V>(&ctx, message).await,
                    Some(Err(e)) => {
                        ctx.close().await;
                        tracing::warn!("{}", e);
                    }
                    None => {
                        ctx.close().await;
                    }
                }
            }
        });
    }

    #[tracing::instrument(skip(ctx, message))]
    async fn message_handler<V: Gremlin>(ctx: &Context, message: Message) {
        match message {
            Message::Text(string) => {
                tracing::warn!("we got a string? {}", string);
            }
            Message::Binary(blob) => match serde_json::from_slice::<Response>(&blob) {
                Ok(response) => Self::response_handler::<V>(&ctx, response).await,
                Err(e) => tracing::warn!("{}", e),
            },
            Message::Ping(msg) => ctx.callback(Message::Pong(msg)).await,
            Message::Pong(msg) => ctx.callback(Message::Ping(msg)).await,
            Message::Close(_) => ctx.close().await,
            Message::Frame(_) => {} // idk
        }
    }

    #[tracing::instrument(skip(ctx, response), fields(request = %response.request_id, status = %response.status.code))]
    async fn response_handler<V: Gremlin>(ctx: &Context, response: Response) {
        match response.status.code {
            200 | 206 => ctx.callback(response).await,
            407 => match &ctx.credentials {
                Some(c) => Self::authenticate::<V>(ctx, c, &response.request_id).await,
                None => ctx.callback(response).await,
            },
            n if n > 500 => ctx.remove(response).await,
            _ => {}
        }
    }

    #[tracing::instrument(skip(ctx, credentials, id))]
    async fn authenticate<V: Gremlin>(ctx: &Context, credentials: &Credentials, id: &Uuid) {
        let mut args = HashMap::new();

        args.insert(
            String::from("sasl"),
            GValue::String(BASE64_STANDARD.encode(&format!(
                "\0{}\0{}",
                credentials.username, credentials.password
            ))),
        );

        let args = match V::serialize(&GValue::from(args)) {
            Ok(value) => value,
            Err(e) => {
                tracing::error!("{}", e);
                *ctx.validity.write().await = false;
                return;
            }
        };
        let message = V::message(
            String::from("authentication"),
            String::from("traversal"),
            args,
            Some(id.clone()),
        );
        let blob = serde_json::to_vec(&message).unwrap();
        let bytes = Bytes::from(blob);

        ctx.callback(Message::Binary(bytes)).await;
    }

    pub async fn send<I, V>(&self, payload: I) -> GremlinResult<GremlinStream<V>>
    where
        I: Payload,
        V: Gremlin
    {
        payload.send(self).await
    }
}

trait Payload {
    fn send<V: Gremlin>(
        self,
        conn: &Connection,
    ) -> impl Future<Output = GremlinResult<GremlinStream<V>>>;
}

impl Payload for Message {
    fn send<V: Gremlin>(
        self,
        conn: &Connection,
    ) -> impl Future<Output = GremlinResult<GremlinStream<V>>> {
        let id = Uuid::new_v4();
        send(conn, id, self)
    }
}

impl<T: Serialize> Payload for crate::message::Message<T> {
    fn send<V: Gremlin>(self, conn: &Connection) -> impl Future<Output=GremlinResult<GremlinStream<V>>> {
        async move {
            let blob = serde_json::to_vec(&self)?;
            blob.send(conn).await
        }
    }
}

impl Payload for Vec<u8> {
    fn send<V: Gremlin>(
        self,
        conn: &Connection,
    ) -> impl Future<Output = GremlinResult<GremlinStream<V>>> {
        let id = Uuid::new_v4();
        send(conn, id, Message::binary(self))
    }
}

async fn send<V: Gremlin>(conn: &Connection, id: Uuid, msg: Message) -> Result<GremlinStream<V>, GremlinError>
{
    let (tx, rx) = mpsc::unbounded_channel();
    let inner = UnboundedReceiverStream::new(rx);

    conn.buffer.write().await.insert(id, tx);
    conn.outbound.send(Message::binary(msg))?;

    Ok(GremlinStream { inner, _v: Default::default() })
}


impl Context {
    async fn callback<I>(&self, message: I)
    where
        I: Callback,
    {
        message.callback(self).await;
    }

    async fn close(&self) {
        *self.validity.write().await = false;
    }

    async fn remove(&self, response: Response) {
        self.requests.write().await.remove(&response.request_id);
    }
}

trait Callback {
    fn callback(self, ctx: &Context) -> impl Future<Output = ()>;
}

impl Callback for Response {
    fn callback(self, ctx: &Context) -> impl Future<Output = ()> {
        async move {
            let mut guard = ctx.requests.write().await;
            let id = self.request_id.clone();
            let item = guard.get_mut(&id);

            if let Some(callback) = item {
                if let Err(error) = callback.send(Ok(self)) {
                    tracing::warn!(request = id.to_string(), error = error.to_string());
                    ctx.close().await;
                }
            } else {
                tracing::warn!(request = id.to_string(), error = "Missing callback");
            }
        }
    }
}

impl Callback for Message {
    fn callback(self, ctx: &Context) -> impl Future<Output = ()> {
        async move {
            if let Err(e) = ctx.callback.send(self) {
                tracing::warn!("{}", e);
                ctx.close().await;
            }
        }
    }
}
