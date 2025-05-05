use crate::conversion::FromGValue;
use crate::process::traversal::{GraphTraversal, GraphTraversalSource};

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

impl<T: FromGValue> Terminator<T> for MockTerminator {
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
pub trait Terminator<T: FromGValue>: Clone {
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

use crate::GremlinResult;
use crate::client::GremlinClient;
use crate::io::GremlinIO;
use crate::process::traversal::RemoteTraversalStream;
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

impl<V: GremlinIO, T: FromGValue + Send + 'static> Terminator<T> for AsyncTerminator<V> {
    type List = BoxFuture<'static, GremlinResult<Vec<T>>>;
    type Next = BoxFuture<'static, GremlinResult<Option<T>>>;
    type HasNext = BoxFuture<'static, GremlinResult<bool>>;
    type Iter = BoxFuture<'static, GremlinResult<RemoteTraversalStream<V, T>>>;

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
            // let stream = client.submit_traversal(&bytecode).await?;
            //
            // Ok(RemoteTraversalStream::new(stream))
            todo!()
        }
        .boxed()
    }
}
