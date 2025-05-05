// mod binary;

mod graphson;

mod error;

mod macros;
mod message;

use serde::{Deserialize, Serialize};
pub(crate) use macros::{get_value, expect_i32, expect_i64, expect_float, expect_double};

pub use error::Error;
pub use graphson::{V2, V3, V3g};
pub use message::GremlinMessage;
use crate::{GValue, GremlinResult};

#[allow(private_bounds)]
pub trait GremlinIO:
    Send
    + Sync
    + Clone
    + std::fmt::Debug
    + Default
    + 'static
{
    type Version: GremlinWrangler;

    fn new() -> Self::Version;

    fn mime() -> &'static str {
        Self::Version::mime()
    }


    fn deserialize<T>(value: T) -> crate::GremlinResult<crate::GValue>
    where
        T: GremlinDeserializer
    {
        Self::Version::deserialize(value)
    }

    fn serialize<T>(value: T) -> crate::GremlinResult<serde_json::Value>
    where
        T: GremlinSerializer
    {
        Self::Version::serialize(value)
    }
}

impl GremlinDeserializer for serde_json::Value {
    fn deserialize<T>(value: T) -> GremlinResult<GValue>
    where
        T: for<'de> Deserialize<'de>
    {
        todo!()
    }
}


trait GremlinWrangler:
GremlinDeserializer +
GremlinSerializer +
GremlinMessage +
Mime
{}

trait GremlinDeserializer {
    fn deserialize<T>(value: T) -> crate::GremlinResult<crate::GValue>
    where
        T: for<'de> serde::Deserialize<'de>;
}
trait GremlinSerializer {
    fn serialize<T>(value: T) -> crate::GremlinResult<serde_json::Value>
    where
        T: serde::Serialize;
}
trait Mime {
    fn mime() -> &'static str;
}


pub trait Gremlin:
    Send
    + Sync
    + Clone
    + std::fmt::Debug
    + Default
    + 'static
{
    fn new() -> Self;

    fn mime() -> &'static str;

    fn deserialize<T>(value: &serde_json::Value) -> crate::GremlinResult<crate::GValue>
        where
            T: for<'de> serde::Deserialize<'de>;

    fn serialize<T>(value: T) -> crate::GremlinResult<serde_json::Value>
        where
            T: serde::Serialize;
}
