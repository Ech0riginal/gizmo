#![doc(hidden)]

use crate::io::{GremlinIO, Request, Response};
use crate::{GremlinError, GremlinResult};
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
use crate::network::WSStream;

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

impl Sink {
    pub fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
        self.0.send(msg)
    }
}

impl Into<Sink> for SplitSink<WSStream, Message> {
    fn into(mut self) -> Sink {
        let (tx, mut rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = self.send(msg).await {
                    tracing::warn!("Error sending message: {:?}", e);
                    break;
                }
            }
        });

        Sink(tx)
    }
}

impl Stream {
    pub fn recv(&mut self) -> impl Future<Output = Option<Message>> {
        self.0.recv()
    }
}

impl Into<Stream> for SplitStream<WSStream> {
    fn into(mut self) -> Stream {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(result) = self.next().await {
                match result {
                    Ok(msg) => match tx.send(msg) {
                        Ok(_) => {}
                        Err(e) => {
                            tracing::warn!("Error reading message: {:?}", e);
                            break;
                        }
                    },
                    Err(e) => {
                        tracing::warn!("Error reading message: {:?}", e);
                        break;
                    }
                }
            }
        });

        Stream(rx)
    }
}
