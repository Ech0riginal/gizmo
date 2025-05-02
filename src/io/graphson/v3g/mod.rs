// TODO move this into Hedwig for proper separation
// you wrote it here bc it's where we're working but, ya know, move it sometime :)

use crate::io::graphson::{ContentType, GraphSONDeserializer, GraphSONSerializer, MessageHandler};
use crate::prelude::*;
use serde_json::Value;
use uuid::Uuid;

use crate::prelude::{GValue, GremlinResult, Message};

pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

graphson_io!(V3g);

impl GraphSONDeserializer for V3g {
    fn deserialize(value: &Value) -> GremlinResult<GValue> {
        match value {
            Value::Object(_) => {
                let _type = match &value["@type"] {
                    Value::String(e) => Ok(e),
                    _type => Err(GremlinError::Json(format!("Unexpected type: {:?}", _type))),
                }?;

                match _type.as_str() {
                    types::G_GEOMETRY | types::G_GEOSHAPE => de::geometry(value),
                    _ => super::V3::deserialize(value),
                }
            }
            _ => super::V3::deserialize(value),
        }
    }
}

impl GraphSONSerializer for V3g {
    fn serialize(value: &GValue) -> GremlinResult<Value> {
        match value {
            GValue::Geometry(_) => ser::geometry(value),
            _ => super::V3::serialize(value),
        }
    }
}

impl MessageHandler for V3g {
    fn message<T>(op: String, processor: String, args: T, id: Option<Uuid>) -> Message<T> {
        super::V3::message(op, processor, args, id)
    }
}

impl ContentType for V3g {
    fn content_type() -> &'static str {
        super::V3::content_type()
    }
}
