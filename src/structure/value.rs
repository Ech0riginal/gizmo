use super::{Column, Direction, Merge};
use crate::process::traversal::TraversalBuilder;
use crate::structure::label::LabelType;
use crate::structure::traverser::Traverser;
use crate::structure::tree::Tree;
use crate::structure::*;
use crate::{GremlinError, GremlinResult};
use std::collections::HashMap;
use std::convert::Infallible;
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
#[allow(unused)]
macro_rules! try_from_gvalue {
    ($variant:ident, $primitive:ty) => {
        impl TryFrom<GValue> for $primitive {
            type Error = GremlinError;

            fn try_from(v: GValue) -> Result<Self, Self::Error> {
                match v {
                    GValue::$variant(v) => Ok(v),
                    gvalue => Err(GremlinError::Cast(
                        stringify!($variant).to_string(),
                        stringify!($primitive).to_string(),
                    )),
                }
            }
        }
    };
}
macro_rules! primitive_interop {
    ($variant:ident, $primitive:ty) => {
        from_primitive!($variant, $primitive);
        from_gvalue!($variant, $primitive);
        // try_from_gvalue!($variant, $primitive);
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
    Bool(Bool),
    Class(Class),
    Date(Date),
    Double(Double),
    Float(Float),
    Integer(Integer),
    List(List),
    Long(Long),
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
    Metric(Metrics),
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
        T: TryFrom<GValue>,
        GremlinError: From<T::Error>,
    {
        Ok(T::try_from(self)?)
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<T>
    where
        T: TryFrom<&'a GValue>,
        GremlinError: From<T::Error>,
    {
        Ok(T::try_from(self)?)
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

impl From<GID> for GValue {
    fn from(value: GID) -> Self {
        match value {
            GID::String(s) => Self::String(s),
            GID::Integer(i) => Self::Integer(i),
            GID::Long(l) => Self::Long(l),
        }
    }
}

impl From<LabelType> for GValue {
    fn from(value: LabelType) -> Self {
        match value {
            LabelType::Str(val) => GValue::String(val),
            LabelType::Bool(val) => GValue::Bool(Bool(val)),
            LabelType::T(val) => GValue::T(val),
        }
    }
}

impl<T> From<Vec<T>> for GValue
where
    GValue: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        let vec = value.into_iter().map(GValue::from).collect::<Vec<_>>();
        GValue::List(List(vec))
    }
}
// impl From<Vec<GValue>> for GValue {
//     fn from(value: Vec<GValue>) -> Self {
//         GValue::List(value.into())
//     }
// }
//
//
// // impl Into<GValue> for Vec<GValue> {
// //     fn into(self) -> GValue {
//         GValue::List(List(self))
//     }
// }

impl From<Infallible> for GremlinError {
    fn from(value: Infallible) -> Self {
        GremlinError::Generic("Inconceivable".into())
    }
}
impl From<GValue> for Vec<String> {
    fn from(value: GValue) -> Self {
        todo!()
    }
}
impl From<TraversalBuilder> for GValue {
    fn from(value: TraversalBuilder) -> Self {
        value.bytecode.into()
    }
}
