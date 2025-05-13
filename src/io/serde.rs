//! A shadow of serde.

use crate::io::{Deserializer, Error, Serializer};
use serde_json::Value;

pub trait Serialize: Sized {
    fn serialize<S>(&self) -> Result<serde_json::Value, super::Error>
    where
        S: Serializer<Self>;
}

pub trait Deserialize: Sized {
    fn deserialize<V, T>(&self) -> Result<T, super::Error>
    where
        V: Deserializer<T>;
}

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
