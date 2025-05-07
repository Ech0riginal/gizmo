use super::{Column, Direction, Merge};
use crate::GremlinResult;
use crate::conversion::{BorrowFromGValue, FromGValue};
use crate::structure::traverser::Traverser;
use crate::structure::tree::Tree;
use crate::structure::*;
use std::collections::HashMap;
use std::fmt::Formatter;

macro_rules! from_primitive {
    ($variant:ident, $primitive:ty) => {
       impl From<$primitive> for GValue {
            fn from(v: $primitive) -> Self {
                GValue::$variant(v)
            }
        }
    };
    ($variant:ident, &$ly:lifetime $primitive:ty) => {
        impl From<&$ly $primitive> for GValue {
            fn from(v: &$ly $primitive) -> Self {
                GValue::$variant(*v.clone())
            }
        }
    };
}
macro_rules! from_gvalue {
    ($variant:ident, $primitive:ty) => {
        impl From<GValue> for $primitive {
            fn from(v: GValue) -> Self {
                match v {
                    GValue::$variant(v) => v,
                    gvalue => panic!("Cannot cast {} to {}", gvalue, stringify!($primitive)),
                }
            }
        }
    };
}
macro_rules! primitive_interop {
    ($variant:ident, $primitive:ty) => {
        from_primitive!($variant, $primitive);
        from_gvalue!($variant, $primitive);
    };

    ($variant:ident, &$ly:lifetime $primitive:ty) => {
        from_primitive!($variant, &$ly $primitive);
    };
}
macro_rules! enom {
    ($($variant:ident($primitive:ty)),+) => {
        /// Represent possible values coming from the [Gremlin Server](http://tinkerpop.apache.org/docs/3.4.0/dev/io/)
        #[allow(clippy::large_enum_variant)]
        #[derive(PartialEq, Clone)]
        pub enum GValue {
            Null,
            $($variant($primitive),)+
        }

        impl std::fmt::Debug for GValue {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    GValue::Null => write!(f, "null"),
                    $(GValue::$variant(value) => write!(f, "{:?}", value),)+
                }
            }
        }

        impl std::fmt::Display for GValue {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    GValue::Null => write!(f, "Null"),
                    $(GValue::$variant(_) => write!(f, stringify!($variant)),)+
                }
            }
        }

        $(primitive_interop!($variant, $primitive);)+
    }
}

enom!(
    // Core
    Bool(bool),
    Class(Class),
    Date(Date),
    Double(f64),
    Float(f32),
    Integer(i32),
    List(List),
    Long(i64),
    Map(Map),
    Set(Set),
    String(String),
    Timestamp(Timestamp),
    Uuid(uuid::Uuid),
    // Structure
    Edge(Edge),
    Path(Path),
    Property(Property),
    StarGraph(StarGraph),
    TinkerGraph(TinkerGraph),
    Tree(Tree),
    Vertex(Vertex),
    VertexProperty(VertexProperty),
    // Process
    // Barrier(Barrier),
    // Binding,
    Bytecode(Bytecode),
    Cardinality(Cardinality),
    Column(Column),
    Direction(Direction),
    // Operator
    Order(Order),
    // Pick(String)
    Pop(Pop),
    // Lambda
    // Metrics
    P(P),
    Scope(Scope),
    T(T),
    TraversalMetrics(TraversalMetrics),
    Traverser(Traverser),
    // Request
    Int128(i128),
    Token(Token),
    Metric(Metric),
    TraversalExplanation(TraversalExplanation),
    IntermediateRepr(IntermediateRepr),
    TextP(TextP),
    Geometry(geo_types::Geometry),
    Merge(Merge),
    BulkSet(BulkSet)
);

impl GValue {
    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: FromGValue,
    {
        T::from_gvalue(self)
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
    where
        T: BorrowFromGValue,
    {
        T::from_gvalue(self)
    }
}

impl From<&str> for GValue {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl<T> From<&T> for GValue
where
    T: Clone,
    Self: From<T>,
{
    fn from(v: &T) -> Self {
        Self::from(v.clone())
    }
}

impl<T> From<HashMap<T, GValue>> for GValue
where
    GKey: From<T>,
{
    fn from(v: HashMap<T, GValue>) -> Self {
        GValue::Map(Map(v
            .into_iter()
            .map(|(k, v)| (GKey::from(k), v))
            .collect::<HashMap<GKey, GValue>>()))
    }
}

impl From<Vec<GValue>> for GValue {
    fn from(value: Vec<GValue>) -> Self {
        GValue::List(value.into())
    }
}

impl From<GKey> for GValue {
    fn from(value: GKey) -> Self {
        match value {
            GKey::T(v) => GValue::T(v),
            GKey::String(v) => GValue::String(v),
            GKey::Token(v) => GValue::Token(v),
            GKey::Vertex(v) => GValue::Vertex(v),
            GKey::Edge(v) => GValue::Edge(v),
            GKey::Direction(v) => GValue::Direction(v),
        }
    }
}
