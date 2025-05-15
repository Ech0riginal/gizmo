use crate::io::Error;
use crate::structure::GID;
use crate::{GValue, Request, Response};
use serde_json::Value;

#[allow(private_bounds)]
pub trait GremlinIO
where
    Self: 'static,
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

pub trait Deserialize: Sized {
    fn deserialize<V, T>(&self) -> Result<T, Error>
    where
        V: Deserializer<T>;
}

pub trait Deserializer<T>: crate::Sealed {
    fn deserialize(val: &Value) -> Result<T, Error>;
}

pub trait Serialize: Sized {
    fn serialize<S>(&self) -> Result<Value, Error>
    where
        S: Serializer<Self>;
}

pub trait Serializer<T>: crate::Sealed {
    fn serialize(val: &T) -> Result<Value, Error>;
}

pub trait IOHelpers {
    fn get<'a>(value: &'a Value, key: &'static str) -> Result<&'a Value, Error> {
        value.get(key).ok_or(Error::Missing(key))
    }
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
            V::deserialize(&self).map(|a| a.into())
        }
    }
}
