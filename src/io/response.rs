use crate::GValue;
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Response {
    pub id: uuid::Uuid,
    pub result: GResult,
    pub status: Status,
}

#[derive(Debug)]
pub struct GResult {
    pub(crate) data: GValue,
    pub(crate) meta: HashMap<String, GValue>,
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

macro_rules! from_int {
    ($int:ty) => {
        impl From<$int> for Status {
            fn from(value: $int) -> Self {
                Self {
                    code: value as i16,
                    message: Default::default(),
                    attributes: HashMap::new(),
                }
            }
        }
    };
}
