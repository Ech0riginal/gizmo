use crate::structure::GValue;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

crate::primitive_prelude!();
crate::very_primitive!(Set, HashSet<GValue>);
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
        let mut tmp = HashSet::new();
        tmp.extend(self.into_iter().map(GValue::from));
        Set(tmp)
    }
}
