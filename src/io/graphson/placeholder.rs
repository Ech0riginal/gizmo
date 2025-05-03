use super::*;
use crate::prelude::{GValue, GremlinResult, Message};

impl GraphSONDeserializer for () {
    fn deserialize(_: &serde_json::Value) -> GremlinResult<GValue> {
        todo!()
    }
}

impl GraphSONSerializer for () {
    fn serialize(_: &GValue) -> GremlinResult<serde_json::Value> {
        todo!()
    }
}

impl ContentType for () {
    fn content_type() -> &'static str {
        todo!()
    }
}

impl MessageHandler for () {
    fn message<T>(_: String, _: String, _: T, _: Option<uuid::Uuid>) -> Message<T> {
        todo!()
    }
}

/// This allows us to have 'empty' serde functionality during init
impl GraphSON for () {
    fn new() -> Self { () }
}
