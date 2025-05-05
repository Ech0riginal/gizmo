use crate::GValue;
use derive_builder::Builder;
use serde::{Deserialize as SerdeDeserialize, Deserializer, Serialize};
use serde_derive::Deserialize;
use serde_json::{Error, Value};
use uuid::Uuid;

#[derive(Debug, Builder, Serialize)]
#[builder(pattern = "mutable")]
pub struct Request {
    pub(crate) id: Uuid,
    pub(crate) op: &'static str,
    pub(crate) proc: &'static str,
    #[builder(setter(custom))]
    pub(crate) args: Value,
}

impl RequestBuilder {
    fn args<T: Serialize>(mut self, value: &T) -> Self {
        match serde_json::to_value(value) {
            Ok(value) => {
                self.args = Some(value);
            }
            Err(error) => panic!("Error serializing message arguments ({})", error),
        }

        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestIdV2 {
    #[serde(rename = "@type")]
    pub(crate) id_type: String,

    #[serde(rename = "@value")]
    pub(crate) value: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Message<T> {
    #[serde(rename_all = "camelCase")]
    V1 {
        request_id: Uuid,
        op: String,
        processor: String,
        args: T,
    },
    #[serde(rename_all = "camelCase")]
    V2 {
        request_id: RequestIdV2,
        op: String,
        processor: String,
        args: T,
    },
    #[serde(rename_all = "camelCase")]
    V3 {
        request_id: Uuid,
        op: String,
        processor: String,
        args: T,
    },
}

#[derive(Debug)]
pub struct Response {
    pub id: Uuid,
    pub result: GValue,
    pub status: Status,
}

#[derive(Debug)]
pub struct Status {
    pub code: i16,
    pub message: Option<String>,
}
