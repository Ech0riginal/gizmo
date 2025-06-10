//! A lot of our tests get thrown off by out-of-order items. I don't care about order, thus:

use crate::io::graphson::tests::diff::Diffd::*;
use crate::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug, thiserror::Error, Eq, PartialEq, Hash)]
pub enum Diffd {
    #[error("The values are the same")]
    Same,
    #[error("These values differ")]
    Different(String, String),
    #[error("An item has no mate.")]
    Item(String),
    #[error("A field ({0}) begs to differ")]
    Field(String),
    #[error("Two collections keys differ at {0}")]
    Key(String),
    #[error("GValues are intrinsically different.")]
    Intrinsic,
}
impl Add for Diffd {
    type Output = Diffd;

    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            Same if rhs == Same => Same,
            Same if rhs != Same => rhs,
            _ => self,
        }
    }
}
struct Field {
    name: String,
    value: String,
}

fn field(str: &'static str) -> Diffd {
    Field(str.to_string())
}
fn diff<A: Debug, B: Debug>(a: A, b: B) -> Diffd {
    Different(format!("{:?}", a), format!("{:?}", b))
}

/// Debugging is easy until you're scanning through JSON with 100+ lines.
/// This aims to make it a bit easier via deep introspection.
pub trait Diff {
    fn diff(&self, other: &Self) -> Diffd;
}

