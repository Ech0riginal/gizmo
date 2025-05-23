use crate::process::traversal::{GraphTraversal, GraphTraversalSource};
use crate::{GValue, GremlinResult};

pub fn traversal() -> RemoteTraversalSource {
    RemoteTraversalSource {}
}

pub struct RemoteTraversalSource {}

impl RemoteTraversalSource {
    pub fn with_remote<V: GremlinIO>(
        &self,
        client: GremlinClient<V>,
    ) -> GraphTraversalSource<AsyncTerminator<V>> {
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

use crate::client::GremlinClient;
use crate::io::GremlinIO;
use crate::network::GremlinStream;
use futures::StreamExt;
use futures::future::{BoxFuture, FutureExt};

#[derive(Clone)]
pub struct AsyncTerminator<V: GremlinIO> {
    client: GremlinClient<V>,
}

impl<V: GremlinIO> AsyncTerminator<V> {
    pub fn new(client: GremlinClient<V>) -> AsyncTerminator<V> {
        AsyncTerminator { client }
    }
}

impl<V: GremlinIO, T: From<GValue> + Send + 'static> Terminator<T> for AsyncTerminator<V> {
    type List = BoxFuture<'static, GremlinResult<Vec<T>>>;
    type Next = BoxFuture<'static, GremlinResult<Option<T>>>;
    type HasNext = BoxFuture<'static, GremlinResult<bool>>;
    type Iter = BoxFuture<'static, GremlinResult<impl GremlinStream>>;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        let iter = self.iter(traversal);

        async move {
            let mut stream = iter.await?;

            let mut vec = vec![];
            #[allow(irrefutable_let_patterns)]
            while let Some(Some(result)) = stream.next().await {
                vec.push(result?.into());
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
            while let Some(Some(item)) = stream.next().await {
                vec.push(item?.into());
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
            while let Some(Some(item)) = stream.next().await {
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
        Box::pin(async move {
            let stream = client.execute(bytecode).await?;
            Ok(stream)
        })
    }
}
