use serde::{Deserialize, Serialize};
use crate::Gremlin;

pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

#[cfg(test)]
mod tests;

crate::io::macros::io!(V2);

impl Gremlin for V2 {
    fn new() -> Self { V2 }

    fn mime() -> &'static str {
        "application/vnd.gremlin-v2.0+json"
    }

    fn deserialize(value: &serde_json::Value) -> crate::GremlinResult<crate::GValue> {
        de::deserialize::<Self>(value)
    }

    fn serialize(value: &crate::GValue) -> crate::GremlinResult<serde_json::Value> {
        ser::serialize::<Self>(value)
    }

    // fn message<T>(op: String, processor: String, args: T, id: Option<uuid::Uuid>) -> crate::message::Message<T> {
    //     let request_id = id.unwrap_or_else(uuid::Uuid::new_v4);
    // 
    //     crate::message::Message::V2 {
    //         request_id: crate::message::RequestIdV2 {
    //             id_type: types::UUID.to_string(),
    //             value: request_id,
    //         },
    //         op,
    //         processor,
    //         args,
    //     }
    // }
}

fn des<T: Serialize>(value: T) -> crate::GremlinResult<crate::GValue> {
}