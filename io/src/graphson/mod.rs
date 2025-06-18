// mod id;
mod key;
mod tags;
#[cfg(test)]
pub(crate) mod tests;
mod utils;
mod v2;
mod v3;

// pub use tags::Tag;
pub use utils::Ensure;

pub struct GraphSON<V> {
    _version: std::marker::PhantomData<V>,
}

pub trait GraphsonDeserializer<T, D> {
    fn deserialize(val: &serde_json::Value) -> Result<T, crate::Error>;
}

pub trait GraphsonSerializer<T, D> {
    fn serialize(val: &T) -> Result<serde_json::Value, crate::Error>;
}

impl<O, D, T> crate::Deserializer<O, serde_json::Value, D> for T
where
    T: GraphsonDeserializer<O, D>,
{
    fn do_deserialize(serial: &serde_json::Value) -> Result<O, crate::Error> {
        <T as GraphsonDeserializer<O, D>>::deserialize(serial)
    }
}

impl<O, D, T> crate::Serializer<O, serde_json::Value, D> for T
where
    T: GraphsonSerializer<O, D>,
{
    fn do_serialize(object: &O) -> Result<serde_json::Value, crate::Error> {
        <T as GraphsonSerializer<O, D>>::serialize(object)
    }
}

mod prelude {
    pub use indexmap::IndexMap;
    pub use serde_json::{Value, json};
    pub use snafu::location;

    pub use super::*;

    pub use crate::api::*;
    pub use crate::macros::*;
    pub use crate::types::*;
}
