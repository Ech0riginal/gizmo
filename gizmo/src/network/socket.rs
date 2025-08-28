use super::*;
use crate::{Error, GremlinResult};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use gizmio::{
    Bytable, DeserializeExt, Deserializer, Dialect, Format, Request, Response, SerializeExt,
    Serializer,
};
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{RwLock, mpsc};
use tungstenite::Message;

/// Handles underlying WebSocket messaging, exposing data relevant only to Gremlin
///
/// TODO get rid of this shit and swap to Actix's WS implementation
#[derive(Clone)]
pub struct GremlinSocket<V> {
    valid: Arc<RwLock<bool>>,
    tx: UnboundedSender<Bytes>,
    rx: Arc<RwLock<UnboundedReceiver<Bytes>>>,
    _v: PhantomData<V>,
}

pub struct NewGremlinSocket<D, F> {
    pub valid: Arc<RwLock<bool>>,
    tx: UnboundedSender<Bytes>,
    rx: UnboundedReceiver<Bytes>,
    _f: PhantomData<F>,
    _d: PhantomData<D>,
}


impl NewGremlinSocket<!, !> {
    pub fn new<D, F>(ws_stream: WSStream) -> NewGremlinSocket<D, F>
    where
        F: Format,
        D: Dialect,
    {
        let valid = Arc::new(RwLock::new(true));
        let (data_tx, data_rx) = mpsc::unbounded_channel();
        let (req_tx, mut req_rx) = mpsc::unbounded_channel();
        let (sink, mut stream): (Sink, Stream) = {
            let tmp = ws_stream.split();
            (tmp.0.into(), tmp.1.into())
        };

        let sink_clone = sink.clone();
        let valid_clone = valid.clone();

        tokio::spawn(async move {
            macro_rules! send {
                ($sender:ident, $data:expr) => {
                    if let Err(e) = $sender.send($data) {
                        tracing::warn!("Error sending message: {:?}", e);
                        break;
                    }
                };
            }

            while let Some(msg) = stream.recv().await {
                match msg {
                    Message::Text(data) => {
                        let bytes = Bytes::from(data);
                        send!(data_tx, bytes);
                    }
                    Message::Binary(bytes) => {
                        send!(data_tx, bytes);
                    }
                    Message::Ping(data) => {
                        send!(sink_clone, Message::Pong(data));
                    }
                    Message::Pong(data) => {
                        send!(sink_clone, Message::Ping(data));
                    }
                    Message::Close(Some(close_frame)) => {
                        tracing::warn!(
                            "WebSocket closing {}. Reason: {}",
                            &close_frame.code,
                            &close_frame.reason
                        );
                        if let Err(e) = sink_clone.send(Message::Close(Some(close_frame))) {
                            tracing::warn!("Failed to acknowledge close frame: {:?}", e);
                        }
                        break;
                    }
                    Message::Close(None) => {
                        break;
                    }
                    Message::Frame(_frame) => {
                        unimplemented!("ws frame") // TODO
                    }
                }
            }

            *valid_clone.write().await = false;
        });

        let valid_clone = valid.clone();
        tokio::spawn(async move {
            while let Some(bytes) = req_rx.recv().await {
                match sink.send(Message::Binary(bytes)) {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::warn!("Error sinking message: {:?}", e);
                        break;
                    }
                }
            }

            *valid_clone.write().await = false;
        });

        NewGremlinSocket {
            valid,
            tx: req_tx,
            rx: data_rx,
            _f: PhantomData::<F>::default(),
            _d: PhantomData::<D>::default(),
        }
    }
}

impl<D, F> NewGremlinSocket<D, F> {
    pub async fn valid(&self) -> bool {
        *self.valid.read().await
    }

    pub fn valid_blocking(&self) -> bool {
        *self.valid.blocking_read()
    }
}

impl<D, F> NewGremlinSocket<D, F>
where
    F: Format,
    F: Serializer<Request, F::Serial, D>,
    F: Deserializer<Response, F::Serial, D>,
    D: Dialect,
{
    pub fn send(&self, request: &Request) -> GremlinResult<()> {
        match request.serialize::<F, D>() {
            Ok(serial) => {
                let mut payload = BytesMut::with_capacity(2048);
                payload.extend(&[F::mime.len() as u8]);
                payload.extend(F::mime.bytes());
                payload.extend(serial.into_bytes());
                let bytes = payload.freeze();
                tracing::trace!("serialized {} bytes", bytes.len());
                tracing::debug!("sending {}", String::from_utf8_lossy(&bytes));
                self.tx.send(bytes).map_err(|_| Error::Closed)?;
            }
            Err(e) => {
                tracing::warn!("Serialization error: {:?}", e);
            }
        }

        Ok(())
    }

    pub async fn recv(&mut self) -> GremlinResult<Option<Response>> {
        let bytes = self
            .rx
            .recv()
            .await
            .ok_or_else(|| Error::Closed)?;

        let serial = match F::Serial::from_bytes(bytes) {
            Ok(serial) => serial,
            Err(e) => {
                tracing::warn!("Malformed serialization format: {:?}", e);
                return Ok(None);
            }
        };

        match serial.deserialize::<F, D, Response>() {
            Ok(response) => Ok(Some(response)),
            Err(e) => {
                tracing::warn!("Malformed response: {:?}", e);
                Ok(None)
            }
        }
    }
}

unsafe impl<D, F> Send for NewGremlinSocket<D, F> {}

unsafe impl<D, F> Sync for NewGremlinSocket<D, F> {}
