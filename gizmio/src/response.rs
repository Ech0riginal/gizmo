use std::fmt::Formatter;
use std::hash::Hasher;
use std::ops::Deref;

use crate::{Either2, GValue, Map, Named};

#[derive(Debug)]
pub struct Response {
    pub id: uuid::Uuid,
    pub data: GValue,
    pub status: Status,
    pub meta: Map<String, GValue>,
}

impl Named for Response {
    const name: &'static str = "Response";
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

pub enum Code {
    Raw(i16),
    Http(http::StatusCode),
}

impl From<i16> for Code {
    fn from(value: i16) -> Self {
        http::StatusCode::from_u16(value as u16)
            .map(Code::Http)
            .unwrap_or(Code::Raw(value))
    }
}

impl Code {
    pub fn i16(&self) -> i16 {
        match self {
            Code::Raw(i) => *i,
            Code::Http(status) => status.as_u16() as i16,
        }
    }
}

impl std::fmt::Debug for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Code::Raw(n) => {
                write!(f, "{}", n)
            }
            Code::Http(stat) => {
                write!(f, "{}", stat.as_u16())?;
                if let Some(reason) = stat.canonical_reason() {
                    write!(f, "({})", reason)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct Status {
    pub code: Code,
    pub message: Option<String>,
    pub attributes: serde_json::Value,
}

impl Named for Status {
    const name: &'static str = "Status";
}
