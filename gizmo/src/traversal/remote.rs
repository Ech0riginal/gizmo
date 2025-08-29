use gizmio::{GValue, List};

pub fn traversal() -> RemoteTraversalSource {
    RemoteTraversalSource {}
}

pub struct RemoteTraversalSource {}

impl RemoteTraversalSource {
    pub fn with_remote<D: Dialect, F: Supports<D>>(
        &self,
        client: GremlinClient<D, F>,
    ) -> GraphTraversalSource<AsyncTerminator<D, F>>
    where
        <F as Format>::Serial: Send + Sync,
    {
        GraphTraversalSource::<MockTerminator>::new(MockTerminator {}).with_remote(client)
    }

    pub fn empty(&self) -> GraphTraversalSource<MockTerminator> {
        GraphTraversalSource::<MockTerminator>::new(MockTerminator {})
    }
}

#[derive(Clone)]
pub struct MockTerminator {}

impl Default for MockTerminator {
    fn default() -> Self {
        MockTerminator {}
    }
}

impl MockTerminator {
    pub fn new() -> Self {
        MockTerminator {}
    }
}

impl<T: From<GValue>> Terminator<T> for MockTerminator {
    type List = ();
    type Next = ();
    type HasNext = ();
    type Iter = ();

    fn to_list<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }

    fn next<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::Next
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }

    fn has_next<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }

    fn iter<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::Iter
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }
}
pub trait Terminator<T>: Clone {
    type List;
    type Next;
    type HasNext;
    type Iter;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>;

    fn next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Next
    where
        E: Terminator<T>;

    fn has_next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
    where
        E: Terminator<T>;

    fn iter<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Iter
    where
        E: Terminator<T>;
}

// #[derive(Clone)]
// pub struct SyncTerminator<V: GremlinIO> {
//     strategies: TraversalStrategies<SD>,
// }

// impl<V: GremlinIO> SyncTerminator<SD> {
//     pub fn new(strategies: TraversalStrategies<SD>) -> SyncTerminator<SD> {
//         SyncTerminator { strategies }
//     }
// }

// impl<V: GremlinIO, T: FromGValue> Terminator<T> for SyncTerminator<SD> {
//     type List = GremlinResult<Vec<T>>;
//     type Next = GremlinResult<Option<T>>;
//     type HasNext = GremlinResult<bool>;
//     type Iter = GremlinResult<RemoteTraversalIterator<SD, T>>;
//
//     fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
//     where
//         E: Terminator<T>,
//     {
//         self.strategies.apply(traversal)?.collect()
//     }
//
//     fn next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Next
//     where
//         E: Terminator<T>,
//     {
//         let results: GremlinResult<Vec<T>> = self.strategies.apply(traversal)?.collect();
//
//         Ok(results?.into_iter().next())
//     }
//
//     fn has_next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
//     where
//         E: Terminator<T>,
//     {
//         let results: GremlinResult<Vec<T>> = self.strategies.apply(traversal)?.collect();
//
//         Ok(results?.iter().next().is_some())
//     }
//
//     fn iter<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Iter
//     where
//         E: Terminator<T>,
//     {
//         self.strategies.apply(traversal)
//     }
// }

use futures::StreamExt;
use futures::future::{BoxFuture, FutureExt};
use gizmio::{Dialect, Format};

use crate::GremlinResult;
use crate::client::{GremlinClient, Supports};
use crate::traversal::{GraphTraversal, GraphTraversalSource, RemoteTraversalStream};

#[derive(Clone)]
pub struct AsyncTerminator<D: Dialect, F: Supports<D>>
where
    <F as Format>::Serial: Send + Sync,
{
    client: GremlinClient<D, F>,
}

impl<D: Dialect, F: Supports<D>> AsyncTerminator<D, F>
where
    <F as Format>::Serial: Send + Sync,
{
    pub fn new(client: GremlinClient<D, F>) -> AsyncTerminator<D, F> {
        AsyncTerminator { client }
    }
}

impl<D: Dialect, F: Supports<D>, T: From<GValue> + Send + 'static> Terminator<T>
    for AsyncTerminator<D, F>
where
    <F as Format>::Serial: Send + Sync,
{
    type List = BoxFuture<'static, GremlinResult<Vec<T>>>;
    type Next = BoxFuture<'static, GremlinResult<Option<T>>>;
    type HasNext = BoxFuture<'static, GremlinResult<bool>>;
    type Iter = BoxFuture<'static, GremlinResult<RemoteTraversalStream<T>>>;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        let iter = self.iter(traversal);

        async move {
            let mut stream = iter.await?;

            let mut vec = vec![];
            #[allow(irrefutable_let_patterns)]
            while let option = stream.next().await {
                if let Some(item) = option {
                    vec.push(item?);
                } else {
                    break;
                }
            }
            Ok(vec)
        }
        .boxed()
    }

    fn next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Next
    where
        E: Terminator<T>,
    {
        let iter = self.iter(traversal);

        async move {
            let mut stream = iter.await?;

            let mut vec = vec![];
            while let Some(item) = stream.next().await {
                vec.push(item?);
            }
            Ok(vec.pop())
        }
        .boxed()
    }

    fn has_next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
    where
        E: Terminator<T>,
    {
        let iter = self.iter(traversal);

        async move {
            let mut stream = iter.await?;

            let mut vec = vec![];
            while let Some(item) = stream.next().await {
                vec.push(item?);
            }
            Ok(vec.len() > 0)
        }
        .boxed()
    }

    fn iter<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Iter
    where
        E: Terminator<T>,
    {
        let client = self.client.clone();
        let bytecode = traversal.bytecode().clone();

        async move {
            let stream = client.execute(bytecode).await?;

            Ok(RemoteTraversalStream::new(stream))
        }
        .boxed()
    }
}
