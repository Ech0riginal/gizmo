
pub trait GremlinMessage {
    fn request_id(&self) -> uuid::Uuid;
    fn op (&self) -> String;
    fn processor(&self) -> String;
    fn args(&self) -> serde_json::Value;
}