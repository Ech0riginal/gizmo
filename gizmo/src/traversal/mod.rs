use crate::{Bytecode, GResultSet};
use crate::{GValue, GremlinResult};
use std::marker::PhantomData;

mod anonymous_traversal_source;
mod builder;
mod graph_traversal;
mod graph_traversal_source;
pub(crate) mod remote;
pub mod step;
pub mod strategies;
pub use builder::TraversalBuilder;
pub use graph_traversal::GraphTraversal;
pub use graph_traversal_source::GraphTraversalSource;
pub use remote::{AsyncTerminator, Terminator, traversal};

pub use anonymous_traversal_source::AnonymousTraversalSource;

lazy_static! {
    pub static ref WRITE_OPERATORS: Vec<&'static str> = vec![
        "addV", "property", "addE", "from", "to", "drop", "mergeV", "mergeE"
    ];
}

pub trait Traversal<S, E> {
    fn bytecode(&self) -> &Bytecode;
}

// pub struct RemoteTraversalIterator<V: GremlinIO, T: FromGValue> {
//     data: PhantomData<T>,
//     result: GResultSet<SD>,
// }

// impl<V: GremlinIO, T: FromGValue> RemoteTraversalIterator<SD, T> {
//     pub fn new(result: GResultSet<SD>) -> RemoteTraversalIterator<SD, T> {
//         RemoteTraversalIterator {
//             result,
//             data: PhantomData,
//         }
//     }
// }

// impl<V: GremlinIO> RemoteTraversalIterator<SD, crate::Null> {
//     pub fn iterate(&mut self) -> GremlinResult<()> {
//         while let Some(response) = self.next() {
//             //consume the entire iterator, returning any errors
//             response?;
//         }
//         Ok(())
//     }
// }

// impl<V: GremlinIO, T: FromGValue> Iterator for RemoteTraversalIterator<SD, T> {
//     type Item = GremlinResult<T>;
//
//     // todo remove unwrap
//     fn next(&mut self) -> Option<Self::Item> {
//         self.result
//             .next()
//             .map(|e| e.unwrap().take::<Traverser>())
//             .map(|t| t.unwrap().take::<T>())
//     }
// }

lazy_static! {
    pub static ref __: AnonymousTraversalSource = AnonymousTraversalSource::new();
}

use core::task::Context;
use core::task::Poll;
use futures::Stream;
use std::pin::Pin;

use crate::io::GremlinIO;
use pin_project_lite::pin_project;
use tokio::stream::StreamExt;

pin_project! {
    pub struct RemoteTraversalStream<V: GremlinIO, T> {
        phantom: PhantomData<T>,
        #[pin]
        stream: GResultSet<V>,
    }
}

impl<V: GremlinIO, T> RemoteTraversalStream<V, T> {
    pub fn new(stream: GResultSet<V>) -> Self {
        RemoteTraversalStream {
            phantom: PhantomData,
            stream,
        }
    }
}

impl<V: GremlinIO> RemoteTraversalStream<V, crate::Null> {
    pub async fn iterate(&mut self) -> GremlinResult<()> {
        while let Some(response) = self.stream.next().await {
            //consume the entire stream, returning any errors
            response?;
        }
        Ok(())
    }
}

impl<V: GremlinIO, T: From<GValue>> Stream for RemoteTraversalStream<V, T> {
    type Item = GremlinResult<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let item = futures::ready!(this.stream.poll_next(cx));

        Poll::Ready(item.map(|e| {
            e.expect("Failed to take an item from the result set")
                .take::<T>()
            // .expect("Failed to convert the item to a Traverser")
            // .take::<T>()
        }))
    }
}
