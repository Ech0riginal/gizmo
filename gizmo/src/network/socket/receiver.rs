use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use dashmap::DashMap;
use futures::stream::SplitStream;
use gizmio::*;
use tokio::stream::Stream;
use tokio::sync::mpsc::Sender;
use tokio::task::{self};
use tokio::{pin, pin_project};
use tokio_tungstenite::{self, tungstenite::protocol::Message};
use uuid::Uuid;

use super::Cmd;
use crate::client::Supports;
use crate::network::WSStream;
use crate::{Error, GremlinResult};

#[pin_project]
pub struct ReceiverLoop<D, F> {
    #[pin]
    stream: SplitStream<WSStream>,
    sender: Sender<Cmd>,
    requests: Arc<DashMap<Uuid, Sender<GremlinResult<Response>>>>,
    _pd: PhantomData<(F, D)>,
}

impl<D, F> ReceiverLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    pub fn new(
        stream: SplitStream<WSStream>,
        requests: Arc<DashMap<Uuid, Sender<GremlinResult<Response>>>>,
        sender: Sender<Cmd>,
    ) -> Self {
        ReceiverLoop {
            stream,
            sender,
            requests,
            _pd: PhantomData,
        }
    }
}

impl<D, F> Future for ReceiverLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        // Drain all immediately-ready items; exit as soon as the stream says Pending.
        loop {
            match this.stream.as_mut().poll_next(cx) {
                // --- Stream ended: our Future completes
                Poll::Ready(None) => return Poll::Ready(()),

                // --- An I/O or protocol error: broadcast to all pending requests
                Poll::Ready(Some(Err(error))) => {
                    let mut requests = this.requests.clone();
                    task::spawn(async move {
                        // let mut guard = requests.lock().await;
                        let error_msg = error.to_string();

                        for requests_ref in requests.iter() {
                            // If your senders are `tokio::mpsc::Sender`, this is `.send(..).await`
                            let error = Err(Error::WebsocketClone(error_msg.clone()));
                            let _ = requests_ref.value().send(error).await;
                        }
                        requests.clear();
                    });
                }

                // --- A normal frame arrived
                Poll::Ready(Some(Ok(msg))) => {
                    match msg {
                        Message::Binary(bytes) => {
                            let serial = match F::Serial::from_bytes(bytes) {
                                Ok(s) => s,
                                Err(e) => {
                                    tracing::debug!(error=?e, "serial decode failed");
                                    continue;
                                }
                            };
                            let response = match serial.deserialize::<F, D, Response>() {
                                Ok(r) => r,
                                Err(e) => {
                                    tracing::debug!(error=?e, "response deserialize failed");
                                    continue;
                                }
                            };

                            tracing::debug!(id=?response.id, status=?response.status.code);

                            let is_final = response.status.code.i16() != 206;
                            let id = response.id;
                            let requests = Arc::clone(&this.requests);

                            // Hand off the lock + async sends to a task; driver never waits here.
                            task::spawn(async move {
                                // let mut guard = requests.lock().await;
                                if is_final {
                                    if let Some((id, tx)) = requests.remove(&id) {
                                        if let Err(e) = tx.send(Ok(response)).await {
                                            tracing::error!(
                                                ?id,
                                                ?e,
                                                "deliver final response failed"
                                            );
                                        }
                                    }
                                } else {
                                    if let Some(_ref) = requests.get(&id) {
                                        if let Err(e) = _ref.value().send(Ok(response)).await {
                                            tracing::error!(id=?_ref.key(), ?e, "deliver final response failed");
                                        }
                                    }
                                }
                            });
                        }
                        Message::Ping(data) => {
                            let tx = this.sender.clone();
                            task::spawn(async move {
                                let _ = tx.send(Cmd::Ping(data)).await;
                            });
                        }
                        _ => {
                            // ignore other frames
                        }
                    }
                }

                // --- No more work right now: park the task
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
