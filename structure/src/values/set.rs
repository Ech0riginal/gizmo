use crate::GValue;

use indexmap::IndexSet;
use std::hash::{Hash, Hasher};

crate::primitive_prelude!();
crate::very_primitive!(Set, IndexSet<GValue>);
crate::new!(Set, IndexSet);
crate::iter!(Set);
crate::into_iter!(Set);

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.iter() {
            item.hash(state);
        }
    }
}

impl<T> Into<Set> for Vec<T>
where
    GValue: From<T>,
{
    fn into(self) -> Set {
        let mut tmp = IndexSet::new();
        tmp.extend(self.into_iter().map(GValue::from));
        Set(tmp)
    }
}
