//! A virtual placeholder for when we initialize the client.

use serde_json::Value;
use uuid::Uuid;
use crate::{GValue, Gremlin, GremlinResult};
// use crate::message::Message;

// impl Gremlin for () {
//     fn mime() -> &'static str {
//         todo!()
//     }
// 
//     fn deserialize(value: &Value) -> GremlinResult<GValue> {
//         todo!()
//     }
// 
//     fn serialize(value: &GValue) -> GremlinResult<Value> {
//         todo!()
//     }
// 
//     fn message<T>(op: String, processor: String, args: T, id: Option<Uuid>) -> Message<T> {
//         todo!()
//     }
// }