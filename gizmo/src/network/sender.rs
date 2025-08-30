use core::{
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::{Bytes, BytesMut};
use futures::Sink;
use gizmio::{Bytable, Dialect, SerializeExt};
use pin_project::pin_project;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::tungstenite::protocol::Message;
use uuid::Uuid;

use crate::Error;
use crate::client::Supports;
use crate::network::{Cmd, RequestMap, WSStream};

const POLL_BUDGET: usize = 64;

#[pin_project]
pub struct SenderLoop<D, F> {
    #[pin]
    sink: futures::stream::SplitSink<WSStream, Message>,
    requests: RequestMap,
    #[pin]
    receiver: Receiver<Cmd>,
    in_flight: Option<InFlight>,
    _pd: core::marker::PhantomData<(D, F)>,
}

#[derive(Debug)]
enum InFlight {
    Queued { id: Uuid, msg: Message },
    // WaitingError { id: Uuid, error: Error },
    Flushing { id: Uuid },
}

impl<D, F> SenderLoop<D, F>
where
    D: Dialect,
    F: Supports<D>,
{
    pub fn new(
        sink: futures::stream::SplitSink<WSStream, Message>,
        requests: RequestMap,
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

        for _ in 0..POLL_BUDGET {
            if let Some(state) = this.in_flight.as_mut() {
                match state {
                    InFlight::Queued { id, msg } => match this.sink.as_mut().poll_ready(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(Err(error)) => {
                            tracing::error!(?id, ?error);
                            continue;
                        }
                        Poll::Ready(Ok(())) => {
                            let to_send = mem::replace(msg, Message::Binary(Bytes::new()));
                            if let Err(error) = this.sink.as_mut().start_send(to_send) {
                                tracing::error!(?id, ?error);
                                continue;
                            }
                            *state = InFlight::Flushing { id: *id };

                            continue;
                        }
                    },
                    InFlight::Flushing { id } => match this.sink.as_mut().poll_flush(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(Err(error)) => {
                            tracing::error!(?id, ?error);
                            continue;
                        }
                        Poll::Ready(Ok(())) => {
                            *this.in_flight = None;

                            continue;
                        }
                    },
                }
            }

            match this.receiver.as_mut().poll_recv(cx) {
                Poll::Pending => return Poll::Pending,

                Poll::Ready(None) => match this.sink.as_mut().poll_close(cx) {
                    Poll::Pending => return Poll::Pending,
                    _ => return Poll::Ready(()),
                },

                Poll::Ready(Some(cmd)) => match cmd {
                    Cmd::Msg((caller, request)) => match request.serialize::<F, D>() {
                        Ok(serial) => {
                            this.requests.insert(request.id, caller);

                            let body = serial.into_bytes();
                            let mime = F::mime.as_bytes();

                            let mut buf = BytesMut::with_capacity(1 + mime.len() + body.len());
                            buf.extend_from_slice(&[mime.len() as u8]);
                            buf.extend_from_slice(mime);
                            buf.extend_from_slice(&body);
                            let payload = buf.freeze();

                            tracing::debug!(id=?request.id, bytes=?payload.len());

                            *this.in_flight = Some(InFlight::Queued {
                                id: request.id,
                                msg: Message::Binary(payload),
                            });
                            continue;
                        }
                        Err(e) => {
                            tokio::spawn(async move {
                                let _ = caller.send(Err(Error::from(e))).await;
                            });
                            return Poll::Ready(());
                        }
                    },

                    Cmd::Ping(data) => {
                        *this.in_flight = Some(InFlight::Queued {
                            id: uuid::Uuid::new_v4(),
                            msg: Message::Pong(data),
                        });
                        continue;
                    }

                    Cmd::Shutdown => {
                        this.requests.clear();
                        match this.sink.as_mut().poll_close(cx) {
                            Poll::Pending => return Poll::Pending,
                            _ => return Poll::Ready(()),
                        }
                    }
                },
            }
        }

        Poll::Pending
    }
}
