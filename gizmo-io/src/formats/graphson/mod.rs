#[cfg(test)]
mod tests;

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
    use crate::*;
    use snafu::ResultExt;

    impl<O, D, T> Deserializer<O, serde_json::Value, D> for T
    where
        O: Named,
        D: Dialect,
        T: GraphsonDeserializer<O, D>,
    {
        fn deserialize(serial: &serde_json::Value) -> Result<O, Error> {
            T::deserialize(serial).context(ObjectSnafu { name: O::name })
        }
    }

    impl<O, D, T> Serializer<O, serde_json::Value, D> for T
    where
        O: Named,
        D: Dialect,
        T: GraphsonSerializer<O, D>,
    {
        fn serialize(object: &O) -> Result<serde_json::Value, Error> {
            T::serialize(object).context(ObjectSnafu { name: O::name })
        }
    }
}

pub(crate) mod prelude {
    pub use indexmap::IndexMap;
    pub use serde_json::{Value, json};
    pub use snafu::location;

    pub use super::*;
    pub use crate::api::*;
    pub use crate::macros::*;
    pub use crate::types::*;
}

#[test]
fn x() {
    let json = serde_json::json!(0.0004f64);
    let f = json.as_f64();
    println!("{:?}", f);
}
