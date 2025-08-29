use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::GremlinResult;

mod anonymous_traversal_source;
mod builder;
mod graph_traversal;
mod graph_traversal_source;
mod predicates;
pub(crate) mod remote;
pub mod step;
pub mod strategies;

pub use anonymous_traversal_source::AnonymousTraversalSource;
pub use builder::TraversalBuilder;
pub use graph_traversal::GraphTraversal;
pub use graph_traversal_source::GraphTraversalSource;
pub use remote::{AsyncTerminator, Terminator, traversal};

lazy_static! {
    pub static ref WRITE_OPERATORS: Vec<&'static str> = vec![
        "addV", "property", "addE", "from", "to", "drop", "mergeV", "mergeE"
    ];
}

pub trait Traversal<S, E> {
    fn bytecode(&self) -> &Bytecode;
}

lazy_static! {
    pub static ref __: AnonymousTraversalSource = AnonymousTraversalSource::new();
}

use futures::Stream;
use gizmio::types::{Bytecode, GValue};
use gizmio::{Null, Traverser};
use pin_project_lite::pin_project;
use tokio::stream::StreamExt;

use crate::network::Socketed;

pin_project! {
    pub struct RemoteTraversalStream<T> {
        #[pin]
        stream: Socketed,
        _t: PhantomData<T>,
    }
}

impl<T> RemoteTraversalStream<T> {
    pub fn new(stream: Socketed) -> Self {
        RemoteTraversalStream {
            stream,
            _t: PhantomData::default(),
        }
    }
}

impl RemoteTraversalStream<Null> {
    pub async fn iterate(&mut self) -> GremlinResult<()> {
        while let Some(response) = self.stream.next().await {
            //consume the entire stream, returning any errors
            response?;
        }
        Ok(())
    }
}

impl<T: From<GValue>> Stream for RemoteTraversalStream<T> {
    type Item = GremlinResult<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let item = futures::ready!(this.stream.poll_next(cx));

        Poll::Ready(item.map(|e| {
            e.expect("Failed to take an item from the result set")
                .take::<Traverser>()
                .expect("Failed to convert the item to a Traverser")
                .take::<T>()
                // .into_iter()
                // .map(|t| t.take::<T>())
                // .collect::<GremlinResult<List<T>>>()
                .map_err(crate::Error::from)
        }))
    }
}
