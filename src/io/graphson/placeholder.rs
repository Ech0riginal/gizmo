//! A virtual placeholder for when we initialize the client.

use crate::GValue;
use crate::io::{Deserializer, Error, GremlinIO, Request, Response, Serializer};
use crate::structure::GID;
use serde_json::Value;

impl GremlinIO for () {
    const version: &'static str = "n/a";

    fn mime() -> &'static str {
        "none"
    }
}

impl Deserializer<Response> for () {
    fn deserialize(_: &Value) -> Result<Response, Error> {
        Err(Error::Unsupported("()".into()))
    }
}

impl Deserializer<GValue> for () {
    fn deserialize(_: &Value) -> Result<GValue, Error> {
        Err(Error::Unsupported("()".into()))
    }
}

impl Deserializer<GID> for () {
    fn deserialize(_: &Value) -> Result<GID, Error> {
        Err(Error::Unsupported("()".into()))
    }
}

impl Serializer<Request> for () {
    fn serialize(_: &Request) -> Result<Value, Error> {
        Err(Error::Unsupported("()".into()))
    }
}

impl Serializer<GValue> for () {
    fn serialize(_: &GValue) -> Result<Value, Error> {
        Err(Error::Unsupported("()".into()))
    }
}

impl Serializer<GID> for () {
    fn serialize(_: &GID) -> Result<Value, Error> {
        Err(Error::Unsupported("()".into()))
    }
}
