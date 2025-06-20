use crate::Object;
use snafu::Location;
use snafu::prelude::*;

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

impl Error {
    #[track_caller]
    pub fn unexpected<T: std::fmt::Debug + ?Sized, M: AsRef<str>>(
        value: &T,
        expectation: M,
    ) -> Self {
        Self::Unexpected {
            actual: format!("{:?}", value),
            expectation: expectation.as_ref().to_string(),
            location: {
                let l = std::panic::Location::caller();
                Location::new(l.file(), l.line(), l.column())
            },
        }
    }
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
