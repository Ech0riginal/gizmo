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
string_reprs! {
    Operator,
    SUM -> "sum",
    MINUS -> "minus",
    MULT -> "mult",
    DIV -> "div",
    MIN -> "min",
    MAX -> "max",
    ASSIGN -> "assign",
    AND -> "and",
    OR -> "or",
    ADD_ALL -> "addAll",
    SUM_LONG -> "sumLong",
}
