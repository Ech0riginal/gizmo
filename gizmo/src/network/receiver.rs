use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::marker::PhantomData;

use futures::Stream;
use gizmio::{Bytable, DeserializeExt, Dialect, Response};
use pin_project::pin_project;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::TrySendError;
use tokio::task;
use tokio_tungstenite::tungstenite::protocol::Message;

use super::{Cmd, RequestMap};
use crate::client::Supports;
use crate::network::WSStream;

#[pin_project]
pub struct ReceiverLoop<D, F> {
    pub valid: bool,
    #[pin]
    stream: futures::stream::SplitStream<WSStream>,
    sender: Sender<Cmd>,
    requests: RequestMap,
    head: Pending,
    _pd: PhantomData<(F, D)>,
}

impl<D, F> ReceiverLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    pub fn new(
        stream: futures::stream::SplitStream<WSStream>,
        requests: RequestMap,
        sender: Sender<Cmd>,
    ) -> Pin<Box<Self>> {
        Box::pin(Self {
            valid: true,
            stream,
            sender,
            requests,
            head: Pending::None,
            _pd: PhantomData,
        })
    }
}

#[derive(Debug)]
enum Pending {
    None,
    Call(Response),
    Error(String),
    Closed(String),
}

impl<D, F> Future for ReceiverLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();

        let mut pending = Pending::None;
        std::mem::swap(&mut *this.head, &mut pending);

        match pending {
            Pending::None => {}
            Pending::Call(call) => {
                if let Some(tx) = this.requests.get(&call.id) {
                    match tx.try_send(Ok(call.clone())) {
                        Ok(_) => {
                            if call.status.code.i16() != 206 {
                                return Poll::Ready(());
                            }
                        }
                        Err(error) => match error {
                            TrySendError::Full(_) => {
                                std::mem::swap(&mut *this.head, &mut Pending::Call(call));
                                cx.waker().wake_by_ref();
                                return Poll::Pending;
                            }
                            TrySendError::Closed(_) => {
                                return Poll::Ready(());
                            }
                        },
                    }
                }
            }
            Pending::Error(error) => {
                tracing::debug!(?error);
                return Poll::Ready(());
            }
            Pending::Closed(reason) => {
                tracing::debug!(?reason);
                return Poll::Ready(());
            }
        }

        match this.stream.as_mut().poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(()),

            Poll::Ready(Some(Err(error))) => {
                *this.head = Pending::Error(error.to_string());
                cx.waker().wake_by_ref();
                Poll::Pending
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
                    // // Always poll after this path
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

                    *this.head = Pending::Closed(reason.clone());
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }

                Message::Pong(_) | Message::Text(_) | _ => Poll::Pending,
            },
        }
    }
}
