use crate::GValue;
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Response {
    pub id: uuid::Uuid,
    pub data: GValue,
    pub status: Status,
    pub meta: HashMap<String, GValue>,
}

impl Eq for Response {}
impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl std::hash::Hash for Response {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
pub struct Status {
    pub code: i16,
    pub message: Option<String>,
    pub attributes: serde_json::Value,
}
