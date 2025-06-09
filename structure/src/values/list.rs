use crate::*;

primitive_prelude!();

pub struct List<T>(pub(crate) Vec<T>);

impl<T> crate::Primitive for List<T> {
    const name: &'static str = "List";
}

impl<T> Default for List<T> {
    fn default() -> Self {
        list![]
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(::core::format_args!("{:?}", self.0))
    }
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(::core::format_args!(stringify!(List)))
    }
}

impl<T> Into<List<T>> for Vec<T> {
    fn into(self) -> List<T> {
        List(self)
    }
}

impl<T: PartialEq> PartialEq<Self> for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Eq> Eq for List<T> {}

impl<T> ops::Deref for List<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: std::hash::Hash> std::hash::Hash for List<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn with_capacity(n: usize) -> Self {
        Self(Vec::with_capacity(n))
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = List(Vec::new());
        for item in iter {
            list.push(item); // or whatever List's insertion method is
        }
        list
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

macro_rules! list {
    () => (
        $crate::List(std::vec::Vec::new())
    );
    ($elem:expr; $n:expr) => (
        $crate::List(std::vec::Vec::from_elem($elem, $n))
    );
    ($($x:expr),+ $(,)?) => (
        // Massage liballoc
        $crate::List(vec![$($x),+])
    );
}

pub(crate) use list;
