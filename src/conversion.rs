use crate::structure::*;

use crate::process::traversal::TraversalBuilder;
use crate::structure::*;
use crate::{GremlinError, GremlinResult};
use TryInto;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use geo_types::{Point, Polygon};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::Formatter;
use std::hash::Hash;

pub trait ToGValue: Send + Sync {
    fn to_gvalue(&self) -> GValue;
}

#[derive(Debug, PartialEq)]
pub struct Params(pub HashMap<String, GValue>);

impl Into<Params> for () {
    fn into(self) -> Params {
        Params(HashMap::new())
    }
}

impl ToGValue for Vec<GValue> {
    fn to_gvalue(&self) -> GValue {
        GValue::List(List::new(self.clone()))
    }
}

impl ToGValue for GID {
    fn to_gvalue(&self) -> GValue {
        match self {
            GID::Int32(n) => GValue::from(*n),
            GID::Int64(n) => GValue::from(*n),
            GID::String(n) => GValue::from(n),
        }
    }
}

macro_rules! impl_to_gvalue {
    ($t:ty, $v:path) => {
        impl ToGValue for $t {
            fn to_gvalue(&self) -> GValue {
                $v(*self)
            }
        }
    };
    ($t:ty, $cast:ty, $v:path) => {
        impl ToGValue for $t {
            fn to_gvalue(&self) -> GValue {
                $v(*self.into())
            }
        }
    };
}

impl_to_gvalue!(f32, GValue::Float);
impl_to_gvalue!(f64, GValue::Double);
impl_to_gvalue!(i32, GValue::Integer);
impl_to_gvalue!(i64, GValue::Long);
// impl_to_gvalue!(chrono::DateTime<chrono::Utc>, GValue::Date);
impl_to_gvalue!(uuid::Uuid, GValue::Uuid);
impl_to_gvalue!(bool, GValue::Bool);

impl ToGValue for chrono::DateTime<chrono::Utc> {
    fn to_gvalue(&self) -> GValue {
        GValue::Date(Date::from(*self))
    }
}

impl ToGValue for &str {
    fn to_gvalue(&self) -> GValue {
        GValue::String(String::from(*self))
    }
}

impl ToGValue for P {
    fn to_gvalue(&self) -> GValue {
        GValue::P(self.clone())
    }
}

impl ToGValue for TextP {
    fn to_gvalue(&self) -> GValue {
        GValue::TextP(self.clone())
    }
}

impl ToGValue for String {
    fn to_gvalue(&self) -> GValue {
        GValue::String(self.clone())
    }
}

impl ToGValue for Bytecode {
    fn to_gvalue(&self) -> GValue {
        GValue::Bytecode(self.clone())
    }
}

// Take from GValue
#[doc(hidden)]
pub trait FromGValue: Sized {
    fn from_gvalue(v: GValue) -> GremlinResult<Self>;
}
macro_rules! impl_from_gvalue {
    ($t:ty, $v:path) => {
        impl FromGValue for $t {
            fn from_gvalue(v: GValue) -> GremlinResult<$t> {
                match v {
                    $v(e) => Ok(e),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot convert {:?} to {}",
                        v,
                        stringify!($t)
                    ))),
                }
            }
        }
    };
}

impl_from_gvalue!(VertexProperty, GValue::VertexProperty);
impl_from_gvalue!(Property, GValue::Property);
impl_from_gvalue!(Map, GValue::Map);
impl_from_gvalue!(Set, GValue::Set);
impl_from_gvalue!(List, GValue::List);
impl_from_gvalue!(Token, GValue::Token);
impl_from_gvalue!(Vertex, GValue::Vertex);
impl_from_gvalue!(Edge, GValue::Edge);
impl_from_gvalue!(Path, GValue::Path);
impl_from_gvalue!(String, GValue::String);
impl_from_gvalue!(f32, GValue::Float);
impl_from_gvalue!(f64, GValue::Double);
impl_from_gvalue!(i32, GValue::Integer);
impl_from_gvalue!(i64, GValue::Long);
impl_from_gvalue!(bool, GValue::Bool);
impl_from_gvalue!(uuid::Uuid, GValue::Uuid);
impl_from_gvalue!(Metric, GValue::Metric);
impl_from_gvalue!(TraversalMetrics, GValue::TraversalMetrics);
impl_from_gvalue!(TraversalExplanation, GValue::TraversalExplanation);
impl_from_gvalue!(IntermediateRepr, GValue::IntermediateRepr);
impl_from_gvalue!(chrono::DateTime<chrono::Utc>, GValue::Date);
impl_from_gvalue!(Traverser, GValue::Traverser);

