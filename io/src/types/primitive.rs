
pub trait Tag_<D> {
    const tag: &'static str;
}

#[macro_export]
macro_rules! primitive_prelude {
    () => {
        use std::{fmt, ops};
    };
}

#[macro_export]
macro_rules! primitive {
    ($name:ident, $inner:ty) => {
        #[derive(Clone)]
        pub struct $name(pub(crate) $inner);

        impl $crate::Object for $name {
            const name: &'static str = stringify!($name);
        }

        $crate::debug!($name);

        $crate::display!($name);

        impl Into<$inner> for $name {
            fn into(self) -> $inner {
                self.0
            }
        }
        impl Into<$name> for $inner {
            fn into(self) -> $name {
                $name(self)
            }
        }
    };
}

#[macro_export]
macro_rules! very_primitive {
    ($name:ident, $inner:ty) => {
        $crate::primitive!($name, $inner);
        $crate::partial_eq!($name);
        $crate::eq!($name);
        $crate::deref!($name, $inner);
        $crate::deref_mut!($name);
    };
}

#[macro_export]
macro_rules! getters {
    ($struct_:ident, $($field:ident -> $type_:ty),+) => {
        impl $struct_ { $(pub fn $field(&self) -> &$type_ { &self.$field })+ }
    };
}

#[macro_export]
macro_rules! obj {
    ($id:ident) => {
        impl $crate::Object for $id {
            const name: &'static str = stringify!($id);
        }
    };
}


#[macro_export]
macro_rules! tag {
    ($id:ident) => {
        $crate::tag!($id, const_format::concatcp!("g:", stringify!($id)));
    };
    ($id:ident, $tag:expr) => {
        impl<D: $crate::Dialect> $crate::Tag_<D> for $id {
            const tag: &'static str = $tag;
        }
    };
    ($id:ident, $dialect:ident) => {
        $crate::tag!($id, $dialect, const_format::concatcp!("g:", stringify!($id)));
    };
    ($id:ident, $dialect:ident, $tag:expr) => {
        impl $crate::Tag_<$dialect> for $id {
            const tag: &'static str = $tag;
        }
    };
}



#[macro_export]
macro_rules! new {
    ($name:ident, $inner:ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self($inner::new())
            }
        }
    };
}

#[macro_export]
macro_rules! deref {
    ($variant:ident, $primitive:ty) => {
        impl ops::Deref for $variant {
            type Target = $primitive;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! deref_mut {
    ($variant:ident) => {
        impl ops::DerefMut for $variant {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($variant:ident) => {
        impl fmt::Debug for $variant {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! display {
    ($variant:ident) => {
        impl fmt::Display for $variant {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, stringify!($variant))
            }
        }
    };
}

#[macro_export]
macro_rules! hash {
    ($variant:ident) => {
        impl std::hash::Hash for $variant {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
}

#[macro_export]
macro_rules! eq {
    ($variant:ident) => {
        impl Eq for $variant {}
    };
}

#[macro_export]
macro_rules! iter {
    ($variant:ident) => {
        impl $variant {
            pub fn iter(&self) -> impl Iterator<Item = &GValue> {
                self.0.iter()
            }
        }
    };
}

#[macro_export]
macro_rules! into_iter {
    ($variant:ident) => {
        impl IntoIterator for $variant {
            type Item = $crate::GValue;
            type IntoIter = impl Iterator<Item = GValue>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

#[macro_export]
macro_rules! partial_eq {
    ($variant:ident) => {
        impl PartialEq<Self> for $variant {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
    };
}

// TODO move these elsewhere
primitive_prelude!();
very_primitive!(Bool, bool);
tag!(Bool);
very_primitive!(Float, f32);
tag!(Float);
very_primitive!(Double, f64);
tag!(Double);
very_primitive!(Integer, i32);
tag!(Integer);
very_primitive!(Long, i64);
tag!(Long);

// Misnomer: These are never used
obj!(String);
tag!(String);

use std::hash::{Hash, Hasher};

impl Hash for Bool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&[self.0 as u8])
    }
}
impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_be_bytes())
    }
}
impl Hash for Double {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_be_bytes())
    }
}
impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_be_bytes())
    }
}
impl Hash for Long {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_be_bytes())
    }
}
