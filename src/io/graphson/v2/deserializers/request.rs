use crate::Request;
use crate::io::{Deserializer, Error, V2};
use serde_json::Value;

impl Deserializer<Request> for V2 {
    fn deserialize(val: &Value) -> Result<Request, Error> {
        todo!()
    }
}
