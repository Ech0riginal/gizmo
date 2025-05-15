pub use crate::io::graphson::V2;
pub use crate::io::graphson::tags::*;
pub use crate::io::*;
pub use crate::structure::*;
pub use crate::*;
pub use chrono::TimeZone;
pub use serde_json::json;
pub use std::collections::HashMap;
pub use std::str::FromStr;

mod core {
    use super::*;

    gvalue_test!(
        class,
        V3,
        Test {
            serial: json!({ "@type" : CLASS, "@value" : "java.io.File"}),
            object: GValue::Class("java.io.File".into()),
        }
    );
    gvalue_test!(
        date,
        V3,
        Test {
            serial: json!({ "@type" : DATE, "@value" : 1481750076295i64 }),
            object: GValue::Date(Date(
                chrono::Utc.timestamp_millis_opt(1481750076295i64).unwrap()
            )),
        }
    );
    gvalue_test!(
        double,
        V3,
        Test {
            serial: json!({ "@type" : DOUBLE, "@value" : 100.0f64 }),
            object: Double(100.0).into(),
        }
    );
    gvalue_test!(
        float,
        V3,
        Test {
            serial: json!({ "@type" : FLOAT, "@value" : 100.0f32 }),
            object: Float(100.0).into(),
        }
    );
    gvalue_test!(
        integer,
        V3,
        Test {
            serial: json!({ "@type" : INT, "@value" : 100i32 }),
            object: Integer(100).into(),
        }
    );
    gvalue_test!(
        list,
        V3,
        Test {
            serial: json!({ "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, "person", true ]}),
            object: GValue::List(vec![GValue::Integer(Integer(1))].into()),
        }
    );
    gvalue_test!(
        long,
        V3,
        Test {
            serial: json!({ "@type" : LONG, "@value" : 100u64 }),
            object: Long(100).into(),
        }
    );
    gvalue_test!(
        map,
        V3,
        Test {
            serial: json!({ "@type" : "g:Map", "@value" : [
                {"@type" : "g:Date", "@value" : 1481750076295u64 },
                "red",
                {
                    "@type" : "g:List",
                    "@value" : [
                    { "@type" : "g:Int32", "@value" : 1 },
                    { "@type" : "g:Int32", "@value" : 2 },
                    { "@type" : "g:Int32", "@value" : 3 }
                ]
                }, { "@type" : "g:Date", "@value" : 1481750076295u64 }, "test", { "@type" : "g:Int32", "@value" : 123 } ]}),
            object: GValue::Map(Map([
                ("label".into(), GValue::String(String::from("person"))),
                (
                    "name".into(),
                    GValue::List(vec![String::from("marko").into()].into()),
                ),
            ]
            .into())),
        }
    );
    gvalue_test!(
        set,
        V3,
        Test {
            serial: json!({ "@type" : "g:Set", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, "person", true ]}),
            object: GValue::Set(
                vec![
                    GValue::Integer(Integer(1)),
                    GValue::String("person".into()),
                    GValue::Bool(Bool(true)),
                ]
                .into()
            ),
        }
    );
    gvalue_test!(
        timestamp,
        V3,
        Test {
            serial: json!({ "@type" : "g:Timestamp", "@value" : 1481750076295i64 }),
            object: GValue::Timestamp(Timestamp(1481750076295i64)),
        }
    );
    gvalue_test!(
        uuid,
        V3,
        Test {
            serial: json!({ "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"}),
            object: GValue::Uuid(Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()),
        }
    );
}
mod structure {
    use super::*;

