#![feature(impl_trait_in_assoc_type)]

mod api;
mod error;
mod graphson;
mod macros;
mod request;
mod response;
mod types;

pub use api::GremlinIO;
pub use error::Error;
pub use graphson::V3;
pub use request::{Args, Request};
pub use response::{Response, Status};
pub use types::*;

pub(crate) use api::Sealed;
