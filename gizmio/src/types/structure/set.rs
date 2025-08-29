use std::hash::{Hash, Hasher};

use indexmap::IndexSet;

use crate::GValue;

crate::primitive_prelude!();
crate::very_primitive!(Set, IndexSet<GValue>);
crate::new!(Set, IndexSet);
crate::iter!(Set);
crate::into_iter!(Set);
crate::tag!(Set);

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.iter() {
            item.hash(state);
        }
    }
}

impl<T> From<Vec<T>> for Set
where
    GValue: From<T>,
{
    fn from(val: Vec<T>) -> Self {
        let mut tmp = IndexSet::new();
        tmp.extend(val.into_iter().map(GValue::from));
        Set(tmp)
    }
}
