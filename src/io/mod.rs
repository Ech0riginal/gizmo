// mod binary;

mod graphson;

mod error;

mod macros;
mod request;
mod response;

pub(crate) use macros::{expect_double, expect_float, expect_i32, expect_i64, get_value};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{GValue, GremlinError, GremlinResult};

pub use error::Error;
pub use graphson::{V2, V3, V3g};
pub use request::{Args, Request};
pub use response::{Response, Status};

#[allow(private_bounds)]
pub trait GremlinIO
where
    Self: 'static,
    Self: Send + Sync + Clone,
    Self: Deserializer<Response> + Serializer<Request>,
    Self: Deserializer<GValue> + Serializer<GValue>,
{
    fn mime() -> &'static str;
}

pub trait Deserializer<T> {
    fn deserialize(value: &serde_json::Value) -> crate::GremlinResult<T>;
}

pub trait Serializer<T> {
    fn serialize(value: &T) -> crate::GremlinResult<serde_json::Value>;
}

trait IOHelpers {
    fn get<'a>(value: &'a Value, key: &'static str) -> Result<&'a Value, GremlinError> {
        value
            .get(key)
            .ok_or(GremlinError::Json(format!("Key '{}' is missing", key)))
    }
}
