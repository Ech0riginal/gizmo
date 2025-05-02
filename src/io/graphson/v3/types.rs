graphson_types! {
    core,
    CLASS, "g:Class",
    DATE, "g:Date",
    DOUBLE, "g:Double",
    FLOAT, "g:Float",
    INT, "g:Int32",
    LIST, "g:List",
    LONG, "g:Int64",
    MAP, "g:Map",
    SET, "g:Set",
    TIMESTAMP, "g:Timestamp",
    UUID, "g:UUID"
}

graphson_types! {
    structure,
    EDGE, "g:Edge",
    PATH, "g:Path",
    PROPERTY, "g:Property",
    TINKER_GRAPH, "g:TinkerGraph",
    VERTEX, "g:Vertex",
    VERTEX_PROPERTY, "g:VertexProperty"

}

graphson_types! {
    process,
    BARRIER, "g:Barrier",
    BINDING, "g:Binding",
    BULK_SET, "g:BulkSet",
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
    TRAVERSAL_METRICS, "g:TraversalMetrics",
    TRAVERSER, "g:Traverser"
}

pub use self::core::*;
pub use self::process::*;
pub use self::structure::*;
