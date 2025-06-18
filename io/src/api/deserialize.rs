use snafu::ResultExt;
use super::*;

pub trait Deserializer<O, S, D> {
    fn do_deserialize(serial: &S) -> Result<O, Error>;
}

pub trait DeserializeExt: Sized {
    fn deserialize<F, D, T>(&self) -> Result<T, Error>
    where
        F: Format,
        F: Deserializer<T, Self, D>,
        D: Dialect,
        T: Object,
    {
        F::do_deserialize(self).context(ObjectSnafu { name: T::name })
    }
}
