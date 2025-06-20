#[cfg(test)]
pub(crate) mod tests;

pub(crate) use utils::*;

mod utils;
mod v2;
mod v3;

pub struct GraphSON<V> {
    _version: std::marker::PhantomData<V>,
}

pub trait GraphsonDeserializer<T, D> {
    fn deserialize(val: &serde_json::Value) -> Result<T, crate::Error>;
}

pub trait GraphsonSerializer<T, D> {
    fn serialize(val: &T) -> Result<serde_json::Value, crate::Error>;
}

mod blankets {
    use crate::api::ObjectSnafu;
    use crate::formats::graphson::{GraphsonDeserializer, GraphsonSerializer};
    use crate::{DeserializeExt, Dialect, Named};
    use snafu::ResultExt;

    impl DeserializeExt for serde_json::Value {}

    impl<O, D, T> crate::Deserializer<O, serde_json::Value, D> for T
    where
        O: Named,
        D: Dialect,
        T: GraphsonDeserializer<O, D>,
    {
        fn deserialize(serial: &serde_json::Value) -> Result<O, crate::Error> {
            T::deserialize(serial).context(ObjectSnafu { name: O::name })
        }
    }

    impl<O, D, T> crate::Serializer<O, serde_json::Value, D> for T
    where
        O: Named,
        D: Dialect,
        T: GraphsonSerializer<O, D>,
    {
        fn serialize(object: &O) -> Result<serde_json::Value, crate::Error> {
            T::serialize(object).context(ObjectSnafu { name: O::name })
        }
    }
}

mod prelude {
    pub use indexmap::IndexMap;
    pub use serde_json::{Value, json};
    pub use snafu::location;

    pub use super::*;
    pub use crate::api::*;
    pub use crate::types::*;

    pub(crate) use crate::macros::*;
}
