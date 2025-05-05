use crate::{GValue, GremlinResult};
// use crate::prelude::{GResultSet};
use crate::structure::{GResultSet, Traverser};
use futures::FutureExt;
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

pub struct RemoteTraversalIterator<V: GremlinIO, T: From<GValue>> {
    data: PhantomData<T>,
    result: GResultSet<V>,
}

impl<V: GremlinIO, T: From<GValue>> RemoteTraversalIterator<V, T> {
    pub fn new(result: GResultSet<V>) -> RemoteTraversalIterator<V, T> {
        RemoteTraversalIterator {
            result,
            data: PhantomData,
        }
    }
}

impl<V: GremlinIO> RemoteTraversalIterator<V, crate::structure::Null> {
    pub fn iterate(&mut self) -> GremlinResult<()> {
        while let Some(response) = self.next() {
            //consume the entire iterator, returning any errors
            response?;
        }
        Ok(())
    }
}

impl<V: GremlinIO, T: From<GValue>> Iterator for RemoteTraversalIterator<V, T> {
    type Item = GremlinResult<T>;

    // todo remove unwrap
    fn next(&mut self) -> Option<Self::Item> {
        // self.result
        //     .next()
        //     .map(|e| e.take::<Traverser>())
        //     .map(|t| t.unwrap().take::<T>())
        todo!()
    }
}

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

impl<V: GremlinIO> RemoteTraversalStream<V, crate::structure::Null> {
    pub async fn iterate(&mut self) -> GremlinResult<()> {
        while let Some(response) = self.next().await {
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
                .take::<Traverser>()
                .expect("Failed to convert the item to a Traverser")
                .take::<T>()
        }))
    }
}
