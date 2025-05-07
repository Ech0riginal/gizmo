crate::io::macros::types! {
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

crate::io::macros::types! {
    structure,
    EDGE, "g:Edge",
    PATH, "g:Path",
    PROPERTY, "g:Property",
    TINKER_GRAPH, "g:TinkerGraph",
    VERTEX, "g:Vertex",
    VERTEX_PROPERTY, "g:VertexProperty"

}

crate::io::macros::types! {
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

/*
   CLASS => todo!("CLASS deserializer"),
   DATE => todo!("DATE deserializer"),
   DOUBLE => todo!("DOUBLE deserializer"),
   FLOAT => todo!("FLOAT deserializer"),
   INT => todo!("INT deserializer"),
   LIST => todo!("LIST deserializer"),
   LONG => todo!("LONG deserializer"),
   MAP => todo!("MAP deserializer"),
   SET => todo!("SET deserializer"),
   TIMESTAMP => todo!("TIMESTAMP deserializer"),
   UUID => todo!("UUID deserializer"),

   EDGE => todo!("EDGE deserializer"),
   PATH => todo!("PATH deserializer"),
   PROPERTY => todo!("PROPERTY deserializer"),
   TINKER_GRAPH => todo!("TINKER_GRAPH deserializer"),
   VERTEX => todo!("VERTEX deserializer"),
   VERTEX_PROPERTY => todo!("VERTEX_PROPERTY deserializer"),

   BARRIER => todo!("BARRIER deserializer"),
   BINDING => todo!("BINDING deserializer"),
   BULK_SET => todo!("BULK_SET deserializer"),
   BYTECODE => todo!("BYTECODE deserializer"),
   CARDINALITY => todo!("CARDINALITY deserializer"),
   COLUMN => todo!("COLUMN deserializer"),
   DIRECTION => todo!("DIRECTION deserializer"),
   DT => todo!("DT deserializer"),
   LAMBDA => todo!("LAMBDA deserializer"),
   MERGE => todo!("MERGE deserializer"),
   METRICS => todo!("METRICS deserializer"),
   OPERATOR => todo!("OPERATOR deserializer"),
   ORDER => todo!("ORDER deserializer"),
   P => todo!("P deserializer"),
   PICK => todo!("PICK deserializer"),
   POP => todo!("POP deserializer"),
   SCOPE => todo!("SCOPE deserializer"),
   T => todo!("T deserializer"),
   TEXT_P => todo!("TEXT_P deserializer"),
   TRAVERSAL_METRICS => todo!("TRAVERSAL_METRICS deserializer"),
   TRAVERSER => todo!("TRAVERSER deserializer"),
*/

pub use self::core::*;
pub use self::process::*;
pub use self::structure::*;
