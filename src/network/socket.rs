use std::marker::PhantomData;
use std::sync::Arc;
use bytes::Bytes;
use futures::StreamExt;
use serde_json::Value;
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tungstenite::Message;
use crate::{GremlinError, GremlinResult};
use crate::io::{GremlinIO, Request, Response};
use super::*;

/// Handles underlying WebSocket messaging, exposing data relevant only to Gremlin
#[derive(Clone)]
pub struct GremlinSocket<V> {
    valid: Arc<RwLock<bool>>,
    tx: UnboundedSender<Bytes>,
    rx: Arc<RwLock<UnboundedReceiver<Bytes>>>,
    _v: PhantomData<V>,
}

impl GremlinSocket<!> {
    pub fn new<V: GremlinIO>(ws_stream: WSStream) -> GremlinSocket<V> {
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

        GremlinSocket {
            valid,
            tx: req_tx,
            rx: Arc::new(RwLock::new(data_rx)),
            _v: PhantomData::<V>::default(),
        }
    }
}

impl<V: GremlinIO> GremlinSocket<V> {
    pub fn send(&self, request: &Request) -> GremlinResult<()> {
        match V::serialize(request) {
            Ok(json) => {
                let bytes = Bytes::from(json.to_string().into_bytes());
                self.tx.send(bytes).map_err(|_| GremlinError::Closed)?;
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
            .write()
            .await
            .recv()
            .await
            .ok_or(GremlinError::Closed)?;

        match serde_json::from_slice::<Value>(&bytes) {
            Ok(json) => match V::deserialize(&json) {
                Ok(response) => Ok(Some(response)),
                Err(e) => {
                    tracing::warn!("Gremlin deserialization error: {:?}", e);
                    Ok(None)
                }
            },
            Err(e) => {
                tracing::warn!("JSON deserialization error: {:?}", e);
                Ok(None)
            }
        }
    }

    pub async fn valid(&self) -> bool {
        *self.valid.read().await
    }

    pub fn valid_blocking(&self) -> bool {
        *self.valid.blocking_read()
    }
}

unsafe impl<V> Send for GremlinSocket<V> {}

unsafe impl<V> Sync for GremlinSocket<V> {}