impl Diff for GValue {
    fn diff(&self, other: &Self) -> Diffd {
        // Boy do I love a good macro
        match self {
            GValue::Null => match other {
                GValue::Null => Same,
                _ => Intrinsic,
            },
            GValue::Bool(a) => match other {
                GValue::Bool(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Class(a) => match other {
                GValue::Class(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Date(a) => match other {
                GValue::Date(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Double(a) => match other {
                GValue::Double(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Float(a) => match other {
                GValue::Float(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Integer(a) => match other {
                GValue::Integer(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::List(a) => match other {
                GValue::List(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Long(a) => match other {
                GValue::Long(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Map(a) => match other {
                GValue::Map(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Set(a) => match other {
                GValue::Set(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::String(a) => match other {
                GValue::String(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Timestamp(a) => match other {
                GValue::Timestamp(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Uuid(a) => match other {
                GValue::Uuid(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Edge(a) => match other {
                GValue::Edge(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Path(a) => match other {
                GValue::Path(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Property(a) => match other {
                GValue::Property(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::StarGraph(a) => match other {
                GValue::StarGraph(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::TinkerGraph(a) => match other {
                GValue::TinkerGraph(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Tree(a) => match other {
                GValue::Tree(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Vertex(a) => match other {
                GValue::Vertex(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::VertexProperty(a) => match other {
                GValue::VertexProperty(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Bytecode(a) => match other {
                GValue::Bytecode(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Cardinality(a) => match other {
                GValue::Cardinality(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Column(a) => match other {
                GValue::Column(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Direction(a) => match other {
                GValue::Direction(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Order(a) => match other {
                GValue::Order(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Pop(a) => match other {
                GValue::Pop(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::P(a) => match other {
                GValue::P(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Scope(a) => match other {
                GValue::Scope(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::T(a) => match other {
                GValue::T(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::TraversalMetrics(a) => match other {
                GValue::TraversalMetrics(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Traverser(a) => match other {
                GValue::Traverser(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Int128(a) => match other {
                GValue::Int128(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Token(a) => match other {
                GValue::Token(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Metric(a) => match other {
                GValue::Metric(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::TraversalExplanation(a) => match other {
                GValue::TraversalExplanation(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::IntermediateRepr(a) => match other {
                GValue::IntermediateRepr(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::TextP(a) => match other {
                GValue::TextP(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Geometry(a) => match other {
                GValue::Geometry(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::Merge(a) => match other {
                GValue::Merge(b) => a.diff(b),
                _ => Intrinsic,
            },
            GValue::BulkSet(a) => match other {
                GValue::BulkSet(b) => a.diff(b),
                _ => Intrinsic,
            },
            _ => Intrinsic,
        }
    }
}
impl Diff for Box<GValue> {
    fn diff(&self, other: &Self) -> Diffd {
        (&**self).diff(&**other)
    }
}

impl Diff for Bool {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Class {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Date {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Double {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Float {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Integer {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl<T> Diff for List<T>
where
    T: Diff + Debug,
{
    fn diff(&self, other: &Self) -> Diffd {
        for this in self.iter() {
            if let Some(_) = other.iter().find(|other| other.diff(this) == Same) {
                {}
            } else {
                return Item(format!("{:?}", this));
            }
        }

        Same
    }
}
impl Diff for Long {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl<K, V> Diff for Map<K, V>
where
    K: Diff + Debug + Eq + Hash,
    V: Diff + Debug,
{
    fn diff(&self, other: &Self) -> Diffd {
        for (this_key, this_value) in self.iter() {
            let _debug = format!("{:?}", this_key);
            if let Some(that_value) = other.get(this_key) {
                match this_value.diff(that_value) {
                    Same => {}
                    other => return other,
                }
            } else {
                if let Some(_) = other.iter().find(|(that_key, that_value)| {
                    that_key.diff(this_key) == Same && that_value.diff(this_value) == Same
                }) {
                    {}
                } else {
                    return Item(format!("{:?}: {:?}", this_key, this_value));
                }
            }
        }

        Same
    }
}
impl Diff for Set {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for String {
    fn diff(&self, other: &Self) -> Diffd {
        if self == other {
            Same
        } else {
            Different(self.to_string(), other.to_string())
        }
    }
}
impl Diff for Timestamp {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Uuid {
    fn diff(&self, other: &Self) -> Diffd {
        if self == other {
            Same
        } else {
            Different(self.to_string(), other.to_string())
        }
    }
}
impl Diff for Edge {
    fn diff(&self, other: &Self) -> Diffd {
        if self.id != other.id {
            return diff(&self.id, &other.id);
        }

        if self.label != other.label {
            return diff(&self.label, &other.label);
        }

        // let map = other.properties;
        self.properties.diff(&other.properties)
    }
}
impl Diff for Path {
    fn diff(&self, other: &Self) -> Diffd {
        match self.labels.diff(&other.labels) {
            Same => {}
            other => return other,
        }
        self.objects.diff(&other.objects)
    }
}
impl Diff for Property {
    fn diff(&self, other: &Self) -> Diffd {
        if self.key != other.key {
            return field("key");
        }

        match self.value.diff(&other.value) {
            Same => {}
            other => return other,
        }

        self.element.diff(&other.element)
    }
}
impl Diff for StarGraph {
    fn diff(&self, other: &Self) -> Diffd {
        if self.id != other.id {
            return field("id");
        }

        if self.label != other.label {
            return field("label");
        }

        self.properties.diff(&other.properties)
    }
}
impl Diff for TinkerGraph {
    fn diff(&self, other: &Self) -> Diffd {
        match self.vertices.diff(&other.vertices) {
            Same => {}
            other => return other,
        }
        self.edges.diff(&other.edges)
    }
}
impl Diff for Tree {
    fn diff(&self, other: &Self) -> Diffd {
        self.branches.diff(&other.branches)
    }
}
impl Diff for Branch {
    fn diff(&self, other: &Self) -> Diffd {
        self.key.diff(&other.key) + self.value.diff(&other.value)
    }
}
impl Diff for Vertex {
    fn diff(&self, other: &Self) -> Diffd {
        if self.id != other.id {
            return field("id");
        }
        if self.label != other.label {
            return field("label");
        }
        self.properties.diff(&other.properties)
    }
}
impl Diff for VertexProperty {
    fn diff(&self, other: &Self) -> Diffd {
        if self.id != other.id {
            return field("id");
        }
        if self.value != other.value {
            return field("value");
        }
        if self.vertex != other.vertex {
            return field("vertex");
        }
        if self.label != other.label {
            return field("label");
        }

        Same
    }
}
impl Diff for Bytecode {
    fn diff(&self, other: &Self) -> Diffd {
        self.source_instructions.diff(&other.source_instructions)
    }
}
impl Diff for Instruction {
    fn diff(&self, other: &Self) -> Diffd {
        self.operator.diff(&other.operator) + self.args.diff(&other.args)
    }
}
impl Diff for Cardinality {
    fn diff(&self, other: &Self) -> Diffd {
        if self == other { Same } else { Intrinsic }
    }
}
impl Diff for Column {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Direction {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Order {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Pop {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for P {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Scope {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for T {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for TraversalMetrics {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Traverser {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for i128 {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Token {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Metrics {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for TraversalExplanation {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for IntermediateRepr {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for TextP {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Geometry {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for Merge {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
impl Diff for BulkSet {
    fn diff(&self, _other: &Self) -> Diffd {
        todo!()
    }
}
