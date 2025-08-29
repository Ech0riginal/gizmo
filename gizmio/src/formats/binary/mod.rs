mod v1;

use bytes::Bytes;

use crate::{Deserializer, Serializer};

#[derive(Clone, Copy, Debug)]
pub struct GraphBinary<V> {
    _version: std::marker::PhantomData<V>,
}

pub trait GraphBinaryDeserializer<T, D> {
    fn deserialize(val: &Bytes) -> Result<T, crate::Error>;
}

pub trait GraphBinarySerializer<T, D> {
    fn serialize(val: &T) -> Result<Bytes, crate::Error>;
}

impl<O, D, T> Deserializer<O, Bytes, D> for T
where
    T: GraphBinaryDeserializer<O, D>,
{
    fn deserialize(serial: &Bytes) -> Result<O, crate::Error> {
        <T as GraphBinaryDeserializer<O, D>>::deserialize(serial)
    }
}

impl<O, D, T> Serializer<O, Bytes, D> for T
where
    T: GraphBinarySerializer<O, D>,
{
    fn serialize(object: &O) -> Result<Bytes, crate::Error> {
        <T as GraphBinarySerializer<O, D>>::serialize(object)
    }
}

pub(crate) mod prelude {
    #![allow(unused_imports)]

    pub use indexmap::IndexMap;
    pub use snafu::location;

    pub use super::*;
    pub use crate::api::*;
    pub use crate::formats::typing::Typed;
    pub use crate::types::*;
}
