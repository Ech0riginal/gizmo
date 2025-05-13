//! https://tinkerpop.apache.org/docs/current/dev/provider/#_graph_driver_provider_requirements

use crate::structure::GKey;
use crate::{GValue, Response};
use derive_builder::Builder;
use std::collections::HashMap;
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
pub struct Args(HashMap<&'static str, GValue>);

impl Args {
    pub fn new() -> Self {
        Self(HashMap::with_capacity(8))
    }
    pub fn arg<V>(mut self, key: &'static str, value: V) -> Self
    where
        Self: Insert<V>,
    {
        Self::insert(&mut self.0, key, value);
        self
    }
}

impl Insert<GValue> for Args {
    fn insert(map: &mut HashMap<&'static str, GValue>, key: &'static str, value: GValue) {
        map.insert(key, value);
    }
}

impl<I> Insert<Option<I>> for Args
where
    Args: Insert<I>,
{
    fn insert(map: &mut HashMap<&'static str, GValue>, key: &'static str, value: Option<I>) {
        if let Some(inner_value) = value {
            Args::insert(map, key, inner_value);
        }
    }
}

macro_rules! insert {
    ($ty:path) => {
       impl Insert<$ty> for Args {
            fn insert(map: &mut HashMap<&'static str, GValue>, key: &'static str, value: $ty) {
                Args::insert(map, key, GValue::from(value));
            }
        }
    };
    (&$lt:lifetime $ty:path) => {
        impl<$lt> Insert<&$lt $ty> for Args {
            fn insert(map: &mut HashMap<&'static str, GValue>, key: &'static str, value: &$lt $ty) {
                Args::insert(map, key, GValue::from(value));
            }
        }
    }
}

insert!(String);
insert!(&'a String);
insert!(&'a str);
insert!(HashMap<&str, GValue>);
insert!(HashMap<GKey, GValue>);

pub(crate) trait Insert<I> {
    fn insert(map: &mut HashMap<&'static str, GValue>, key: &'static str, value: I);
}
