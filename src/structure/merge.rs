// use crate::process::traversal::step::OptionStep;
use crate::structure::{GKey, GValue};
use std::collections::HashMap;

#[derive(Clone)]
pub struct MergeBuilder {
    pub(crate) inner: Merge,
    map: HashMap<GKey, GValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Merge {
    OnCreate,
    OnMatch,
    OutV,
    InV,
}

impl std::ops::Deref for MergeBuilder {
    type Target = Merge;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Merge> for MergeBuilder {
    fn from(value: Merge) -> Self {
        Self {
            inner: value,
            map: Default::default(),
        }
    }
}

#[allow(nonstandard_style)]
impl Merge {
    pub fn OnCreate() -> MergeBuilder {
        MergeBuilder::from(Self::OnCreate)
    }

    pub fn OnMatch() -> MergeBuilder {
        MergeBuilder::from(Self::OnMatch)
    }

    pub fn OutV() -> MergeBuilder {
        MergeBuilder::from(Self::OutV)
    }

    pub fn InV() -> MergeBuilder {
        MergeBuilder::from(Self::InV)
    }
}
impl MergeBuilder {
    // Polymorphic rust demands these flags, or we'll get tons of warnings
    #[allow(private_bounds)]
    pub fn insert<K, V>(self, key: K, value: V) -> Self
    where
        K: Into<GKey>,
        V: Into<GValue> + Handled,
    {
        V::Helper::insert(self, key, value)
    }

    pub fn extend<K, V>(mut self, iter: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<GKey>,
        V: Into<GValue>,
    {
        self.map
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }
}

trait Handled: Sized {
    type Helper: Insert<Self>;
}

trait Insert<V> {
    fn insert<K: Into<GKey>>(merge: MergeBuilder, key: K, item: V) -> MergeBuilder;
}

#[allow(dead_code)]
struct ValueHandler;

#[allow(dead_code)]
struct OptionHandler;

impl<A: Into<GValue>> Insert<A> for ValueHandler {
    fn insert<K: Into<GKey>>(mut merge: MergeBuilder, key: K, item: A) -> MergeBuilder {
        merge.map.insert(key.into(), item.into());
        merge
    }
}

impl<V: Into<GValue>> Insert<Option<V>> for OptionHandler {
    fn insert<K: Into<GKey>>(mut merge: MergeBuilder, key: K, item: Option<V>) -> MergeBuilder {
        if let Some(value) = item {
            merge.map.insert(key.into(), value.into());
        }
        merge
    }
}

// impl Into<OptionStep> for MergeBuilder {
//     fn into(self) -> OptionStep {
//         OptionStep::from((self.inner, self.map))
//     }
// }
