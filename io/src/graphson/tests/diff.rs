//! A lot of our tests get thrown off by out-of-order items. I don't care about order, thus:

use crate::*;
use Difference::*;
use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

#[derive(Debug)]
pub struct Debuggery<'a> {
    pub diff: Difference,
    pub a: Box<&'a dyn Debug>,
    pub b: Box<&'a dyn Debug>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Difference {
    Same,
    Intrinsic,
    Field(String),
}

impl<'a> PartialEq for Debuggery<'a> {
    fn eq(&self, other: &Self) -> bool {
        match &self.diff {
            Same => match other.diff {
                Same => true,
                _ => false,
            },
            Intrinsic => match other.diff {
                Intrinsic => true,
                _ => false,
            },
            Field(a_field) => match &other.diff {
                Field(b_field) => a_field == b_field,
                _ => false,
            },
        }
    }
}

impl<'a> Display for Debuggery<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.diff {
            Same => write!(f, "Same\n"),
            Intrinsic => {
                _ = write!(f, "Intrinsic\n");
                _ = write!(f, "-------------------\n");
                _ = write!(f, "{:?}\n", self.a);
                _ = write!(f, "-------------------\n");
                write!(f, "{:?}\n", self.b)
            }
            Field(field) => {
                _ = write!(f, "{}\n", field);
                _ = write!(f, "-------------------\n");
                _ = write!(f, "{:?}\n", self.a);
                _ = write!(f, "-------------------\n");
                write!(f, "{:?}\n", self.b)
            }
        }
    }
}
trait Differ: Sized + Debug {
    fn differ<'a>(&'a self, other: &'a Self, diff: Difference) -> Debuggery<'a> {
        Debuggery {
            diff,
            a: Box::new(self),
            b: Box::new(other),
        }
    }
}
impl<T: Debug> Differ for T {}

/// Debugging is easy until you're scanning through JSON with 100+ lines.
/// This aims to make it a bit easier via deep introspection.
pub trait Diff {
    fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a>;
}

macro_rules! branch {
    ($a:ident, $b:ident, $field:ident) => {
        let result = $a.$field.diff(&$b.$field);
        if result.diff != Same {
            return result;
        }
    };
}
macro_rules! same {
    ($a:ident, $b:ident) => {
        $a.differ($b, Same)
    };
}

macro_rules! diff {
    ($variant:ident, $($field:ident),+) => {
        impl Diff for $variant {
            fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
                $(branch!(self, other, $field);)+
                same!(self, other)
            }
        }
    };
}

macro_rules! basic {
    ($variant:ident) => {
        impl Diff for $variant {
            fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
                if self == other {
                    self.differ(other, Same)
                } else {
                    self.differ(other, Intrinsic)
                }
            }
        }
    };
}

impl<T: Diff + Debug> Diff for Option<T> {
    fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
        match self {
            Some(a) => match other {
                Some(b) => a.diff(b),
                None => self.differ(other, Intrinsic),
            },
            None => match other {
                None => self.differ(other, Same),
                Some(_) => self.differ(other, Intrinsic),
            },
        }
    }
}