impl FromGValue for Null {
    fn from_gvalue(v: GValue) -> GremlinResult<Self> {
        match v {
            GValue::Null => Ok(Null {}),
            _ => Err(GremlinError::Cast(format!(
                "Cannot convert {:?} to {}",
                v,
                stringify!($t)
            ))),
        }
    }
}

impl FromGValue for GKey {
    fn from_gvalue(v: GValue) -> GremlinResult<GKey> {
        match v {
            GValue::Direction(d) => Ok(GKey::Direction(d)),
            GValue::String(s) => Ok(GKey::String(s)),
            GValue::Token(s) => Ok(GKey::String(s.value().clone())),
            GValue::Vertex(s) => Ok(GKey::Vertex(s)),
            GValue::Edge(s) => Ok(GKey::Edge(s)),
            _ => Err(GremlinError::Cast(format!(
                "Cannot convert {:?} to {}",
                v, "GKey"
            ))),
        }
    }
}

impl FromGValue for GValue {
    fn from_gvalue(v: GValue) -> GremlinResult<GValue> {
        Ok(v)
    }
}
// Borrow from GValue

impl<T: FromGValue> FromGValue for Vec<T> {
    fn from_gvalue(v: GValue) -> GremlinResult<Vec<T>> {
        match v {
            GValue::List(l) => {
                let results: GremlinResult<Vec<T>> =
                    l.take().into_iter().map(T::from_gvalue).collect();
                Ok(results?)
            }
            _ => Err(GremlinError::Cast(format!(
                "Cannot convert {:?} to List of T",
                v
            ))),
        }
    }
}

#[doc(hidden)]
pub trait BorrowFromGValue: Sized {
    fn from_gvalue<'a>(v: &'a GValue) -> GremlinResult<&'a Self>;
}

macro_rules! impl_borrow_from_gvalue {
    ($t:ty, $v:path) => {
        impl BorrowFromGValue for $t {
            fn from_gvalue<'a>(v: &'a GValue) -> GremlinResult<&'a $t> {
                match v {
                    $v(e) => Ok(e),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot convert {:?} to {}",
                        v,
                        stringify!($t)
                    ))),
                }
            }
        }
    };
}

impl_borrow_from_gvalue!(VertexProperty, GValue::VertexProperty);
impl_borrow_from_gvalue!(Property, GValue::Property);
impl_borrow_from_gvalue!(Map, GValue::Map);
impl_borrow_from_gvalue!(Set, GValue::Set);
impl_borrow_from_gvalue!(List, GValue::List);
impl_borrow_from_gvalue!(Vertex, GValue::Vertex);
impl_borrow_from_gvalue!(Edge, GValue::Edge);
impl_borrow_from_gvalue!(Path, GValue::Path);
impl_borrow_from_gvalue!(String, GValue::String);
impl_borrow_from_gvalue!(Token, GValue::Token);
impl_borrow_from_gvalue!(f32, GValue::Float);
impl_borrow_from_gvalue!(f64, GValue::Double);
impl_borrow_from_gvalue!(i32, GValue::Integer);
impl_borrow_from_gvalue!(i64, GValue::Long);
impl_borrow_from_gvalue!(uuid::Uuid, GValue::Uuid);
impl_borrow_from_gvalue!(chrono::DateTime<chrono::Utc>, GValue::Date);
impl_borrow_from_gvalue!(bool, GValue::Bool);

