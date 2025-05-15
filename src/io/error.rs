use crate::{GValue, GremlinError};
use std::fmt::Debug;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0} is not supported.")]
    Unsupported(String),
    #[error("{0} was not expected.")]
    Unexpected(String),
    #[error("Unexpected JSON. {msg}: {value:?}")]
    UnexpectedJson {
        msg: String,
        value: serde_json::Value,
    },
    #[error("UnexpectedGValue. {msg}: {value:?}")]
    UnexpectedGValue {
        msg: String,
        value: crate::GValue
    },
    #[error("Value is missing key '{0}'")]
    Missing(&'static str),
    #[error("Cannot cast {0} to {1}")]
    Cast(String, String),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error("do what")]
    Huh,
}

impl From<GremlinError> for Error {
    fn from(value: GremlinError) -> Self {
        match value {
            GremlinError::Cast(from_, to_) => Self::Cast(from_, to_),
            _ => panic!("unhandled gremlin in the pipes!"),
        }
    }
}

impl<T, E> FromIterator<Result<T, E>> for Error
where
    Self: From<E>,
{
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Self {
        iter.into_iter()
            .filter(|result| result.is_err())
            .take(1)
            .collect()
    }
}


impl Error {
    pub(crate) fn unexpected<V, S>(val: V, msg: S) -> Self
        where
            Self: Expect<V>,
            S: AsRef<str>,
    {
        Self::expect(val, msg.as_ref())
    }
}

trait Expect<T> {
    fn expect(value: T, msg: &str) -> Error;
}

impl Expect<Value> for Error {
    fn expect(val: Value, msg: &str) -> Error {
        Self::UnexpectedJson {
            msg: msg.to_string(),
            value: val,
        }
    }
}


impl Expect<&Value> for Error {
    fn expect(val: &Value, msg: &str) -> Error {
        Self::UnexpectedJson {
            msg: msg.to_string(),
            value: val.clone(),
        }
    }
}

impl Expect<&GValue> for Error {
    fn expect(val: &GValue, msg: &str) -> Error {
        Self::UnexpectedGValue {
            msg: msg.to_string(),
            value: val.clone(),
        }
    }
}