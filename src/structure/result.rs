use crate::{GremlinError, GremlinResult};

use crate::structure::GValue;
use futures::Stream;

use crate::client::GremlinClient;
use crate::io::{GremlinIO, Response};
use core::task::Context;
use core::task::Poll;
use futures::channel::mpsc::Receiver;
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::pin::Pin;

pin_project! {
    pub struct GResultSet<V: GremlinIO> {
        client: GremlinClient<V>,
        results: VecDeque<GValue>,
        pub response: Response,
        #[pin]
        receiver: Receiver<GremlinResult<Response>>,
    }
}

impl<V: GremlinIO> std::fmt::Debug for GResultSet<V> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "GResultSet {{ response: {:?}, resuls: {:?} }}",
            self.response, self.results
        )
    }
}

impl<V: GremlinIO> GResultSet<V> {
    pub(crate) fn new(
        client: GremlinClient<V>,
        results: VecDeque<GValue>,
        response: Response,
        receiver: Receiver<GremlinResult<Response>>,
    ) -> GResultSet<V> {
        GResultSet {
            client,
            results,
            response,
            receiver,
        }
    }
}

impl<V: GremlinIO> Stream for GResultSet<V> {
    type Item = GremlinResult<GValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        // loop {
        //     match this.results.pop_front() {
        //         Some(r) => return Poll::Ready(Some(Ok(r))),
        //         None => {
        //             if this.response.status.code == 206 {
        //                 match futures::ready!(this.receiver.as_mut().poll_next(cx)) {
        //                     Some(Ok(response)) => {
        //                         *this.results = VecDeque::from(response.result);
        //                         *this.response = response;
        //                     }
        //                     Some(Err(e)) => {
        //                         return Poll::Ready(Some(Err(e)));
        //                     }
        //                     None => {
        //                         return Poll::Ready(None);
        //                     }
        //                 }
        //             } else {
        //                 return Poll::Ready(None);
        //             }
        //         }
        //     }
        // }
        todo!()
    }
}
