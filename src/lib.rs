#![feature(never_type)]


#![feature(trait_alias)]
#![feature(type_changing_struct_update)]
#![feature(try_trait_v2)]
#[macro_use]
extern crate lazy_static;

mod conversion;
mod error;
mod io;
mod message;

mod pool;

mod client;

mod connection;

mod options;

pub mod process;
pub mod structure;
pub mod utils;


pub type GremlinResult<T> = Result<T, GremlinError>;
pub use error::GremlinError;
pub use structure::GValue;
pub use io::Gremlin;

pub mod prelude {
    pub use super::*;

    pub use tokio::stream::StreamExt;

    pub use crate::client::GremlinClient;
    pub use crate::io::{V2, V3, V3g};
    pub use crate::options::*;
    pub use crate::{edge, vertex};

    pub use crate::process::traversal;
    pub use crate::process::traversal::__;
    pub use crate::process::traversal::AsyncTerminator;
    pub use crate::process::traversal::GraphTraversalSource;
    pub use crate::process::traversal::traversal;

    pub use crate::conversion::{BorrowFromGValue, FromGValue, ToGValue};
    pub(crate) use crate::message::Message;
    pub use crate::structure::*;
}

