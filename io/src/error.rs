use crate::Object;
use snafu::Location;
use snafu::prelude::*;

/// Extension trait to attach an Object's name to an error
pub trait Obj<A> {
    #[track_caller]
    fn ctx<T: Object>(self) -> Result<A, Error>;
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to build a {name}"))]
    Object {
        name: &'static str,
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
    #[snafu(display("Unsupported type tag '{tag}'"))]
    Unsupported {
        tag: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Expected {expectation}, got '{actual}'"))]
    Unexpected {
        actual: String,
        expectation: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Json missing key: {key}"))]
    Missing {
        key: String,
        #[snafu(implicit)]
        location: Location,
    },

    /****************************************** Wrappers ******************************************/
    #[snafu(display("{source}"))]
    Serde {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}"))]
    Uuid {
        source: uuid::Error,
        #[snafu(implicit)]
        location: Location,
    },

    /// This error will never actually occur and should be considered !
    Infallible,
}

impl From<uuid::Error> for Error {
    #[track_caller]
    fn from(value: uuid::Error) -> Self {
        let caller = std::panic::Location::caller();
        Error::Uuid {
            source: value,
            location: Location::new(caller.file(), caller.line(), caller.column()),
        }
    }
}

impl From<serde_json::Error> for Error {
    #[track_caller]
    fn from(value: serde_json::Error) -> Self {
        let caller = std::panic::Location::caller();
        Error::Serde {
            source: value,
            location: Location::new(caller.file(), caller.line(), caller.column()),
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
