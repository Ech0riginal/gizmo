use crate::conversion::FromGValue;

use crate::process::traversal::remote::Terminator;
use crate::prelude::{
    process::traversal::GraphTraversal, GraphSON,
    GremlinClient, GremlinResult,
};

// #[derive(Clone)]
// pub struct RemoteStrategy<SD: GraphSON> {
//     client: GremlinClient<SD>,
// }
//
// impl<SD: GraphSON> RemoteStrategy<SD> {
//     pub fn new(client: GremlinClient<SD>) -> RemoteStrategy<SD> {
//         RemoteStrategy { client }
//     }
//
//     pub(crate) fn apply<S, E: FromGValue, A>(
//         &self,
//         traversal: &GraphTraversal<S, E, A>,
//     ) -> GremlinResult<RemoteTraversalIterator<SD, E>>
//     where
//         A: Terminator<E>,
//     {
//         let result = self.client.submit_traversal(traversal.bytecode())?;
//
//         Ok(RemoteTraversalIterator::new(result))
//     }
// }