    gvalue_test!(
        edge,
        V3,
        Test {
            serial: json!({ "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } } } } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        path,
        V3,
        Test {
            serial: json!({ "@type" : "g:Path", "@value" : { "labels" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Set", "@value" : [ ] }, { "@type" : "g:Set", "@value" : [ ] }, { "@type" : "g:Set", "@value" : [ ] } ] }, "objects" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software" } } ] } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        property,
        V3,
        Test {
            serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        tinkergraph,
        V3,
        Test {
            serial: json!({ "@type" : "tinker:graph", "@value" : { "vertices" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 1 }, "value" : "stephen", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 10 }, "value" : "centreville", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1990 }, "endTime" : { "@type" : "g:Int32", "@value" : 2000 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 11 }, "value" : "dulles", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2000 }, "endTime" : { "@type" : "g:Int32", "@value" : 2006 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 12 }, "value" : "purcellville", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2006 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 2 }, "value" : "matthias", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 13 }, "value" : "bremen", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2007 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 14 }, "value" : "baltimore", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2007 }, "endTime" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 15 }, "value" : "oakland", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2011 }, "endTime" : { "@type" : "g:Int32", "@value" : 2014 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 16 }, "value" : "seattle", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2014 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 3 }, "value" : "daniel", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 17 }, "value" : "spremberg", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1982 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 18 }, "value" : "kaiserslautern", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 }, "endTime" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 19 }, "value" : "aachen", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2009 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "label" : "name" } } ] } } } ], "edges" : [ { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 14 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2010 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 15 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 4 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 16 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 17 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2010 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 18 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2011 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 19 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 20 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 4 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 21 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2012 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 22 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 23 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 24 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 25 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 26 }, "label" : "traverses", "inVLabel" : "software", "outVLabel" : "software", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        vertex,
        V3,
        Test {
            serial: json!({ "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        vertexproperty,
        V3,
        Test {
            serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" }}),
            object: GValue::Null,
        }
    );
}
mod process {
    use super::*;

    gvalue_test!(
        barrier,
        V3,
        Test {
            serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        binding,
        V3,
        Test {
            serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        bulkset,
        V3,
        Test {
            serial: json!({ "@type" : "g:BulkSet", "@value" : [ "marko", { "@type" : "g:Int64", "@value" : 1 }, "josh", { "@type" : "g:Int64", "@value" : 2 } ]}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        bytecode,
        V3,
        Test {
            serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        cardinality,
        V3,
        Test {
            serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        column,
        V3,
        Test {
            serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        direction,
        V3,
        Test {
            serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        operator,
        V3,
        Test {
            serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        order,
        V3,
        Test {
            serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        pick,
        V3,
        Test {
            serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        pop,
        V3,
        Test {
            serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        lambda,
        V3,
        Test {
            serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        metrics,
        V3,
        Test {
            serial: json!({ "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 4 }, "elementCount", { "@type" : "g:Int64", "@value" : 4 } ] }, "name", "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "7.0.0()", "metrics", { "@type" : "g:List", "@value" : [ { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 7 }, "elementCount", { "@type" : "g:Int64", "@value" : 7 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "3.0.0()" ] } } ] } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p,
        V3,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_within,
        V3,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_without,
        V3,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_and,
        V3,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_or,
        V3,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        scope,
        V3,
        Test {
            serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        t,
        V3,
        Test {
            serial: json!({ "@type" : "g:T", "@value" : "label"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        textp,
        V3,
        Test {
            serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        traversalmetrics,
        V3,
        Test {
            serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 0.004 }, "metrics", { "@type" : "g:List", "@value" : [ { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 4 }, "elementCount", { "@type" : "g:Int64", "@value" : 4 } ] }, "name", "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "7.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 13 }, "elementCount", { "@type" : "g:Int64", "@value" : 13 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "2.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 7 }, "elementCount", { "@type" : "g:Int64", "@value" : 7 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "3.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 1 }, "elementCount", { "@type" : "g:Int64", "@value" : 1 } ] }, "name", "TreeStep", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "4.0.0()" ] } } ] } ] }}),
            object: GValue::TraversalMetrics(TraversalMetrics::new(
                Double(0.004),
                vec![
                    Metrics::new(
                        "7.0.0()",
                        "TinkerGraphStep(vertex,[~label.eq(person)])",
                        100.0,
                        4,
                        4,
                        25.0,
                        vec![],
                    ),
                    Metrics::new(
                        "2.0.0()",
                        "VertexStep(OUT,vertex)",
                        100.0,
                        13,
                        13,
                        25.0,
                        vec![],
                    ),
                    Metrics::new(
                        "3.0.0()",
                        "VertexStep(OUT,vertex)",
                        100.0,
                        7,
                        7,
                        25.0,
                        vec![],
                    ),
                    Metrics::new("4.0.0()", "TreeStep", 100.0, 1, 1, 25.0, vec![]),
                ],
            )),
        }
    );
    gvalue_test!(
        traverser,
        V3,
        Test {
            serial: json!({ "@type" : "g:Traverser", "@value" : { "bulk" : { "@type" : "g:Int64", "@value" : 1 }, "value" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } }}),
            object: GValue::Null,
        }
    );
}
mod request {
    use super::*;

    request_test!(
        authentication_response,
        V3,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "authentication", "processor" : "", "args" : { "saslMechanism" : "PLAIN", "sasl" : "AHN0ZXBocGhlbgBwYXNzd29yZA==" }}),
            object: Request {
                id: Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "authentication",
                proc: "",
                args: Args::new()
                    .arg("saslMechanism", "PLAIN")
                    .arg("sasl", "AHN0ZXBocGhlbgBwYXNzd29yZA=="),
            },
        }
    );
    request_test!(
        session_eval,
        V3,
        Test {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "g.V(x)", "language", "gremlin-groovy", "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: Request {
                id: Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "session",
                args: Args::new()
                    .arg("gremlin", "g.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg(
                        "session",
                        GValue::Uuid(
                            Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
                        )
                    )
                    .arg("bindings", {
                        let mut tmp = HashMap::new();
                        tmp.insert("x", GValue::Integer(1.into()));
                        tmp
                    }),
            },
        }
    );
    request_test!(
        session_eval_aliased,
        V3,
        Test {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "social.V(x)", "language", "gremlin-groovy", "aliases", { "@type" : "g:Map", "@value" : [ "g", "social" ] }, "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "session",
                args: Args::new()
                    .arg("gremlin", "social.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg("aliases", {
                        let mut tmp = HashMap::new();
                        tmp.insert("g", GValue::String("social".into()));
                        GValue::Map(Map::from(tmp))
                    })
                    .arg(
                        "session",
                        GValue::Uuid(
                            ::uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
                        )
                    )
                    .arg("bindings", {
                        let mut tmp = HashMap::new();
                        tmp.insert("x", GValue::Integer(1.into()));
                        GValue::Map(Map::from(tmp))
                    }),
            },
        }
    );
    request_test!(
        session_close,
        V3,
        Test {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "close", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" } ] }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "close",
                proc: "session",
                args: Args::new().arg(
                    "session",
                    GValue::Uuid(
                        ::uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
                    )
                )
            },
        }
    );
    request_test!(
        sessionless_eval,
        V3,
        Test {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "g.V(x)", "language", "gremlin-groovy", "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "",
                args: Args::new()
                    .arg("gremlin", "g.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg("bindings", {
                        let mut tmp = HashMap::new();
                        tmp.insert("x", GValue::Integer(1.into()));
                        GValue::Map(Map::from(tmp))
                    }),
            },
        }
    );
    request_test!(
        sessionless_eval_aliased,
        V3,
        Test {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "social.V(x)", "language", "gremlin-groovy", "aliases", { "@type" : "g:Map", "@value" : [ "g", "social" ] }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "",
                args: Args::new()
                    .arg("gremlin", "social.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg("aliases", {
                        let mut tmp = HashMap::new();
                        tmp.insert("g", GValue::from("social"));
                        GValue::Map(Map::from(tmp))
                    })
                    .arg("bindings", {
                        let mut tmp = HashMap::new();
                        tmp.insert("x", GValue::Integer(1.into()));
                        GValue::Map(Map::from(tmp))
                    }),
            },
        }
    );
}
mod response {
    use super::*;

