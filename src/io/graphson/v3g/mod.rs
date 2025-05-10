// TODO move this into Hedwig for proper separation
// you wrote it here bc it's where we're working but, ya know, move it sometime :)

use crate::io::{Deserializer, Error, GremlinIO, Request, Response, Serializer, V3};
use crate::{GValue, GremlinResult};
use serde_json::Value;

pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

crate::io::macros::io!(V3g);

impl GremlinIO for V3g {
    const version: &'static str = V3::version;
    fn mime() -> &'static str {
        V3::mime()
    }
}

impl Deserializer<Response> for V3g {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        todo!()
    }
}

impl Deserializer<GValue> for V3g {
    fn deserialize(value: &Value) -> Result<GValue, Error> {
        todo!()
    }
}

impl Serializer<Request> for V3g {
    fn serialize(value: &Request) -> Result<Value, Error> {
        todo!()
    }
}

impl Serializer<GValue> for V3g {
    fn serialize(value: &GValue) -> Result<Value, Error> {
        todo!()
    }
}

// impl Gremlin for V3g {
//     fn mime() -> &'static str {
//         V3::mime()
//     }
//
//     fn deserialize(value: &Value) -> Result<crate::GValue, Error> {
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
//     fn serialize(value: &crate::GValue) -> Result<Value, Error> {
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
