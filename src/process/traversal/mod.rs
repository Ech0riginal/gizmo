use crate::conversion::FromGValue;
use crate::prelude::{GResultSet, GraphSON, GremlinResult};
use crate::structure::Traverser;
use std::marker::PhantomData;

mod anonymous_traversal_source;
mod builder;
pub(crate) mod bytecode;
mod graph_traversal;
mod graph_traversal_source;
mod order;
pub(crate) mod remote;
mod scope;
pub mod step;
pub mod strategies;
pub use builder::TraversalBuilder;
pub use bytecode::{Bytecode, WRITE_OPERATORS};
pub use graph_traversal::GraphTraversal;
pub use graph_traversal_source::GraphTraversalSource;
pub use order::Order;
pub use remote::{AsyncTerminator, Terminator, traversal};
pub use scope::Scope;

pub use anonymous_traversal_source::AnonymousTraversalSource;

use lazy_static::lazy_static;

// use step::*;

pub trait Traversal<S, E> {
    fn bytecode(&self) -> &Bytecode;
}

// pub struct RemoteTraversalIterator<SD: GraphSON, T: FromGValue> {
//     data: PhantomData<T>,
//     result: GResultSet<SD>,
// }

// impl<SD: GraphSON, T: FromGValue> RemoteTraversalIterator<SD, T> {
//     pub fn new(result: GResultSet<SD>) -> RemoteTraversalIterator<SD, T> {
//         RemoteTraversalIterator {
//             result,
//             data: PhantomData,
//         }
//     }
// }

// impl<SD: GraphSON> RemoteTraversalIterator<SD, crate::structure::Null> {
//     pub fn iterate(&mut self) -> GremlinResult<()> {
//         while let Some(response) = self.next() {
//             //consume the entire iterator, returning any errors
//             response?;
//         }
//         Ok(())
//     }
// }

// impl<SD: GraphSON, T: FromGValue> Iterator for RemoteTraversalIterator<SD, T> {
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

use pin_project_lite::pin_project;
use tokio_stream::StreamExt;

pin_project! {
    pub struct RemoteTraversalStream<SD: GraphSON, T> {
        phantom: PhantomData<T>,
        #[pin]
        stream: GResultSet<SD>,
    }
}

impl<SD: GraphSON, T> RemoteTraversalStream<SD, T> {
    pub fn new(stream: GResultSet<SD>) -> Self {
        RemoteTraversalStream {
            phantom: PhantomData,
            stream,
        }
    }
}

impl<SD: GraphSON> RemoteTraversalStream<SD, crate::structure::Null> {
    pub async fn iterate(&mut self) -> GremlinResult<()> {
        while let Some(response) = self.next().await {
            //consume the entire stream, returning any errors
            response?;
        }
        Ok(())
    }
}

impl<SD: GraphSON, T: FromGValue> Stream for RemoteTraversalStream<SD, T> {
    type Item = GremlinResult<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let item = futures::ready!(this.stream.poll_next(cx));

        Poll::Ready(item.map(|e| {
            e.expect("Failed to take an item from the result set")
                .take::<Traverser>()
                .expect("Failed to convert the item to a Traverser")
                .take::<T>()
        }))
    }
}
