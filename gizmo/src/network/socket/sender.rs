use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use bytes::BytesMut;
use dashmap::DashMap;
use futures::Sink;
use gizmio::*;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::{pin, pin_project};
use tokio_tungstenite::{self, tungstenite::protocol::Message};
use uuid::Uuid;

use crate::client::Supports;
use crate::network::WSStream;
use crate::network::socket::cmd::Cmd;
use crate::{Error, GremlinResult};

#[pin_project]
pub struct SenderLoop<D, F> {
    #[pin]
    sink: futures::stream::SplitSink<WSStream, Message>,
    requests: Arc<DashMap<Uuid, Sender<GremlinResult<Response>>>>,
    #[pin]
    receiver: Receiver<Cmd>,
    in_flight: Option<InFlight>,
    _pd: std::marker::PhantomData<(D, F)>,
}

#[derive(Debug)]
enum InFlight {
    /// Waiting for sink to become ready so we can `start_send`.
    WaitingReady { id: Option<Uuid>, msg: Message },
    /// Message has been `start_send`'d; we’re flushing it out.
    Flushing { id: Option<Uuid> },
}

impl<D, F> SenderLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    pub fn new(
        sink: futures::stream::SplitSink<WSStream, Message>,
        requests: Arc<DashMap<Uuid, Sender<GremlinResult<Response>>>>,
        receiver: Receiver<Cmd>,
    ) -> Pin<Box<Self>> {
        Box::pin(Self {
            sink,
            requests,
            receiver,
            in_flight: None,
            _pd: core::marker::PhantomData,
        })
    }
}

impl<D, F> Future for SenderLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();

        loop {
            if let Some(state) = this.in_flight.as_mut() {
                match state {
                    InFlight::WaitingReady { id, msg } => {
                        match this.sink.as_mut().poll_ready(cx) {
                            Poll::Pending => return Poll::Pending,
                            Poll::Ready(Err(e)) => {
                                if let Some(real_id) = *id {
                                    let err = Error::from(e);
                                    // remove and notify
                                    if let Some((_, tx)) = this.requests.remove(&real_id) {
                                        let _ = tx.send(Err(err));
                                    }
                                }
                                *this.in_flight = None;
                                continue;
                            }
                            Poll::Ready(Ok(())) => {
                                if let Err(e) = this.sink.as_mut().start_send(msg.clone()) {
                                    if let Some(real_id) = *id {
                                        let err = Error::from(e);
                                        if let Some((_, tx)) = this.requests.remove(&real_id) {
                                            let _ = tx.send(Err(err));
                                        }
                                    }
                                    *this.in_flight = None;
                                    continue;
                                }
                                *state = InFlight::Flushing { id: *id };
                                // loop to poll_flush immediately
                                continue;
                            }
                        }
                    }
                    InFlight::Flushing { id } => {
                        match this.sink.as_mut().poll_flush(cx) {
                            Poll::Pending => return Poll::Pending,
                            Poll::Ready(Err(e)) => {
                                if let Some(real_id) = *id {
                                    let err = Error::from(e);
                                    if let Some((_, tx)) = this.requests.remove(&real_id) {
                                        let _ = tx.send(Err(err));
                                    }
                                }
                                *this.in_flight = None;
                                continue;
                            }
                            Poll::Ready(Ok(())) => {
                                // flushed
                                *this.in_flight = None;
                                // continue to accept next command
                                continue;
                            }
                        }
                    }
                }
            }

            match this.receiver.as_mut().poll_recv(cx) {
                Poll::Pending => return Poll::Pending,

                Poll::Ready(None) => match this.sink.as_mut().poll_close(cx) {
                    Poll::Pending => return Poll::Pending,
                    _ => return Poll::Ready(()),
                },

                Poll::Ready(Some(cmd)) => {
                    match cmd {
                        Cmd::Msg((caller, request)) => {
                            this.requests.insert(request.id, caller);
                            match request.serialize::<F, D>() {
                                // pack it up pack it in
                                Ok(serial) => {
                                    let body = serial.into_bytes();
                                    let mut buf =
                                        BytesMut::with_capacity(1 + F::mime.len() + body.len());
                                    buf.extend_from_slice(&[F::mime.len() as u8]);
                                    buf.extend_from_slice(F::mime.as_bytes());
                                    buf.extend_from_slice(&body);
                                    let payload = buf.freeze();

                                    tracing::debug!(id=?request.id, bytes=payload.len());

                                    *this.in_flight = Some(InFlight::WaitingReady {
                                        id: Some(request.id),
                                        msg: Message::Binary(payload),
                                    });

                                    continue;
                                }
                                // jump around
                                Err(e) => {
                                    if let Some((_, tx)) = this.requests.remove(&request.id) {
                                        tokio::spawn(async move {
                                            let _ = tx.send(Err(Error::from(e))).await;
                                        });
                                    }
                                    continue;
                                }
                            }
                        }

                        Cmd::Ping(data) => {
                            *this.in_flight = Some(InFlight::WaitingReady {
                                id: None, // control frame, not tied to a request
                                msg: Message::Pong(data),
                            });
                            continue;
                        }

                        Cmd::Shutdown => {
                            // Drop all outstanding waiters; no awaits.
                            this.requests.clear();
                            // Try to close sink gracefully.
                            match this.sink.as_mut().poll_close(cx) {
                                Poll::Pending => return Poll::Pending,
                                _ => return Poll::Ready(()),
                            }
                        }
                    }
                }
            }
        }
    }
}
