// TODO move this into Hedwig for proper separation
// you wrote it here bc it's where we're working but, ya know, move it sometime :)

use crate::io::{Deserializer, GremlinIO, Serializer, V3};
use crate::message::{Request, Response};
use crate::{GValue, GremlinResult};
use serde_json::Value;

pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

crate::io::macros::io!(V3g);

impl GremlinIO for V3g {
    fn mime() -> &'static str {
        V3::mime()
    }
}

impl Deserializer<Response> for V3g {
    fn deserialize(value: &Value) -> GremlinResult<Response> {
        todo!()
    }
}

impl Deserializer<GValue> for V3g {
    fn deserialize(value: &Value) -> GremlinResult<GValue> {
        todo!()
    }
}

impl Serializer<Request> for V3g {
    fn serialize(value: &Request) -> GremlinResult<Value> {
        todo!()
    }
}

impl Serializer<GValue> for V3g {
    fn serialize(value: &GValue) -> GremlinResult<Value> {
        todo!()
    }
}

// impl Gremlin for V3g {
//     fn mime() -> &'static str {
//         V3::mime()
//     }
//
//     fn deserialize(value: &Value) -> crate::GremlinResult<crate::GValue> {
//         match value {
//             Value::Object(_) => {
//                 let _type = match &value["@type"] {
//                     Value::String(e) => Ok(e),
//                     _type => Err(crate::GremlinError::Json(format!("Unexpected type: {:?}", _type))),
//                 }?;
//
//                 match _type.as_str() {
//                     types::G_GEOMETRY | types::G_GEOSHAPE => de::geometry(value),
//                     _ => V3::deserialize(value),
//                 }
//             }
//             _ => V3::deserialize(value),
//         }
//     }
//
//     fn serialize(value: &crate::GValue) -> crate::GremlinResult<Value> {
//         match value {
//             crate::GValue::Geometry(_) => ser::geometry(value),
//             _ => V3::serialize(value),
//         }
//     }
//
//     fn message<T>(op: String, processor: String, args: T, id: Option<Uuid>) -> crate::message::Message<T> {
//         V3::message(op, processor, args, id)
//     }
// }