// impl From<&Point> for GValue {
//     fn from(value: &Point) -> Self {
//         GValue::Geometry(geo_types::Geometry::Point(*value))
//     }
// }
//
// impl From<Point> for GValue {
//     fn from(value: Point) -> Self {
//         GValue::Geometry(geo_types::Geometry::Point(value))
//     }
// }
//
// impl From<&Polygon> for GValue {
//     fn from(value: &Polygon) -> Self {
//         GValue::Geometry(geo_types::Geometry::Polygon(value.clone()))
//     }
// }
//
// impl From<DateTime<Utc>> for GValue {
//     fn from(val: DateTime<Utc>) -> Self {
//         GValue::Date(val)
//     }
// }
//
// impl From<NaiveDate> for GValue {
//     fn from(val: NaiveDate) -> Self {
//         let naive_dt: NaiveDateTime = val.and_hms_opt(0, 0, 0).unwrap();
//         let date: Date = Utc.from_utc_datetime(&naive_dt);
//         GValue::Date(date)
//     }
// }
//
// impl From<String> for GValue {
//     fn from(val: String) -> Self {
//         GValue::String(val)
//     }
// }
//
// impl From<&String> for GValue {
//     fn from(val: &String) -> Self {
//         GValue::String(val.clone())
//     }
// }
//
// impl From<i8> for GValue {
//     fn from(val: i8) -> Self {
//         GValue::Integer(val as i32)
//     }
// }
// impl From<i16> for GValue {
//     fn from(val: i16) -> Self {
//         GValue::Integer(val as i32)
//     }
// }
// impl From<i32> for GValue {
//     fn from(val: i32) -> Self {
//         GValue::Integer(val)
//     }
// }
//
// impl From<i64> for GValue {
//     fn from(val: i64) -> Self {
//         GValue::Long(val)
//     }
// }
//
// impl From<f32> for GValue {
//     fn from(val: f32) -> Self {
//         GValue::Float(val)
//     }
// }
//
// impl From<&GID> for GValue {
//     fn from(value: &GID) -> Self {
//         value.to_gvalue()
//     }
// }
//
// impl From<f64> for GValue {
//     fn from(val: f64) -> Self {
//         GValue::Double(val)
//     }
// }
//
// impl<'a> From<&'a str> for GValue {
//     fn from(val: &'a str) -> Self {
//         GValue::String(String::from(val))
//     }
// }
//
// impl From<Vertex> for GValue {
//     fn from(val: Vertex) -> Self {
//         GValue::Vertex(val)
//     }
// }
//
// impl From<&Vertex> for GValue {
//     fn from(val: &Vertex) -> Self {
//         GValue::Vertex(val.clone())
//     }
// }
//
// impl From<Path> for GValue {
//     fn from(val: Path) -> Self {
//         GValue::Path(val)
//     }
// }
// impl From<Edge> for GValue {
//     fn from(val: Edge) -> Self {
//         GValue::Edge(val)
//     }
// }
//
// impl From<VertexProperty> for GValue {
//     fn from(val: VertexProperty) -> Self {
//         GValue::VertexProperty(val)
//     }
// }
//
// impl From<Traverser> for GValue {
//     fn from(val: Traverser) -> Self {
//         GValue::Traverser(val)
//     }
// }
// impl From<TraversalMetrics> for GValue {
//     fn from(val: TraversalMetrics) -> Self {
//         GValue::TraversalMetrics(val)
//     }
// }
//
// impl From<TraversalExplanation> for GValue {
//     fn from(val: TraversalExplanation) -> Self {
//         GValue::TraversalExplanation(val)
//     }
// }
//
// impl From<Map> for GValue {
//     fn from(value: Map) -> Self {
//         Self::Map(value)
//     }
// }
//
// impl From<Metric> for GValue {
//     fn from(val: Metric) -> Self {
//         GValue::Metric(val)
//     }
// }
//
// impl From<Property> for GValue {
//     fn from(val: Property) -> Self {
//         GValue::Property(val)
//     }
// }
//
// impl From<Scope> for GValue {
//     fn from(val: Scope) -> Self {
//         GValue::Scope(val)
//     }
// }
//
// impl From<Order> for GValue {
//     fn from(val: Order) -> Self {
//         GValue::Order(val)
//     }
// }
//
// impl From<Merge> for GValue {
//     fn from(value: Merge) -> Self {
//         GValue::Merge(value)
//     }
// }
//
// impl From<Direction> for GValue {
//     fn from(value: Direction) -> Self {
//         GValue::Direction(value)
//     }
// }
//
// impl From<Column> for GValue {
//     fn from(value: Column) -> Self {
//         GValue::Column(value)
//     }
// }
//
// impl From<Token> for GValue {
//     fn from(val: Token) -> Self {
//         GValue::Token(val)
//     }
// }
//
// impl From<HashMap<String, GValue>> for GValue {
//     fn from(val: HashMap<String, GValue>) -> Self {
//         GValue::Map(Map::from(val))
//     }
// }
//
// impl From<HashMap<GKey, GValue>> for GValue {
//     fn from(val: HashMap<GKey, GValue>) -> Self {
//         GValue::Map(Map::from(val))
//     }
// }
//
// impl From<BTreeMap<String, GValue>> for GValue {
//     fn from(val: BTreeMap<String, GValue>) -> Self {
//         GValue::Map(Map::from(val))
//     }
// }
//
// impl From<Vec<GValue>> for GValue {
//     fn from(val: Vec<GValue>) -> Self {
//         GValue::List(List::new(val))
//     }
// }
//
// impl From<GValue> for Vec<GValue> {
//     fn from(val: GValue) -> Self {
//         vec![val]
//     }
// }
//
// impl From<GValue> for VecDeque<GValue> {
//     fn from(val: GValue) -> Self {
//         match val {
//             GValue::List(l) => VecDeque::from(l.take()),
//             GValue::Set(l) => VecDeque::from(l.take()),
//             _ => VecDeque::from(vec![val]),
//         }
//     }
// }
//
// impl From<GKey> for GValue {
//     fn from(val: GKey) -> Self {
//         match val {
//             GKey::Direction(d) => GValue::Direction(d),
//             GKey::T(t) => GValue::T(t),
//             GKey::String(s) => GValue::String(s),
//             GKey::Token(s) => GValue::String(s.value().clone()),
//             GKey::Vertex(v) => GValue::Vertex(v),
//             GKey::Edge(v) => GValue::Edge(v),
//         }
//     }
// }
//
// impl From<P> for GValue {
//     fn from(val: P) -> GValue {
//         GValue::P(val)
//     }
// }
//
// impl From<TextP> for GValue {
//     fn from(val: TextP) -> GValue {
//         GValue::TextP(val)
//     }
// }
//
// impl From<T> for GValue {
//     fn from(val: T) -> GValue {
//         GValue::T(val)
//     }
// }
//
// impl From<Bytecode> for GValue {
//     fn from(val: Bytecode) -> GValue {
//         GValue::Bytecode(val)
//     }
// }
//
// impl From<bool> for GValue {
//     fn from(val: bool) -> GValue {
//         GValue::Bool(val)
//     }
// }
//
// impl From<LabelType> for GValue {
//     fn from(val: LabelType) -> GValue {
//         match val {
//             LabelType::Str(val) => val.into(),
//             LabelType::Bool(val) => val.into(),
//             LabelType::T(val) => val.into(),
//         }
//     }
// }
//
// impl From<Cardinality> for GValue {
//     fn from(val: Cardinality) -> GValue {
//         GValue::Cardinality(val)
//     }
// }
//
// impl From<uuid::Uuid> for GValue {
//     fn from(val: uuid::Uuid) -> GValue {
//         GValue::Uuid(val)
//     }
// }
//
impl From<TraversalBuilder> for GValue {
    fn from(value: TraversalBuilder) -> Self {
        value.bytecode.into()
    }
}
//
// impl TryFrom<GValue> for String {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::String(s) => Ok(s),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to String",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for i32 {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::Integer(s) => Ok(s),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to i32",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for i64 {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::Long(s) => Ok(s),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to i64",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for uuid::Uuid {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::Uuid(uid) => Ok(uid),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to Uuid",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for Date {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::Date(date) => Ok(date),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to DateTime<Utc>",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for bool {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::Bool(val) => Ok(val),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to bool",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for f32 {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::Float(x) => Ok(x),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to f32",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for f64 {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         match value {
//             GValue::Double(x) => Ok(x),
//             GValue::List(s) => from_list(s),
//             GValue::VertexProperty(vp) => vp.take(),
//             GValue::Property(p) => p.take(),
//             _ => Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to f64",
//                 value
//             ))),
//         }
//     }
// }
//
// impl TryFrom<GValue> for BTreeMap<String, GValue> {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         if let GValue::Map(m) = value {
//             m.try_into()
//         } else {
//             Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to BTreeMap<String, GValue>",
//                 value
//             )))
//         }
//     }
// }
//
// impl TryFrom<GValue> for HashMap<GKey, GValue> {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         if let GValue::Map(m) = value {
//             Ok(m.into())
//         } else {
//             Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to HashMap<GKey, GValue>",
//                 value
//             )))
//         }
//     }
// }
//
// impl TryFrom<GValue> for HashMap<String, GValue> {
//     type Error = GremlinError;
//
//     fn try_from(value: GValue) -> GremlinResult<Self> {
//         if let GValue::Map(m) = value {
//             m.try_into()
//         } else {
//             Err(GremlinError::Cast(format!(
//                 "Cannot cast {:?} to HashMap<String, GValue>",
//                 value
//             )))
//         }
//     }
// }

