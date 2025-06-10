use crate::io::Error;
use crate::{GID, GValue, Request, Response};
use serde_json::Value;

#[allow(private_bounds)]
pub trait GremlinIO
where
    Self: 'static,
    Self: Sealed,
    Self: Send + Sync + Clone,
    Self: Deserializer<Response> + Serializer<Request>,
    Self: Deserializer<GValue> + Serializer<GValue>,
    Self: Deserializer<GID> + Serializer<GID>,
{
    #[allow(nonstandard_style)]
    const version: &'static str;

    #[allow(nonstandard_style)]
    const mime: &'static str;
}

pub trait Sealed {}

pub trait Deserialize: Sized {
    fn deserialize<V, T>(&self) -> Result<T, Error>
    where
        V: Deserializer<T>;
}

pub trait Deserializer<T>: Sealed {
    fn deserialize(val: &Value) -> Result<T, Error>;
}

pub trait Serialize: Sized {
    fn serialize<S>(&self) -> Result<Value, Error>
    where
        S: Serializer<Self>;
}

pub trait Serializer<T>: Sealed {
    fn serialize(val: &T) -> Result<Value, Error>;
}

mod blankets {
    use crate::io::api::{Deserialize, Serialize};
    use crate::io::{Deserializer, Error, Serializer};
    use serde_json::Value;

    impl<T> Serialize for T {
        fn serialize<S>(&self) -> Result<Value, Error>
        where
            S: Serializer<Self>,
        {
            S::serialize(self)
        }
    }

    impl Deserialize for Value {
        fn deserialize<V, T>(&self) -> Result<T, Error>
        where
            V: Deserializer<T>,
        {
            V::deserialize(self)
        }
    }
}
