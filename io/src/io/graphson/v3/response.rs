use crate::io::graphson::prelude::*;
use crate::io::{Response, Status};

use serde_json::Value;

impl Deserializer<Response> for V3 {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        <V2 as Deserializer<Response>>::deserialize(value)
    }
}

impl Deserializer<Status> for V3 {
    fn deserialize(val: &Value) -> Result<Status, Error> {
        <V2 as Deserializer<Status>>::deserialize(val)
    }
}

impl Serializer<Response> for V3 {
    fn serialize(val: &Response) -> Result<Value, Error> {
        <V2 as Serializer<Response>>::serialize(val)
    }
}

impl Serializer<Status> for V3 {
    fn serialize(val: &Status) -> Result<Value, Error> {
        <V2 as Serializer<Status>>::serialize(val)
    }
}