fn from_list<T>(glist: List) -> GremlinResult<T>
where
    T: TryFrom<GValue, Error = GremlinError>,
{
    let mut vec = glist.take();

    match vec.len() {
        1 => vec.pop().unwrap().try_into(),
        _ => Err(GremlinError::Cast(format!(
            "Cannot cast a List to {}",
            std::any::type_name::<T>(),
        ))),
    }
}

impl<T> From<Option<T>> for GValue
where
    T: Into<GValue>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => GValue::Null,
        }
    }
}

// Optional

macro_rules! impl_try_from_option {
    ($t:ty) => {
        impl TryFrom<GValue> for Option<$t> {
            type Error = GremlinError;

            fn try_from(value: GValue) -> GremlinResult<Self> {
                if let GValue::Null = value {
                    return Ok(None);
                }
                let res: $t = value.try_into()?;
                Ok(Some(res))
            }
        }
    };
}

// impl_try_from_option!(String);
// impl_try_from_option!(i32);
// impl_try_from_option!(i64);
// impl_try_from_option!(f32);
// impl_try_from_option!(f64);
// impl_try_from_option!(Date);
// impl_try_from_option!(uuid::Uuid);
// impl_try_from_option!(bool);

fn for_list<T>(glist: &List) -> GremlinResult<Vec<T>>
where
    T: TryFrom<GValue, Error = GremlinError>,
{
    glist
        .iter()
        .map(|x| x.clone().try_into())
        .collect::<GremlinResult<Vec<T>>>()
}

