use super::*;
use snafu::ResultExt;

pub trait Serializer<O, S, D> {
    fn serialize(object: &O) -> Result<S, Error>;
}

pub trait SerializeExt: Sized {
    fn serialize<F, D>(&self) -> Result<F::Serial, Error>
    where
        F: Format,
        F: Serializer<Self, F::Serial, D>,
        D: Dialect,
        Self: Named,
    {
        let result = F::serialize(self).context(ObjectSnafu { name: Self::name });

        #[cfg(feature = "tracing")]
        match result.as_ref() {
            Ok(value) => tracing::trace!("{:?}", value),
            Err(e) => tracing::trace!("{:?}", e),
        }

        result
    }
}
