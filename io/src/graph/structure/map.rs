use crate::*;
use indexmap::IndexMap;
use std::hash::{Hash, Hasher};
// Represent a Map<[GKey](struct.GKey),[GValue](struct.GValue)> which has ability to allow for non-String keys.
// TinkerPop type [here](http://tinkerpop.apache.org/docs/current/dev/io/#_map)

primitive_prelude!();

#[derive(Clone)]
pub struct Map<K, V>(pub(crate) IndexMap<K, V>);
impl<K, V> crate::Primitive for Map<K, V> {
    const name: &'static str = "Map";
}
impl<K: Default, V: Default> Default for Map<K, V> {
    fn default() -> Self {
        Self(IndexMap::default())
    }
}
impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(::core::format_args!("{:?}", self.0))
    }
}
impl<K: fmt::Display, V: fmt::Display> fmt::Display for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(::core::format_args!(stringify!(Map)))
    }
}
impl<K, V> From<Map<K, V>> for IndexMap<K, V> {
    fn from(val: Map<K, V>) -> Self {
        val.0
    }
}
impl<K, V> From<IndexMap<K, V>> for Map<K, V> {
    fn from(val: IndexMap<K, V>) -> Self {
        Map(val)
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq<Self> for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.iter())
    }
}

impl<K: Eq, V: Eq> Eq for Map<K, V> {}
impl<K: Hash, V: Hash> Hash for Map<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}
impl<K, V> ops::Deref for Map<K, V> {
    type Target = IndexMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V> ops::DerefMut for Map<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for Map<K, V>
where
    K: Eq + Hash,
{
    fn from(value: [(K, V); N]) -> Self {
        let mut tmp = Self::with_capacity(N);
        for (k, v) in value {
            tmp.insert(k, v);
        }
        tmp
    }
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Map(IndexMap::<K, V, _>::new())
    }
    pub fn with_capacity(n: usize) -> Self {
        Map(IndexMap::<K, V, _>::with_capacity(n))
    }
}

impl<K, V> Map<K, V>
where
    K: Eq + Hash,
{
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    pub fn remove<T: Into<K>>(&mut self, key: T) -> Option<V> {
        let key = key.into();
        self.0.swap_remove(&key)
    }
    pub fn remove_ok<V_, T: Into<K>>(&mut self, key: T) -> Result<V_, io::Error>
    where
        K: fmt::Display,
        V: Into<V_>,
    {
        let key = key.into();
        self.0
            .swap_remove(&key)
            .map(|item| item.into())
            .ok_or(io::Error::Missing(format!("{key}")))
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tmp = Self::new();
        for (k, v) in iter {
            tmp.insert(k, v);
        }
        tmp
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = indexmap::map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