    response_test!(
        authentication_challenge,
        V3,
        Test {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { "@type" : "g:Map", "@value" : [ ] } }, "result" : { "data" : null, "meta" : { "@type" : "g:Map", "@value" : [ ] } }}),
            object: crate::Response {
                id: uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap(),
                status: crate::Status {
                    code: 407,
                    message: Default::default(),
                    attributes: serde_json::Value::Object(serde_json::Map::new()),
                },
                data: GValue::Null,
                meta: HashMap::new(),
            },
        }
    );
    response_test!(
        standard_result,
        V3,
        Test {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 200, "attributes" : { "@type" : "g:Map", "@value" : [ ] } }, "result" : { "data" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } ] }, "meta" : { "@type" : "g:Map", "@value" : [ ] } }}),
            object: crate::Response {
                id: uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap(),
                status: crate::Status {
                    code: 200,
                    message: None,
                    attributes: serde_json::Value::Object(serde_json::Map::new()),
                },
                meta: HashMap::new(),
                data: GValue::List(List(vec![GValue::Vertex(Vertex {
                    id: GID::Integer(1.into()),
                    label: "person".into(),
                    properties: {
                        let mut tmp = HashMap::new();
                        tmp.insert(
                            "name".into(),
                            vec![VertexProperty {
                                id: GID::Long(0.into()),
                                label: "name".into(),
                                value: Box::new(GValue::String("marko".into())),
                                vertex: Some(GID::Integer(1.into())),
                                properties: Default::default(),
                            }],
                        );
                        tmp.insert(
                            "location".into(),
                            vec![
                                VertexProperty {
                                    id: GID::Long(6.into()),
                                    value: Box::new(GValue::String("san diego".into())),
                                    label: "location".into(),
                                    vertex: Some(GID::Integer(1.into())),
                                    properties: Some({
                                        let mut tmp2 = HashMap::new();
                                        tmp2.insert(
                                            "startTime".into(),
                                            GValue::Integer(1997.into()),
                                        );
                                        tmp2.insert("endTime".into(), GValue::Integer(2001.into()));
                                        tmp2
                                    }),
                                },
                                VertexProperty {
                                    id: GID::Long(7.into()),
                                    label: "location".into(),
                                    value: Box::new(GValue::String("santa cruz".into())),
                                    vertex: Some(GID::Integer(1.into())),
                                    properties: Some({
                                        let mut tmp2 = HashMap::new();
                                        tmp2.insert(
                                            "startTime".into(),
                                            GValue::Integer(2001.into()),
                                        );
                                        tmp2.insert("endTime".into(), GValue::Integer(2004.into()));
                                        tmp2
                                    }),
                                },
                                VertexProperty {
                                    id: GID::Long(8.into()),
                                    label: "location".into(),
                                    value: Box::new(GValue::String("brussels".into())),
                                    vertex: Some(GID::Integer(1.into())),
                                    properties: Some({
                                        let mut tmp2 = HashMap::new();
                                        tmp2.insert(
                                            "startTime".into(),
                                            GValue::Integer(2004.into()),
                                        );
                                        tmp2.insert("endTime".into(), GValue::Integer(2005.into()));
                                        tmp2
                                    }),
                                },
                                VertexProperty {
                                    id: GID::Long(9.into()),
                                    label: "location".into(),
                                    value: Box::new(GValue::String("santa fe".into())),
                                    vertex: Some(GID::Integer(1.into())),
                                    properties: Some({
                                        let mut tmp2 = HashMap::new();
                                        tmp2.insert(
                                            "startTime".into(),
                                            GValue::Integer(2005.into()),
                                        );
                                        tmp2
                                    }),
                                },
                            ],
                        );
                        tmp
                    },
                })]))
            },
        }
    );
}
// mod extended {
//     use super::*;
//
//     test!(
//         bigdecimal,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:BigDecimal", "@value" : 123456789987654321123456789987654321u128 }),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         biginteger,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:BigInteger", "@value" : 123456789987654321123456789987654321u128 }),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         byte,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:Byte", "@value" : 1}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         bytebuffer,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:ByteBuffer", "@value" : "c29tZSBieXRlcyBmb3IgeW91"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         char,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:Char", "@value" : "x"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         duration,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:Duration", "@value" : "PT120H"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         inetaddress,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:InetAddress", "@value" : "localhost"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         instant,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:Instant", "@value" : "2016-12-14T16:39:19.349Z"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         localdate,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:LocalDate", "@value" : "2016-01-01"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         localdatetime,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:LocalDateTime", "@value" : "2016-01-01T12:30"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         localtime,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:LocalTime", "@value" : "12:30:45"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         monthday,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:MonthDay", "@value" : "--01-01"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         offsetdatetime,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:OffsetDateTime", "@value" : "2007-12-03T10:15:30+01:00"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         offsettime,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:OffsetTime", "@value" : "10:15:30+01:00"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         period,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:Period", "@value" : "P1Y6M15D"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         short,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:Int16", "@value" : 100}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         year,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:Year", "@value" : "2016"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         yearmonth,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:YearMonth", "@value" : "2016-06"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         zoneddatetime,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:ZonedDateTime", "@value" : "2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}),
//             object: GValue::Null,
//         }
//     );
//     test!(
//         zoneoffset,
//         V3,
//         Test {
//             serial: json!({ "@type" : "gx:ZoneOffset", "@value" : "+03:06:09"}),
//             object: GValue::Null,
//         }
//     );
// }
