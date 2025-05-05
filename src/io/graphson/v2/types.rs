crate::io::macros::types! {
    core,
    CLASS, "g:Class",
    INT, "g:Int32",
    LONG, "g:Int64",
    FLOAT, "g:Float",
    DOUBLE, "g:Double",
    DATE, "g:Date",
    TIMESTAMP, "g:Timestamp",
    UUID, "g:UUID"
}

crate::io::macros::types! {
    structure,
    EDGE, "g:Edge",
    PATH, "g:Path",
    PROPERTY, "g:Property",
    STAR_GRAPH, "g:StarGraph",
    TINKER_GRAPH, "tinker:graph",
    TREE, "g:Tree",
    VERTEX, "g:Vertex",
    VERTEX_PROPERTY, "g:VertexProperty"

}

crate::io::macros::types! {
    process,
    BARRIER, "g:Barrier",
    BINDING, "g:Binding",
    BYTECODE, "g:Bytecode",
    CARDINALITY, "g:Cardinality",
    COLUMN, "g:Column",
    DIRECTION, "g:Direction",
    DT, "g:DT",
    LAMBDA, "g:Lambda",
    MERGE, "g:Merge",
    METRICS, "g:Metrics",
    OPERATOR, "g:Operator",
    ORDER, "g:Order",
    P, "g:P",
    PICK, "g:Pick",
    POP, "g:Pop",
    SCOPE, "g:Scope",
    T, "g:T",
    TEXT_P, "g:TextP",
    TRAVERSAL_EXPLANATION, "g:TraversalExplanation",
    TRAVERSAL_METRICS, "g:TraversalMetrics",
    TRAVERSER, "g:Traverser"
}

pub use self::core::*;
pub use self::process::*;
pub use self::structure::*;
