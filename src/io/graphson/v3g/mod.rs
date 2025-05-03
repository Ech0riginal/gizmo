// TODO move this into Hedwig for proper separation
// you wrote it here bc it's where we're working but, ya know, move it sometime :)

use serde_json::Value;
use uuid::Uuid;
use crate::io::{Gremlin, V3};

pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

crate::io::macros::io!(V3g);

impl Gremlin for V3g {
    fn mime() -> &'static str {
        V3::mime()
    }

    fn deserialize(value: &Value) -> crate::GremlinResult<crate::GValue> {
        match value {
            Value::Object(_) => {
                let _type = match &value["@type"] {
                    Value::String(e) => Ok(e),
                    _type => Err(crate::GremlinError::Json(format!("Unexpected type: {:?}", _type))),
                }?;

                match _type.as_str() {
                    types::G_GEOMETRY | types::G_GEOSHAPE => de::geometry(value),
                    _ => V3::deserialize(value),
                }
            }
            _ => V3::deserialize(value),
        }
    }

    fn serialize(value: &crate::GValue) -> crate::GremlinResult<Value> {
        match value {
            crate::GValue::Geometry(_) => ser::geometry(value),
            _ => V3::serialize(value),
        }
    }

    fn message<T>(op: String, processor: String, args: T, id: Option<Uuid>) -> crate::message::Message<T> {
        V3::message(op, processor, args, id)
    }
}
