use crate::Object;
use snafu::Location;
use snafu::prelude::*;

/// Extension trait to use a bit less code wrapping snafus
pub trait Ctx<A> {
    #[track_caller]
    fn ctx<T: Object>(self) -> Result<A, Leaf>;
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Leaf {
    #[snafu(display("Failed to build a {name} "))]
    Structure {
        name: &'static str,
        #[snafu(source(from(Leaf, Box::new)))]
        source: Box<Leaf>,
    },
    #[snafu(display("Invalid GraphSON"))]
    Invalid {
        #[snafu(source(from(Leaf, Box::new)))]
        source: Box<Leaf>,
    },
    #[snafu(display("Unsupported type tag"))]
    Unsupported {
        tag: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Expected {expectation}"))]
    Unexpected {
        expectation: String,
        actual: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}"))]
    Serde {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Json missing key: {key}"))]
    Missing {
        key: String,
        #[snafu(implicit)]
        location: Location,
    },
    /// This error will never actually occur and can be considered !
    Infallible,
}

impl<T, E> FromIterator<Result<T, E>> for Leaf
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

impl<A> Ctx<A> for Result<A, Leaf> {
    fn ctx<T: Object>(self) -> Result<A, Leaf> {
        self.context(StructureSnafu { name: T::name })
    }
}

impl<A> Ctx<A> for Result<A, serde_json::Error> {
    fn ctx<T: Object>(self) -> Result<A, Leaf> {
        self.context(SerdeSnafu)
            .context(StructureSnafu { name: T::name })
    }
}
