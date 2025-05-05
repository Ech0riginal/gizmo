//! https://tinkerpop.apache.org/docs/current/dev/provider/#_graph_driver_provider_requirements

use std::collections::HashMap;
use derive_builder::Builder;
use serde::{Serialize, Serializer};
use uuid::Uuid;
use crate::GValue;

#[derive(Debug, Builder, Serialize)]
// #[builder(pattern = "mutable")]
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

#[derive(Debug)]
pub struct Args(HashMap<&'static str, GValue>);

impl Args {
    pub fn new() -> Self {
        Self(HashMap::with_capacity(8))
    }
    pub fn arg<V>(mut self, key: &'static str, value: V) -> Self
    where
        V: Argd
    {
        V::Handler::insert(&mut self.0, key, value);
        self
    }
}

trait Argd: Sized {
    type Handler: Insert<Self>;
}

trait Insert<I> {
    fn insert(
        map: &mut HashMap<&'static str, GValue>,
        key: &'static str,
        item: I,
    );
}

struct ArgHandler;
struct IntoHandler;
struct OptionHandler;

// impl Argd for GValue {
//     type Handler = ArgHandler;
// }
impl<I: Into<GValue>> Argd for I {
    type Handler = IntoHandler;
}

impl<I: Into<GValue>> Argd for Option<I> {
    type Handler = OptionHandler;
}


impl Insert<GValue> for ArgHandler
{
    fn insert(
        map: &mut HashMap<&'static str, GValue>,
        key: &'static str,
        item: GValue,
    ) {
        map.insert(key, item);
    }
}

impl<I> Insert<I> for IntoHandler
where I:
    Into<GValue>
{
    fn insert(
        map: &mut HashMap<&'static str, GValue>,
        key: &'static str,
        item: I,
    ) {
        map.insert(key, item.into());
    }
}

impl<I> Insert<Option<I>> for OptionHandler
where I:
    Into<GValue>
{
    fn insert(
        map: &mut HashMap<&'static str, GValue>,
        key: &'static str,
        item: Option<I>,
    ) {
        if let Some(thing) = item {
            map.insert(key, thing.into());
        }
    }
}


impl Serialize for Args {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_request() {
    let req = Request::builder()
        .op("EVAL")
        .proc("EVAL")


    ;
}