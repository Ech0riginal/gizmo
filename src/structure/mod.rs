pub trait Primitive {
    #[allow(nonstandard_style)]
    const name: &'static str;
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

        impl crate::Primitive for $name {
            const name: &'static str = stringify!($name);
        }

        crate::deref!($name, $inner);

        crate::debug!($name);

        crate::display!($name);

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl Into<$inner> for $name {
            fn into(self) -> $inner {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! very_primitive {
    ($name:ident, $inner:ty) => {
        crate::primitive!($name, $inner);
        crate::partial_eq!($name);
        crate::eq!($name);
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
            type Item = crate::GValue;
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

primitive_prelude!();
very_primitive!(Bool, bool);
very_primitive!(Float, f32);
very_primitive!(Double, f64);
very_primitive!(Integer, i32);
very_primitive!(Long, i64);

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

mod bulk;
mod bytecode;
mod cardinality;
mod class;
mod column;
mod datetime;
mod direction;
mod edge;
mod either;
mod gid;
mod label;
mod list;
mod macros;
mod map;
mod merge;
mod metrics;
mod null;
mod order;
mod p;
mod path;
mod pop;
mod property;
mod result;
mod scope;
mod set;
mod star;
mod t;
mod text_p;
mod tinker;
mod token;
mod traverser;
mod tree;
mod value;
mod vertex;
mod vertex_property;

pub use bulk::BulkSet;
pub use bytecode::Bytecode;
pub use cardinality::Cardinality;
pub use class::Class;
pub use column::Column;
pub use datetime::*;
pub use direction::Direction;
pub use edge::Edge;
pub use either::*;
pub use gid::{GID, GIDs};
pub use label::Labels;
pub use list::List;
pub use map::{GKey, Map};
pub use merge::Merge;
pub use metrics::{IntermediateRepr, Metrics, TraversalExplanation, TraversalMetrics};
pub use null::Null;
pub use order::Order;
pub use p::{IntoPredicate, P};
pub use path::Path;
pub use pop::Pop;
pub use property::Property;
pub use result::GResultSet;
pub use scope::Scope;
pub use set::Set;
pub use star::StarGraph;
use std::hash::{Hash, Hasher};
pub use t::T;
pub use text_p::TextP;
pub use tinker::TinkerGraph;
pub use token::Token;
pub use traverser::Traverser;
pub use tree::Tree;
pub use value::GValue;
pub use vertex::Vertex;
pub use vertex_property::{GProperty, VertexProperty};

use crate::GremlinResult;
pub(crate) use tree::Branch;
