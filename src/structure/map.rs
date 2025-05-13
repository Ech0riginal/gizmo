use super::{Direction, T};
use crate::error::GremlinError;
use crate::structure::*;
use crate::structure::{Edge, GValue, Vertex};
use std::collections::hash_map::IntoIter;
use std::collections::{BTreeMap, HashMap};
use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;

// Represent a Map<[GKey](struct.GKey),[GValue](struct.GValue)> which has ability to allow for non-String keys.
// TinkerPop type [here](http://tinkerpop.apache.org/docs/current/dev/io/#_map)

crate::primitive_prelude!();
crate::very_primitive!(Map, HashMap<GKey, GValue>);

impl<S> From<HashMap<S, GValue>> for Map
where
    S: AsRef<str>,
{
    fn from(val: HashMap<S, GValue>) -> Self {
        Map(val
            .into_iter()
            .map(|(k, v)| (GKey::String(k.as_ref().to_string()), v))
            .collect())
    }
}

impl TryFrom<Map> for HashMap<String, GValue> {
    type Error = GremlinError;

    fn try_from(map: Map) -> Result<Self, Self::Error> {
        map.into_iter()
            .map(|(k, v)| Ok((k.try_into()?, v)))
            .collect()
    }
}

impl From<BTreeMap<String, GValue>> for Map {
    fn from(val: BTreeMap<String, GValue>) -> Self {
        let map = val.into_iter().map(|(k, v)| (GKey::String(k), v)).collect();
        Map(map)
    }
}

impl TryFrom<Map> for BTreeMap<String, GValue> {
    type Error = GremlinError;

    fn try_from(map: Map) -> Result<Self, Self::Error> {
        map.into_iter()
            .map(|(k, v)| Ok((k.try_into()?, v)))
            .collect()
    }
}

impl<T: Into<GKey>> std::ops::Index<T> for Map {
    type Output = GValue;

    fn index(&self, key: T) -> &GValue {
        self.0.get(&key.into()).expect("no entry found for key")
    }
}

impl IntoIterator for Map {
    type Item = (GKey, GValue);
    type IntoIter = IntoIter<GKey, GValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::iter::FromIterator<(String, GValue)> for Map {
    fn from_iter<I: IntoIterator<Item = (String, GValue)>>(iter: I) -> Self {
        Map(iter
            .into_iter()
            .map(|(k, v)| (GKey::String(k), v))
            .collect())
    }
}
macro_rules! enom {
    ($name:ident, $($variant:ident),+) => {
        /// Possible key types in a [Map](struct.Map)
        #[allow(clippy::large_enum_variant)]
        #[derive(PartialEq, Clone, Eq, Hash)]
        pub enum $name {
            $($variant($variant),)+
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant(value) => write!(f, "{:?}", value),)+
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant(_) => write!(f, stringify!($variant)),)+
                }
            }
        }

        impl<T> From<&T> for $name
        where
            T: Clone,
            Self: From<T>,
        {
            fn from(v: &T) -> Self {
                Self::from(v.clone())
            }
        }

        $(
            impl From<$variant> for $name {
                fn from(v: $variant) -> Self {
                    $name::$variant(v)
                }
            }
        )+
    };
}

enom!(GKey, T, String, Token, Vertex, Edge, Direction);

impl From<&str> for GKey {
    fn from(val: &str) -> Self {
        GKey::String(String::from(val))
    }
}

impl From<GValue> for GKey {
    fn from(value: GValue) -> Self {
        match value {
            GValue::T(t) => Self::T(t),
            GValue::String(string) => Self::String(string),
            GValue::Token(tok) => Self::Token(tok),
            GValue::Vertex(vert) => Self::Vertex(vert),
            GValue::Edge(e) => Self::Edge(e),
            GValue::Direction(d) => Self::Direction(d),
            gvalue => panic!("Cannot cast GValue::{} to GKey", gvalue),
        }
    }
}

impl TryFrom<GKey> for String {
    type Error = GremlinError;

    fn try_from(k: GKey) -> Result<Self, Self::Error> {
        if let GKey::String(s) = k {
            Ok(s)
        } else {
            Err(GremlinError::Cast(format!("{:?}", k), "String".into()))
        }
    }
}
impl TryFrom<&GKey> for String {
    type Error = GremlinError;

    fn try_from(k: &GKey) -> Result<Self, Self::Error> {
        if let GKey::String(s) = k {
            Ok(s.clone())
        } else {
            Err(GremlinError::Cast(format!("{:?}", k), "String".into()))
        }
    }
}