mod gvalues {
    use super::*;
    impl Diff for GValue {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            // Boy do I love a good macro
            match self {
                GValue::Null => match other {
                    GValue::Null => self.differ(other, Same),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Bool(a) => match other {
                    GValue::Bool(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Class(a) => match other {
                    GValue::Class(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Date(a) => match other {
                    GValue::Date(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Double(a) => match other {
                    GValue::Double(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Float(a) => match other {
                    GValue::Float(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Integer(a) => match other {
                    GValue::Integer(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::List(a) => match other {
                    GValue::List(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Long(a) => match other {
                    GValue::Long(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Map(a) => match other {
                    GValue::Map(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Set(a) => match other {
                    GValue::Set(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::String(a) => match other {
                    GValue::String(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Timestamp(a) => match other {
                    GValue::Timestamp(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Uuid(a) => match other {
                    GValue::Uuid(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Edge(a) => match other {
                    GValue::Edge(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Path(a) => match other {
                    GValue::Path(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Property(a) => match other {
                    GValue::Property(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::StarGraph(a) => match other {
                    GValue::StarGraph(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::TinkerGraph(a) => match other {
                    GValue::TinkerGraph(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Tree(a) => match other {
                    GValue::Tree(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Vertex(a) => match other {
                    GValue::Vertex(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::VertexProperty(a) => match other {
                    GValue::VertexProperty(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Bytecode(a) => match other {
                    GValue::Bytecode(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Cardinality(a) => match other {
                    GValue::Cardinality(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Column(a) => match other {
                    GValue::Column(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Direction(a) => match other {
                    GValue::Direction(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Order(a) => match other {
                    GValue::Order(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Pop(a) => match other {
                    GValue::Pop(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::P(a) => match other {
                    GValue::P(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Scope(a) => match other {
                    GValue::Scope(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::T(a) => match other {
                    GValue::T(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::TraversalMetrics(a) => match other {
                    GValue::TraversalMetrics(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Traverser(a) => match other {
                    GValue::Traverser(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Int128(a) => match other {
                    GValue::Int128(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Token(a) => match other {
                    GValue::Token(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Metric(a) => match other {
                    GValue::Metric(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                // GValue::TraversalExplanation(a) => match other {
                //     GValue::TraversalExplanation(b) => a.diff(b),
                //     _ => self.differ(other, Intrinsic),
                // },
                // GValue::IntermediateRepr(a) => match other {
                //     GValue::IntermediateRepr(b) => a.diff(b),
                //     _ => self.differ(other, Intrinsic),
                // },
                GValue::TextP(a) => match other {
                    GValue::TextP(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Geometry(a) => match other {
                    GValue::Geometry(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::Merge(a) => match other {
                    GValue::Merge(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
                GValue::BulkSet(a) => match other {
                    GValue::BulkSet(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
            }
        }
    }
    impl Diff for Box<GValue> {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            (&**self).diff(&**other)
        }
    }
    impl Diff for GID {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            match self {
                GID::String(a) => match other {
                    GID::String(b) => {
                        if a == b {
                            self.differ(other, Same)
                        } else {
                            self.differ(other, Intrinsic)
                        }
                    }
                    _ => self.differ(other, Intrinsic),
                },
                GID::Integer(a) => match other {
                    GID::Integer(b) => {
                        if a == b {
                            self.differ(other, Same)
                        } else {
                            self.differ(other, Intrinsic)
                        }
                    }
                    _ => self.differ(other, Intrinsic),
                },
                GID::Long(a) => match other {
                    GID::Long(b) => {
                        if a == b {
                            self.differ(other, Same)
                        } else {
                            self.differ(other, Intrinsic)
                        }
                    }
                    _ => self.differ(other, Intrinsic),
                },
            }
        }
    }

    impl<T> Diff for List<T>
    where
        T: Diff + Debug,
    {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            for this in self.iter() {
                if let Some(_) = other.iter().find(|other| other.diff(this).diff == Same) {
                    {}
                } else {
                    return self.differ(other, Intrinsic);
                }
            }

            self.differ(other, Same)
        }
    }
    impl Diff for Long {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl<K, V> Diff for Map<K, V>
    where
        K: Diff + Debug + Eq + Hash,
        V: Diff + Debug,
    {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            for (this_key, this_value) in self.iter() {
                let _debug = format!("{:?}", this_key);
                if let Some(that_value) = other.get(this_key) {
                    match this_value.diff(that_value).diff {
                        Same => {}
                        diff => return this_value.differ(that_value, diff),
                    }
                } else {
                    if let Some(_) = other.iter().find(|(that_key, that_value)| {
                        that_key.diff(this_key).diff == Same
                            && that_value.diff(this_value).diff == Same
                    }) {
                        {}
                    } else {
                        return self.differ(other, Field(format!("{:?}", this_key)));
                    }
                }
            }

            self.differ(other, Same)
        }
    }
    basic!(Bool);
    basic!(Class);
    basic!(Date);
    basic!(Double);
    basic!(Float);
    basic!(Integer);
    basic!(Set);
    basic!(String);
    basic!(Timestamp);
    basic!(Uuid);
    diff!(Edge, id, label, properties);
    diff!(Path, labels, objects);
    diff!(Property, key, value, element);
    diff!(StarGraph, id, label, properties);
    diff!(TinkerGraph, edges, vertices);
    diff!(Tree, branches);
    diff!(Branch, key, value);
    diff!(Vertex, id, label, properties);
    diff!(VertexProperty, id, value, vertex, label);
    diff!(Bytecode, source_instructions);
    diff!(Instruction, operator, args);
    basic!(Cardinality);
    basic!(Column);
    basic!(Direction);
    basic!(Order);
    impl Diff for Pop {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for P {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for Scope {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for T {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for TraversalMetrics {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for Traverser {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for i128 {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for Token {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for Metrics {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    // impl Diff for TraversalExplanation {
    //     fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
    //         todo!()
    //     }
    // }
    // impl Diff for IntermediateRepr {
    //     fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
    //         todo!()
    //     }
    // }
    impl Diff for TextP {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for Geometry {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for Merge {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
    impl Diff for BulkSet {
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            todo!()
        }
    }
}
mod serde {
    use super::*;
    use serde_json::Value;

    impl Diff for Value {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            match self {
                Value::Null => match other {
                    Value::Null => self.differ(other, Same),
                    _ => self.differ(other, Intrinsic),
                },
                Value::Bool(a) => match other {
                    Value::Bool(b) => {
                        let diff = if a == b { Same } else { Intrinsic };
                        self.differ(other, diff)
                    }
                    _ => self.differ(other, Intrinsic),
                },
                Value::Number(a) => match other {
                    Value::Number(b) => {
                        let diff = if a == b { Same } else { Intrinsic };
                        self.differ(other, diff)
                    }
                    _ => self.differ(other, Intrinsic),
                },
                Value::String(a) => match other {
                    Value::String(b) => {
                        let diff = if a == b { Same } else { Intrinsic };
                        self.differ(other, diff)
                    }
                    _ => self.differ(other, Intrinsic),
                },
                Value::Array(a) => match other {
                    Value::Array(b) => a.diff(&b),
                    _ => self.differ(other, Intrinsic),
                },
                Value::Object(a) => match other {
                    Value::Object(b) => a.diff(b),
                    _ => self.differ(other, Intrinsic),
                },
            }
        }
    }

    impl<T> Diff for Vec<T>
    where
        T: Diff + Debug,
    {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            for this in self.iter() {
                if let Some(_) = other.iter().find(|other| other.diff(this).diff == Same) {
                    {}
                } else {
                    return self.differ(other, Intrinsic);
                }
            }

            self.differ(other, Same)
        }
    }

    impl Diff for serde_json::Map<String, Value> {
        fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
            for (this_key, this_value) in self.iter() {
                let _debug = format!("{:?}", this_key);
                if let Some(that_value) = other.get(this_key) {
                    let prop = this_value.diff(that_value);
                    match prop.diff {
                        Same => {}
                        _ => return prop,
                    }
                } else {
                    if let Some(_) = other.iter().find(|(that_key, that_value)| {
                        that_key.diff(this_key).diff == Same
                            && that_value.diff(this_value).diff == Same
                    }) {
                        {}
                    } else {
                        return self.differ(other, Field(format!("{:?}", this_key)));
                    }
                }
            }

            self.differ(other, Same)
        }
    }
}

impl Diff for &'static str {
    fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
        if self == other {
            self.differ(other, Same)
        } else {
            self.differ(other, Intrinsic)
        }
    }
}
impl Diff for Args {
    fn diff<'a>(&'a self, other: &'a Self) -> Debuggery<'a> {
        self.0.diff(&other.0)
    }
}

diff!(Response, id, data, status, meta);
diff!(Request, id, op, proc, args);
diff!(Status, code, message, attributes);
basic!(i16);
basic!(i32);
basic!(i64);
basic!(f32);
basic!(f64);
