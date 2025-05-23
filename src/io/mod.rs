// mod binary;

mod graphson;

mod error;

mod macros;
mod request;
mod response;
mod serde;

pub(crate) use macros::*;
pub(crate) use request::*;
pub(crate) use response::*;

pub use error::Error;
pub use graphson::V2; //, V3, V3g};
pub use request::{Args, Request};
pub use response::{Response, Status};
pub use serde::{Deserialize, Serialize};

use crate::structure::{GID, GValue};
use serde_json::Value;

#[allow(private_bounds)]
pub trait GremlinIO
where
    Self: 'static,
    Self: Send + Sync + Clone,
    Self: Deserializer<Response> + Serializer<Request>,
    Self: Deserializer<GValue> + Serializer<GValue>,
    Self: Deserializer<GID> + Serializer<GID>,
{
    #[allow(nonstandard_style)]
    const version: &'static str;

    fn mime() -> &'static str;
}

pub trait Deserializer<T> {
    fn deserialize(val: &serde_json::Value) -> Result<T, Error>;
}

pub trait Serializer<T> {
    fn serialize(val: &T) -> Result<serde_json::Value, Error>;
}

trait IOHelpers {
    fn get<'a>(value: &'a Value, key: &'static str) -> Result<&'a Value, Error> {
        value.get(key).ok_or(Error::Missing(key))
    }
}
