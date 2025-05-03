use serde_json::Value;
use uuid::Uuid;

pub trait Message {
    fn request_id(&self) -> Uuid;
    fn op (&self) -> String;
    fn processor(&self) -> String;
    fn args(&self) -> Value;
}