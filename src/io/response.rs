use std::hash::Hasher;

#[derive(Debug)]
pub struct Response {
    pub id: uuid::Uuid,
    pub result: crate::GValue,
    pub status: Status,
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
}
