use super::*;
use snafu::ResultExt;

pub trait Deserializer<O, S, D> {
    fn deserialize(serial: &S) -> Result<O, Error>;
}

pub trait DeserializeExt: Sized {
    fn deserialize<F, D, T>(&self) -> Result<T, Error>
    where
        F: Format,
        F: Deserializer<T, Self, D>,
        D: Dialect,
        T: Object,
    {
        let result = F::deserialize(self).context(ObjectSnafu { name: T::name });

        #[cfg(feature = "tracing")]
        match result.as_ref() {
            Ok(value) => tracing::trace!("{:?}", value),
            Err(e) => tracing::trace!("{:?}", e),
        }

        result
    }
}
