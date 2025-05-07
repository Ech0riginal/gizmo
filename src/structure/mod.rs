#[macro_export]
macro_rules! primitive_prelude {
    () => {
        use std::{fmt, ops};
    };
}

#[macro_export]
macro_rules! primitive {
    ($name:ident, $inner:ty) => {
        #[derive(Clone, PartialEq)]
        pub struct $name(pub(crate) $inner);

        impl ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, stringify!($name))
            }
        }

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
pub use metrics::{IntermediateRepr, Metric, TraversalExplanation, TraversalMetrics};
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
pub use t::T;
pub use text_p::TextP;
pub use tinker::TinkerGraph;
pub use token::Token;
pub use traverser::Traverser;
pub use tree::Tree;
pub use value::GValue;
pub use vertex::Vertex;
pub use vertex_property::{GProperty, VertexProperty};

// pub(crate) use label::LabelType;
pub(crate) use tree::Branch;
