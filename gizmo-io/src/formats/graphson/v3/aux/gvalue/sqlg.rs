use super::serde;
use crate::dialects::SQLg;
use crate::formats::graphson::prelude::*;

serde!(
    SQLg,
    Class -> Class,
    Date -> Date,
    Double -> Double,
    Float -> Float,
    Integer -> Integer,
    List -> List<GValue>,
    Long -> Long,
    Map -> Map<GValue, GValue>,
    Set -> Set,
    Timestamp -> Timestamp,
    Uuid -> Uuid,

    Edge -> Edge,
    Path -> Path,
    Property -> Property,
    TinkerGraph -> TinkerGraph,
    Vertex -> Vertex,
    VertexProperty -> VertexProperty,

    Barrier -> Barrier,
    Binding -> Binding,
    BulkSet -> BulkSet,
    Bytecode -> Bytecode,
    Cardinality -> Cardinality,
    Column -> Column,
    Direction -> Direction,
    Lambda -> Lambda,
    Merge -> Merge,
    Metrics -> Metrics,
    Operator -> Operator,
    Order -> Order,
    P -> P,
    Pick -> Pick,
    Pop -> Pop,
    Scope -> Scope,
    T -> T,
    TextP -> TextP,
    TraversalMetrics -> TraversalMetrics,
    Traverser -> Traverser,

    Geometry -> Geometry,
);
