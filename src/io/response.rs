#[derive(Debug)]
pub struct Response {
    pub id: uuid::Uuid,
    pub result: crate::GValue,
    pub status: Status,
}

#[derive(Debug)]
pub struct Status {
    pub code: i16,
    pub message: Option<String>,
}
