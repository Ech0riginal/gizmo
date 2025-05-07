use super::{Direction, T};
use crate::GremlinResult;
use crate::error::GremlinError;
use crate::structure::*;
use crate::structure::{Edge, GValue, Vertex};
use std::collections::hash_map::IntoIter;
use std::collections::{BTreeMap, HashMap};
use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;

/// Represent a Map<[GKey](struct.GKey),[GValue](struct.GValue)> which has ability to allow for non-String keys.
/// TinkerPop type [here](http://tinkerpop.apache.org/docs/current/dev/io/#_map)
// #[derive(PartialEq, Clone)]
// pub struct Map(pub(crate) HashMap<GKey, GValue>);

crate::primitive_prelude!();
crate::primitive!(Map, HashMap<GKey, GValue>);

// impl std::fmt::Debug for Map {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{ ")?;
//
//         if !self.0.is_empty() {
//             let mut iter = self.0.iter();
//             if let Some((k, v)) = iter.next() {
//                 write!(f, "({:?}, {:?}), ", k, v)?;
//             }
//             while let Some((k, v)) = iter.next() {
//                 write!(f, ", ({:?}, {:?})", k, v)?;
//             }
//         }
//
//         write!(f, "}}")
//     }
// }

impl Map {
    pub(crate) fn empty() -> Map {
        Map(HashMap::default())
    }
}

// impl From<HashMap<GKey, GValue>> for Map {
//     fn from(val: HashMap<GKey, GValue>) -> Self {
//         Map(val)
//     }
// }

// impl From<Map> for HashMap<GKey, GValue> {
//     fn from(map: Map) -> Self {
//         map.0
//     }
// }

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

impl Map {
    pub(crate) fn remove<T>(&mut self, key: T) -> Option<GValue>
    where
        T: Into<GKey>,
    {
        self.0.remove(&key.into())
    }
    /// Iterate all key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&GKey, &GValue)> {
        self.0.iter()
    }

    ///Returns a reference to the value corresponding to the key.
    pub fn get<T>(&self, key: T) -> Option<&GValue>
    where
        T: Into<GKey>,
    {
        self.0.get(&key.into())
    }

    ///Returns try_get and conversion
    pub fn try_get<K, V>(&self, key: K) -> GremlinResult<V>
    where
        K: Into<GKey>,
        V: TryFrom<GValue, Error = GremlinError>,
    {
        self.0
            .get(&key.into())
            .cloned()
            .or_else(|| Some(GValue::Null))
            .map(V::try_from)
            .ok_or_else(|| GremlinError::Cast(String::from("field not found")))?
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Into<GKey>> std::ops::Index<T> for Map {
    type Output = GValue;

    fn index(&self, key: T) -> &GValue {
        self.0.get(&key.into()).expect("no entry found for key")
    }
}

// impl std::ops::Deref for Map {
//     type Target = HashMap<GKey, GValue>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

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
            Err(GremlinError::Cast(String::from(format!(
                "Cannot cast from {:?} to String",
                k
            ))))
        }
    }
}
