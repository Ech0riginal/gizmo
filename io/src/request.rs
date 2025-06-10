//! https://tinkerpop.apache.org/docs/current/dev/provider/#_graph_driver_provider_requirements

use crate::{GValue, Map};
use derive_builder::Builder;
use indexmap::IndexMap;
use std::hash::Hasher;
use uuid::Uuid;

#[derive(Clone, Debug, Builder)]
pub struct Request {
    #[builder(default = "uuid::Uuid::new_v4()")]
    pub(crate) id: Uuid,
    pub(crate) op: &'static str,
    pub(crate) proc: &'static str,
    pub(crate) args: Args,
}
impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::create_empty()
    }
}
impl Eq for Request {}
impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl std::hash::Hash for Request {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Args(pub(crate) Map<&'static str, GValue>);

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Self {
        Self(Map::with_capacity(8))
    }

    pub fn iter(&self) -> indexmap::map::Iter<&'static str, GValue> {
        self.0.iter()
    }

    pub fn arg<V>(mut self, key: &'static str, value: V) -> Self
    where
        Self: Insert<V>,
    {
        Self::insert(&mut self.0, key, value);
        self
    }
}

impl std::ops::Deref for Args {
    type Target = IndexMap<&'static str, GValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Insert<GValue> for Args {
    fn insert(map: &mut IndexMap<&'static str, GValue>, key: &'static str, value: GValue) {
        map.insert(key, value);
    }
}

impl<I> Insert<Option<I>> for Args
where
    Args: Insert<I>,
{
    fn insert(map: &mut IndexMap<&'static str, GValue>, key: &'static str, value: Option<I>) {
        if let Some(inner_value) = value {
            Args::insert(map, key, inner_value);
        }
    }
}

macro_rules! insert {
    ($ty:path) => {
       impl Insert<$ty> for Args {
            fn insert(map: &mut IndexMap<&'static str, GValue>, key: &'static str, value: $ty) {
                Args::insert(map, key, GValue::from(value));
            }
        }
    };
    (&$lt:lifetime $ty:path) => {
        impl<$lt> Insert<&$lt $ty> for Args {
            fn insert(map: &mut IndexMap<&'static str, GValue>, key: &'static str, value: &$lt $ty) {
                Args::insert(map, key, GValue::from(value));
            }
        }
    }
}

insert!(String);
insert!(&'a String);
insert!(&'a str);
insert!(IndexMap<&str, GValue>);
insert!(IndexMap<GValue, GValue>);

trait Insert<I> {
    fn insert(map: &mut IndexMap<&'static str, GValue>, key: &'static str, value: I);
}