fn for_list_to_set<T>(glist: &List) -> GremlinResult<HashSet<T>>
where
    T: TryFrom<GValue, Error = GremlinError> + Hash + Eq,
{
    glist
        .iter()
        .map(|x| x.clone().try_into())
        .collect::<GremlinResult<HashSet<T>>>()
}

fn for_set<T>(gset: &Set) -> GremlinResult<HashSet<T>>
where
    T: TryFrom<GValue, Error = GremlinError> + Hash + Eq,
{
    gset.iter()
        .map(|x| x.clone().try_into())
        .collect::<GremlinResult<HashSet<T>>>()
}

macro_rules! impl_try_from_set {
    ($t:ty) => {
        //Ideally this would be handled in conversion.rs but because the GValue::Set holds a Vec
        //we handle converting it here
        impl FromGValue for HashSet<$t> {
            fn from_gvalue(value: GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list_to_set(&s),
                    GValue::Set(s) => for_set(&s),
                    GValue::Null => Ok(HashSet::new()),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to HashSet",
                        value
                    ))),
                }
            }
        }

        impl TryFrom<GValue> for HashSet<$t> {
            type Error = GremlinError;

            fn try_from(value: GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list_to_set(&s),
                    GValue::Set(s) => for_set(&s),
                    GValue::Null => Ok(HashSet::new()),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to HashSet",
                        value
                    ))),
                }
            }
        }

        impl TryFrom<&GValue> for HashSet<$t> {
            type Error = GremlinError;

            fn try_from(value: &GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list_to_set(s),
                    GValue::Set(s) => for_set(s),
                    GValue::Null => Ok(HashSet::new()),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to HashSet",
                        value
                    ))),
                }
            }
        }
    };
}

// impl_try_from_set!(String);
// impl_try_from_set!(i32);
// impl_try_from_set!(i64);
// impl_try_from_set!(Date);
// impl_try_from_set!(uuid::Uuid);
// impl_try_from_set!(bool);
//floats do not conform to the Eq or Hash traits
// impl_try_from_set!(f32);
// impl_try_from_set!(f64);

macro_rules! impl_try_from_list {
    ($t:ty) => {
        impl TryFrom<GValue> for Vec<$t> {
            type Error = GremlinError;

            fn try_from(value: GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list(&s),
                    GValue::Null => Ok(Vec::new()),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to Vec",
                        value
                    ))),
                }
            }
        }

        impl TryFrom<&GValue> for Vec<$t> {
            type Error = GremlinError;

            fn try_from(value: &GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list(s),
                    GValue::Null => Ok(Vec::new()),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to Vec",
                        value
                    ))),
                }
            }
        }
    };
}

impl_try_from_list!(String);
impl_try_from_list!(i32);
impl_try_from_list!(i64);
impl_try_from_list!(f32);
impl_try_from_list!(f64);
impl_try_from_list!(Date);
impl_try_from_list!(uuid::Uuid);
impl_try_from_list!(bool);
