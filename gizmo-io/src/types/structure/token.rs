use crate::{obj, tag};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Token {
    value: String,
}

obj!(Token);
tag!(Token);

impl Token {
    pub fn new<T>(value: T) -> Token
    where
        T: Into<String>,
    {
        Token {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
