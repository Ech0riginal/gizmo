use crate::Error;
use crate::*;
use indexmap::IndexMap;
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
            type Error = Error;

            fn try_from(v: GValue) -> Result<Self, Self::Error> {
                match v {
                    GValue::$variant(v) => Ok(v),
                    gvalue => Err(Error::Cast(
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
    ($($variant:ident($primitive:ty)),+$(,)?) => {
        /// Represent possible values coming from the [Gremlin Server](http://tinkerpop.apache.org/docs/3.4.0/dev/io/)
        #[allow(clippy::large_enum_variant)]
        #[derive(Clone, Eq, Hash, PartialEq)]
        #[derive(Default)]
pub enum GValue {
            #[default]
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
impl Object for GValue {
    const name: &'static str = "GValue";
}
enom!(
    // Core
    Bool(Bool),
    Class(Class),
    Date(Date),
    Double(Double),
    Float(Float),
    Integer(Integer),
    List(List<GValue>),
    Long(Long),
    Map(Map<GValue, GValue>),
    Set(Set),
    String(String),
    Timestamp(Timestamp),
    Uuid(Uuid),
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
    Barrier(Barrier),
    Binding(Binding),
    Bytecode(Bytecode),
    BulkSet(BulkSet),
    Cardinality(Cardinality),
    Column(Column),
    Direction(Direction),
    Operator(Operator),
    Order(Order),
    Lambda(Lambda),
    Metrics(Metrics),
    P(P),
    Pop(Pop),
    Pick(Pick),
    Scope(Scope),
    T(T),
    TraversalMetrics(TraversalMetrics),
    Traverser(Traverser),
    Geometry(Geometry),
    // Request
    Int128(i128),
    // Token(Token),
    // TraversalExplanation(TraversalExplanation),
    // IntermediateRepr(IntermediateRepr),
    TextP(TextP),
    Merge(Merge),
);

impl GValue {
    pub fn take<T>(self) -> Result<T, Error>
    where
        T: TryFrom<GValue>,
        Error: From<T::Error>,
    {
        Ok(T::try_from(self)?)
    }

    pub fn get<'a, T>(&'a self) -> Result<T, Error>
    where
        T: TryFrom<&'a GValue>,
        Error: From<T::Error>,
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

impl<T> From<IndexMap<T, GValue>> for GValue
where
    GValue: From<T>,
{
    fn from(v: IndexMap<T, GValue>) -> Self {
        GValue::Map(Map(v
            .into_iter()
            .map(|(k, v)| (GValue::from(k), v))
            .collect::<IndexMap<GValue, GValue>>()))
    }
}

// TODO GKey could very well be useful but I've already seen it broken in the IO ref soo <AOL>: Goodbye.
// impl From<GKey> for GValue {
//     fn from(value: GKey) -> Self {
//         match value {
//             GKey::T(v) => GValue::T(v),
//             GKey::String(v) => GValue::String(v),
//             GKey::Token(v) => GValue::Token(v),
//             GKey::Vertex(v) => GValue::Vertex(v),
//             GKey::Edge(v) => GValue::Edge(v),
//             GKey::Direction(v) => GValue::Direction(v),
//         }
//     }
// }

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

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Infallible
    }
}
impl From<GValue> for Vec<String> {
    fn from(value: GValue) -> Self {
        match value {
            GValue::List(list) => list.into_iter().flat_map(|i| i.take::<String>()).collect(),
            GValue::Set(set) => set.into_iter().flat_map(|i| i.take::<String>()).collect(),
            _ => vec![],
        }
    }
}
