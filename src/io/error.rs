use crate::GremlinError;
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0} is not supported.")]
    Unsupported(String),
    #[error("Unexpected JSON. {msg}: {value:?}")]
    UnexpectedJson {
        msg: String,
        value: serde_json::Value,
    },
    #[error("UnexpectedGValue. {msg}: {value:?}")]
    UnexpectedGValue { msg: String, value: crate::GValue },
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
