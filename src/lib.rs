#![feature(arbitrary_self_types)]
#![feature(never_type)]
#![feature(trait_alias)]
#![feature(type_changing_struct_update)]
#![feature(try_trait_v2)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(associated_type_defaults)]
#![feature(macro_metavar_expr)]
#[macro_use]
extern crate lazy_static;

mod client;
// mod conversion;
mod error;
mod io;
mod network;
mod options;
pub mod process;
pub mod structure;

pub type GremlinResult<T> = Result<T, GremlinError>;
// pub use client::GremlinClient;
pub use error::GremlinError;
pub use io::{Request, Response};
pub use structure::{GValue, Primitive};

pub(self) trait Sealed {}

pub(crate) use io::Status;

// pub mod prelude {
//     pub use super::*;
//
//     pub use tokio::stream::StreamExt;
//
//     pub use crate::io::{V2, V3, V3g};
//     pub use crate::options::*;
//     pub use crate::{edge, vertex};
//     //
//     pub use crate::process::traversal;
//     pub use crate::process::traversal::__;
//     pub use crate::process::traversal::AsyncTerminator;
//     pub use crate::process::traversal::GraphTraversalSource;
//     pub use crate::process::traversal::traversal;
//
//     pub use crate::structure::*;
// }
