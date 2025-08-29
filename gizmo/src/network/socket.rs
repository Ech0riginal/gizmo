use core::future::Future;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use dashmap::DashMap;
use futures::StreamExt;
use gizmio::*;
use tokio::stream::Stream;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::task::JoinSet;
use tokio::{pin, pin_project};

use super::*;
use crate::client::Supports;
use crate::network::WSStream;
use crate::{Error, GremlinResult};

pub struct Socketeer<D, F> {
    pub valid: bool,
    sender: Sender<Cmd>,
    _futures: JoinSet<()>,
    _d: PhantomData<D>,
    _f: PhantomData<F>,
}

unsafe impl<D, F> Send for Socketeer<D, F> {}
unsafe impl<D, F> Sync for Socketeer<D, F> {}

impl<D, F> Socketeer<D, F>
where
    D: Dialect,
    F: Supports<D>,
    F::Serial: Send + Sync,
{
    pub fn new(wsstream: WSStream) -> Self {
        let (sink, stream) = wsstream.split();
        let (sender, receiver) = channel(16);
        let requests = Arc::new(DashMap::new());
        let mut _futures = JoinSet::new();

        _futures.spawn(ReceiverLoop::<D, F>::new(
            stream,
            requests.clone(),
            sender.clone(),
        ));
        _futures.spawn(SenderLoop::<D, F>::new(sink, requests, receiver));

        Self {
            valid: true,
            sender,
            _futures,
            _d: PhantomData::default(),
            _f: PhantomData::default(),
        }
    }

    pub async fn send(&mut self, request: Request) -> GremlinResult<Socketed> {
        tracing::debug!(id=?request.id, op=?request.op, proc=?request.proc, args=?request.args);

        let (sender, mut receiver) = channel(1);

        self.sender
            .send(Cmd::Msg((sender, request)))
            .await
            .map_err(|e| {
                self.valid = false;
                e
            })?;

        match receiver.recv().await {
            Some(Ok(response)) => Ok(Socketed::new(response, receiver)),
            Some(Err(error)) => {
                match error {
                    Error::WebsocketClone(_) | Error::Websocket(_) => {
                        self.valid = false;
                    }
                    _ => {}
                }

                tracing::debug!(?error);

                Err(error)
            }
            None => Err(Error::Closed),
        }
    }
}

#[pin_project]
pub struct Socketed {
    pub status: Status,
    buffer: VecDeque<GValue>,
    #[pin]
    receiver: Receiver<GremlinResult<Response>>,
}
impl Socketed {
    pub fn new(response: Response, receiver: Receiver<GremlinResult<Response>>) -> Socketed {
        let mut buffer = VecDeque::with_capacity(32);

        match response.data {
            GValue::List(values) => buffer.extend(values),
            gvalue => buffer.push_back(gvalue),
        }

        Socketed {
            status: response.status,
            buffer,
            receiver,
        }
    }
}

impl Stream for Socketed {
    type Item = GremlinResult<GValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            match this.buffer.pop_front() {
                Some(r) => return Poll::Ready(Some(Ok(r))),
                None => {
                    if this.status.code.i16() == 206 {
                        let fut = this.receiver.recv();
                        pin!(fut);

                        match futures::ready!(fut.poll(cx)) {
                            Some(Ok(response)) => {
                                *this.status = response.status;

                                match response.data {
                                    GValue::List(list) => this.buffer.extend(list),
                                    value => this.buffer.push_back(value),
                                }
                            }
                            Some(Err(e)) => {
                                return Poll::Ready(Some(Err(e)));
                            }
                            None => {
                                return Poll::Ready(None);
                            }
                        }
                    } else {
                        return Poll::Ready(None);
                    }
                }
            }
        }
    }
}
