mod v2;
mod v3;
mod v3g;

mod placeholder;
#[cfg(test)]
pub(self) mod tests;

pub use v2::V2;
pub use v3::V3;
pub use v3g::V3g;

use crate::prelude::{GValue, GremlinResult, Message};

/// Encompasses GraphSON v2, v3, and our custom types. Our custom types will always be the last in
/// this list.
#[allow(unused)]
pub(crate) mod types {
    pub(crate) use super::v2::types as v2;
    pub(crate) use super::v3::types as v3;
    pub(crate) use super::v3g::types as v3g;
}

pub trait GraphSON:
    GraphSONDeserializer
    + GraphSONSerializer
    + ContentType
    + MessageHandler
    + Send
    + Sync
    + Clone
    + std::fmt::Debug
    + Default
    + 'static
{
    fn new() -> Self;
}

// Wow, I hate this
pub(crate) struct Ctx {
    in_property: bool,
}

pub trait GraphSONDeserializer {
    fn deserialize(value: &serde_json::Value) -> GremlinResult<GValue>;
}

pub trait GraphSONSerializer {
    fn serialize(value: &GValue) -> GremlinResult<serde_json::Value>;
}

pub trait ContentType {
    fn content_type() -> &'static str;
}

pub trait MessageHandler {
    fn message<T>(op: String, processor: String, args: T, id: Option<uuid::Uuid>) -> Message<T>;
}
