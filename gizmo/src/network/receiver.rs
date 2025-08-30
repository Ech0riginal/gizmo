use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::collections::VecDeque;
use std::ops::Deref;
use std::{marker::PhantomData, sync::Arc};

use dashmap::DashMap;
use futures::{Sink, Stream};
use gizmio::{Bytable, DeserializeExt, Dialect, Response};
use pin_project::pin_project;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::TrySendError;
use tokio::task;
use tokio_tungstenite::tungstenite::protocol::Message;
use uuid::Uuid;

use super::Cmd;
use crate::client::Supports;
use crate::network::WSStream;
use crate::{Error, GremlinResult};

const POLL_BUDGET: usize = 8;

#[pin_project]
pub struct ReceiverLoop<D, F> {
    pub valid: bool,
    #[pin]
    stream: futures::stream::SplitStream<WSStream>,
    sender: Sender<Cmd>,
    requests: Arc<DashMap<Uuid, Sender<GremlinResult<Response>>>>,
    head: Pending,
    queue: VecDeque<Pending>,
    _pd: PhantomData<(F, D)>,
}

impl<D, F> ReceiverLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    pub fn new(
        stream: futures::stream::SplitStream<WSStream>,
        requests: Arc<DashMap<Uuid, Sender<GremlinResult<Response>>>>,
        sender: Sender<Cmd>,
    ) -> Pin<Box<Self>> {
        Box::pin(Self {
            valid: true,
            stream,
            sender,
            requests,
            head: Pending::None,
            queue: VecDeque::with_capacity(32),
            _pd: PhantomData,
        })
    }
}

#[derive(Debug, PartialEq)]
enum Pending {
    None,
    Call(Response),
}

#[derive(Debug)]
struct Call {
    id: Uuid,
    resp: Response,
}

impl PartialEq for Call {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<D, F> Future for ReceiverLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();

        if let Pending::Call(call) = this.head {
            match this.requests.remove(&call.id) {
                Some((id, tx)) => match tx.try_send(Ok(call.clone())) {
                    Ok(_) if call.status.code.i16() != 206 => {
                        std::mem::swap(&mut *this.head, &mut Pending::None);
                    }
                    Ok(_) => {
                        this.requests.insert(id, tx);
                        std::mem::swap(&mut *this.head, &mut Pending::None);
                    }
                    Err(error) => match error {
                        TrySendError::Full(_) => {
                            return Poll::Pending;
                        }
                        TrySendError::Closed(_) => {
                            _ = this.requests.remove(&call.id);
                        }
                    },
                },
                None => {}
            }
        }

        match this.stream.as_mut().poll_next(cx) {
            Poll::Pending => Poll::Pending,

            Poll::Ready(None) => {
                let senders: Vec<_> = this.requests.iter().map(|e| e.value().clone()).collect();
                this.requests.clear();
                task::spawn(async move {
                    let msg = "websocket closed".to_string();
                    for tx in senders {
                        let _ = tx.send(Err(Error::WebsocketClone(msg.clone()))).await;
                    }
                });
                Poll::Ready(())
            }

            Poll::Ready(Some(Err(error))) => {
                let senders: Vec<_> = this.requests.iter().map(|e| e.value().clone()).collect();
                this.requests.clear();

                let err_str = error.to_string();
                task::spawn(async move {
                    for tx in senders {
                        let _ = tx.send(Err(Error::WebsocketClone(err_str.clone()))).await;
                    }
                });
                Poll::Ready(())
            }

            Poll::Ready(Some(Ok(msg))) => match msg {
                Message::Binary(bytes) => {
                    let serial = match F::Serial::from_bytes(bytes) {
                        Ok(s) => s,
                        Err(e) => {
                            tracing::debug!(error=?e, "serial decode failed");
                            return Poll::Pending;
                        }
                    };
                    let response = match serial.deserialize::<F, D, Response>() {
                        Ok(r) => r,
                        Err(e) => {
                            tracing::debug!(error=?e, "response deserialize failed");
                            return Poll::Pending;
                        }
                    };

                    tracing::debug!(id=?response.id, status=?response.status.code);

                    let new = Pending::Call(response);
                    *this.head = new;
                    // Always poll after this path
                    cx.waker().wake_by_ref();

                    Poll::Pending
                }

                Message::Ping(data) => {
                    let tx: Sender<Cmd> = this.sender.clone();

                    task::spawn(async move {
                        let _ = tx.send(Cmd::Ping(data)).await;
                    });

                    Poll::Pending
                }

                Message::Close(option) => {
                    *this.valid = false;

                    let reason = if let Some(frame) = option {
                        tracing::debug!(code=?frame.code, reason=?frame.reason);
                        String::from_utf8_lossy(frame.reason.as_bytes()).to_string()
                    } else {
                        "Closing".into()
                    };

                    tracing::debug!(?reason);

                    let senders: Vec<_> = this.requests.iter().map(|e| e.value().clone()).collect();
                    this.requests.clear();

                    task::spawn(async move {
                        for tx in senders {
                            let _ = tx.send(Err(Error::WebsocketClone(reason.clone()))).await;
                        }
                    });

                    Poll::Ready(())
                }

                Message::Pong(_) | Message::Text(_) | _ => Poll::Pending,
            },
        }
    }
}
