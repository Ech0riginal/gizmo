//! A lot of our tests get thrown off by out-of-order items. I don't care about order, thus:

use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use Difference::*;

use crate::*;

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
                _ = write!(f, "Expected: {:?}\n", self.a);
                write!(f, "  Actual: {:?}\n", self.b)
            }
            Field(field) => {
                _ = write!(f, "{}\n", field);
                _ = write!(f, "Expected: {:?}\n", self.a);
                write!(f, "  Actual: {:?}\n", self.b)
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
        // Compiler will say _other's unused due to macro use
        fn diff<'a>(&'a self, _other: &'a Self) -> Debuggery<'a> {
            macro_rules! match_maker {
                ($($variant:ident,)*$(,)?) => {
                    match self {
                        GValue::Null => match _other {
                            GValue::Null => self.differ(_other, Same),
                            _ => self.differ(_other, Intrinsic),
                        },
                        $(
                        GValue::$variant(a) => match _other {
                            GValue::$variant(b) => a.diff(b),
                            _ => self.differ(_other, Intrinsic),
                        },
                        )*
                    }
                };
            }

            match_maker!(
                // Core
                Bool,
                Class,
                Date,
                Double,
                Float,
                Integer,
                List,
                Long,
                Map,
                Set,
                String,
                Timestamp,
                Uuid,
                // Structure
                Edge,
                Path,
                Property,
                StarGraph,
                TinkerGraph,
                Tree,
                Vertex,
                VertexProperty,
                // Process
                Barrier,
                Binding,
                BulkSet,
                Bytecode,
                Cardinality,
                Column,
                Direction,
                Operator,
                Order,
                Lambda,
                Merge,
                Metrics,
                P,
                Pop,
                Pick,
                Scope,
                T,
                TextP,
                TraversalMetrics,
                Traverser,
                Geometry,
                Int128,
            )
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
                        if *a == *b {
                            self.differ(other, Same)
                        } else {
                            self.differ(other, Intrinsic)
                        }
                    }
                    _ => self.differ(other, Intrinsic),
                },
                GID::Integer(a) => match other {
                    GID::Integer(b) => {
                        if *a == *b {
                            self.differ(other, Same)
                        } else {
                            self.differ(other, Intrinsic)
                        }
                    }
                    _ => self.differ(other, Intrinsic),
                },
                GID::Long(a) => match other {
                    GID::Long(b) => {
                        if *a == *b {
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
    basic!(Long);
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

    basic!(Barrier);
    basic!(Binding);
    basic!(BulkSet);
    diff!(Bytecode, source_instructions, step_instructions);
    basic!(Cardinality);
    basic!(Column);
    basic!(Direction);
    diff!(Instruction, op, args);
    diff!(Lambda, script, language, arguments);
    basic!(Merge);
    diff!(
        Metrics,
        id,
        duration,
        name,
        elements,
        traversers,
        annotations,
        nested
    );
    basic!(Operator);
    basic!(Order);
    basic!(P);
    basic!(Pick);
    basic!(Pop);
    basic!(Scope);
    basic!(T);
    basic!(TextP);
    // basic!(Token);  // This is the same a T right?
    diff!(TraversalMetrics, duration, metrics);
    diff!(Traverser, bulk, value);

    basic!(Geometry);
    basic!(i128);
}
mod serde {
    use serde_json::Value;

    use super::*;

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
