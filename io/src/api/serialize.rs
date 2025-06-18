use super::*;

pub trait Serializer<O, S, D> {
    fn do_serialize(object: &O) -> Result<S, Error>;
}

pub trait SerializeExt: Sized {
    fn serialize<F, D>(&self) -> Result<F::Serial, Error>
    where
        F: Format,
        F: Serializer<Self, F::Serial, D>,
        D: Dialect,
    {
        F::do_serialize(self)
    }
}
