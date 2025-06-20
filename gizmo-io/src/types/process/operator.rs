//! https://github.com/apache/tinkerpop/blob/master/gremlin-core/src/main/java/org/apache/tinkerpop/gremlin/process/traversal/Operator.java
use crate::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Operator {
    Sum,
    Minus,
    Mult,
    Div,
    Min,
    Max,
    Assign,
    And,
    Or,
    AddAll,
    SumLong,
}

obj!(Operator);
tag!(Operator);
