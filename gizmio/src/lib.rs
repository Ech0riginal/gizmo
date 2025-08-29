#![feature(breakpoint)]
#![feature(impl_trait_in_assoc_type)]
#![feature(associated_type_defaults)]

mod api;
pub mod dialects;
pub mod formats;
pub(crate) mod macros;
mod request;
mod response;
pub mod types;

pub(crate) use api::*;
#[allow(unused_imports)]
pub(crate) use dialects::{Janus, SQLg};
pub use types::*;

pub use crate::api::Error;
pub use crate::api::{Bytable, Dialect, Format};
pub use crate::api::{DeserializeExt, Deserializer};
pub use crate::api::{SerializeExt, Serializer};
pub use crate::api::{V1, V2, V3};
pub use crate::request::{Args, Request};
pub use crate::response::{Response, Status};
