use crate::io::GraphSONDeserializer;

use crate::message::Response;
use crate::prelude::{GraphSON, GremlinClient, GremlinResult};
use crate::structure::GValue;
use futures::Stream;

use core::task::Context;
use core::task::Poll;
use futures::channel::mpsc::Receiver;
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::pin::Pin;

pin_project! {
    pub struct GResultSet<SD: GraphSON> {
        client: GremlinClient<SD>,
        results: VecDeque<GValue>,
        pub response: Response,
        #[pin]
        receiver: Receiver<GremlinResult<Response>>,
    }
}

impl<SD: GraphSON> std::fmt::Debug for GResultSet<SD> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "GResultSet {{ response: {:?}, resuls: {:?} }}",
            self.response, self.results
        )
    }
}

impl<SD: GraphSON> GResultSet<SD> {
    pub(crate) fn new(
        client: GremlinClient<SD>,
        results: VecDeque<GValue>,
        response: Response,
        receiver: Receiver<GremlinResult<Response>>,
    ) -> GResultSet<SD> {
        GResultSet {
            client,
            results,
            response,
            receiver,
        }
    }
}

impl<SD: GraphSON> Stream for GResultSet<SD>
where
    SD: GraphSONDeserializer,
{
    type Item = GremlinResult<GValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            match this.results.pop_front() {
                Some(r) => return Poll::Ready(Some(Ok(r))),
                None => {
                    if this.response.status.code == 206 {
                        match futures::ready!(this.receiver.as_mut().poll_next(cx)) {
                            Some(Ok(response)) => {
                                let results: VecDeque<GValue> =
                                    SD::deserialize(&response.result.data)?.into();

                                *this.results = results;
                                *this.response = response;
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
