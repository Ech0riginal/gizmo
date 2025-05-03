pub(self) use crate::io::graphson::tests::*;
pub(self) use crate::io::graphson::v2::types::*;
pub(self) use std::collections::HashMap;

mod core {
    pub(self) use super::*;

    use ::uuid::Uuid;
    use std::str::FromStr;

    test_prelude!();

    test!(
        class,
        V2,
        Test {
            serial: json!({ "@type" : CLASS, "@value" : "java.io.File"}),
            object: GValue::Class("java.io.File".into()),
        }
    );
    test!(
        date,
        V2,
        Test {
            serial: json!({ "@type" : DATE, "@value" : 1481750076295i64 }),
            object: GValue::Date(chrono::Utc.timestamp_millis_opt(1481750076295i64).unwrap()),
        }
    );
    test!(
        timestamp,
        V2,
        Test {
            serial: json!({ "@type" : TIMESTAMP, "@value" : 1481750076295i64 }),
            object: GValue::Timestamp(chrono::Utc.timestamp_millis_opt(1481750076295i64).unwrap()),
        }
    );
    test!(
        double,
        V2,
        Test {
            serial: json!({ "@type" : DOUBLE, "@value" : 100.0f64 }),
            object: GValue::Double(100.0),
        }
    );
    test!(
        float,
        V2,
        Test {
            serial: json!({ "@type" : FLOAT, "@value" : 100.0f32 }),
            object: GValue::Float(100.0),
        }
    );
    test!(
        integer,
        V2,
        Test {
            serial: json!({ "@type" : INT, "@value" : 100i32 }),
            object: GValue::Int32(100),
        }
    );
    test!(
        long,
        V2,
        Test {
            serial: json!({ "@type" : LONG, "@value" : 100u64 }),
            object: GValue::Int64(100),
        }
    );

    test!(
        uuid,
        V2,
        Test {
            serial: json!({ "@type" : UUID, "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"}),
            object: GValue::Uuid(Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()),
        }
    );
}
mod structure {
    pub(self) use super::*;

    test_prelude!();

    macro_rules! marko {
        () => {
            GValue::Vertex(Vertex {
                id: GID::Int32(1),
                label: "person".into(),
                properties: {
                    let mut tmp = HashMap::new();
                    tmp.insert(
                        "name".into(),
                        vec![VertexProperty {
                            id: GID::Int64(0),
                            label: "name".into(),
                            value: Box::new(GValue::String("marko".into())),
                            vertex: Some(GID::Int32(1)),
                            properties: Default::default(),
                        }],
                    );
                    tmp.insert(
                        "location".into(),
                        vec![
                            VertexProperty {
                                id: GID::Int64(6),
                                value: Box::new(GValue::String("san diego".into())),
                                label: "location".into(),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(1997));
                                    tmp2.insert("endTime".into(), GValue::Int32(2001));
                                    tmp2
                                }),
                            },
                            VertexProperty {
                                id: GID::Int64(7),
                                label: "location".into(),
                                value: Box::new(GValue::String("santa cruz".into())),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(2001));
                                    tmp2.insert("endTime".into(), GValue::Int32(2004));
                                    tmp2
                                }),
                            },
                            VertexProperty {
                                id: GID::Int64(8),
                                label: "location".into(),
                                value: Box::new(GValue::String("brussels".into())),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(2004));
                                    tmp2.insert("endTime".into(), GValue::Int32(2005));
                                    tmp2
                                }),
                            },
                            VertexProperty {
                                id: GID::Int64(9),
                                label: "location".into(),
                                value: Box::new(GValue::String("santa fe".into())),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(2005));
                                    tmp2
                                }),
                            },
                        ],
                    );
                    tmp
                },
            })
        };
    }

    test!(
        edge,
        V2,
        Test {
            serial: json!({ "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } }}),
            object: GValue::Edge(Edge {
                id: GID::Int32(13),
                label: "develops".to_string(),
                in_v: Vertex {
                    id: GID::Int32(10),
                    label: "software".into(),
                    properties: Default::default(),
                },
                out_v: Vertex {
                    id: GID::Int32(1),
                    label: "person".into(),
                    properties: Default::default(),
                },
                properties: [("since".into(), Box::new(GValue::Int32(2009))),].into(),
            }),
        }
    );
    test!(
        path,
        V2,
        Test {
            serial: json!({ "@type" : "g:Path", "@value" : { "labels" : [ [ ], [ ], [ ] ], "objects" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } } ] }}),
            object: GValue::Path(Path::new(
                vec![vec![].into(), vec![].into(), vec![].into()].into(),
                GValue::List(List(vec![
                    GValue::Vertex(Vertex {
                        id: 1i32.into(),
                        label: "person".to_string(),
                        properties: Default::default(),
                    }),
                    GValue::Vertex(Vertex {
                        id: 10i32.into(),
                        label: "software".to_string(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    label: "name".to_string(),
                                    id: 4i64.into(),
                                    value: Box::new(GValue::String("gremlin".into())),
                                    vertex: Some(GID::Int32(10)),
                                    properties: Default::default(),
                                }],
                            );
                            tmp
                        },
                    }),
                    GValue::Vertex(Vertex {
                        id: 11i32.into(),
                        label: "software".to_string(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    id: 5i64.into(),
                                    value: Box::new(GValue::String("tinkergraph".into())),
                                    vertex: Some(GID::Int32(11)),
                                    label: "name".to_string(),
                                    properties: Default::default(),
                                }],
                            );
                            tmp
                        },
                    }),
                ]))
                .into(),
            )),
        }
    );
    test!(
        property,
        V2,
        Test {
            serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 }, "element" : { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "outV" : { "@type" : "g:Int32", "@value" : 1 }, "inV" : { "@type" : "g:Int32", "@value" : 10 } } } }}),
            object: GValue::Property(Property {
                key: "since".into(),
                value: Box::new(GValue::Int32(2009)),
                element: Box::new(GValue::Edge(Edge {
                    id: GID::Int32(13),
                    label: "develops".to_string(),
                    in_v: Vertex {
                        id: GID::Int32(10),
                        label: "software".into(),
                        properties: Default::default(),
                    },
                    out_v: Vertex {
                        id: GID::Int32(1),
                        label: "person".into(),
                        properties: Default::default(),
                    },
                    properties: Default::default()
                }))
            }),
        }
    );
    test!(
        stargraph,
        V2,
        Test {
            serial: json!({ "starVertex" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }}),
            object: GValue::StarGraph(StarGraph {
                id: GID::Int32(1),
                label: "person".into(),
                properties: {
                    let mut tmp = HashMap::new();
                    tmp.insert(
                        "name".into(),
                        vec![VertexProperty {
                            id: GID::Int64(0),
                            label: "name".into(),
                            value: Box::new(GValue::String("marko".into())),
                            vertex: Some(GID::Int32(1)),
                            properties: Default::default(),
                        }],
                    );
                    tmp.insert(
                        "location".into(),
                        vec![
                            VertexProperty {
                                id: GID::Int64(6),
                                value: Box::new(GValue::String("san diego".into())),
                                label: "location".into(),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(1997));
                                    tmp2.insert("endTime".into(), GValue::Int32(2001));
                                    tmp2
                                }),
                            },
                            VertexProperty {
                                id: GID::Int64(7),
                                label: "location".into(),
                                value: Box::new(GValue::String("santa cruz".into())),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(2001));
                                    tmp2.insert("endTime".into(), GValue::Int32(2004));
                                    tmp2
                                }),
                            },
                            VertexProperty {
                                id: GID::Int64(8),
                                label: "location".into(),
                                value: Box::new(GValue::String("brussels".into())),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(2004));
                                    tmp2.insert("endTime".into(), GValue::Int32(2005));
                                    tmp2
                                }),
                            },
                            VertexProperty {
                                id: GID::Int64(9),
                                label: "location".into(),
                                value: Box::new(GValue::String("santa fe".into())),
                                vertex: Some(GID::Int32(1)),
                                properties: Some({
                                    let mut tmp2 = HashMap::new();
                                    tmp2.insert("startTime".into(), GValue::Int32(2005));
                                    tmp2
                                }),
                            },
                        ],
                    );
                    tmp
                },
            }),
        }
    );

    test!(
        tinkergraph,
        V2,
        Test {
            serial: json!({ "@type" : "tinker:graph", "@value" : { "vertices" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 1 }, "value" : "stephen", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 10 }, "value" : "centreville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1990 }, "endTime" : { "@type" : "g:Int32", "@value" : 2000 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 11 }, "value" : "dulles", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2000 }, "endTime" : { "@type" : "g:Int32", "@value" : 2006 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 12 }, "value" : "purcellville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2006 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 2 }, "value" : "matthias", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 13 }, "value" : "bremen", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2007 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 14 }, "value" : "baltimore", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2007 }, "endTime" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 15 }, "value" : "oakland", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2011 }, "endTime" : { "@type" : "g:Int32", "@value" : 2014 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 16 }, "value" : "seattle", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2014 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 3 }, "value" : "daniel", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 17 }, "value" : "spremberg", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1982 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 18 }, "value" : "kaiserslautern", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 }, "endTime" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 19 }, "value" : "aachen", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2009 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } } ], "edges" : [ { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 14 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 15 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 16 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 17 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 18 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 19 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 20 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 21 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2012 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 22 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 23 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 24 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 25 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 26 }, "label" : "traverses", "inVLabel" : "software", "outVLabel" : "software", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::TinkerGraph(TinkerGraph {
                vertices: vec![
                    Vertex {
                        id: GID::Int32(1),
                        label: "person".into(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    id: GID::Int64(0),
                                    label: "name".into(),
                                    value: Box::new(GValue::String("marko".into())),
                                    vertex: Some(GID::Int32(1)),
                                    properties: Default::default(),
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                vec![
                                    VertexProperty {
                                        id: GID::Int64(6),
                                        value: Box::new(GValue::String("san diego".into())),
                                        label: "location".into(),
                                        vertex: Some(GID::Int32(1)),
                                        properties: Some({
                                            let mut tmp2 = HashMap::new();
                                            tmp2.insert("startTime".into(), GValue::Int32(1997));
                                            tmp2.insert("endTime".into(), GValue::Int32(2001));
                                            tmp2
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(7),
                                        label: "location".into(),
                                        value: Box::new(GValue::String("santa cruz".into())),
                                        vertex: Some(GID::Int32(1)),
                                        properties: Some({
                                            let mut tmp2 = HashMap::new();
                                            tmp2.insert("startTime".into(), GValue::Int32(2001));
                                            tmp2.insert("endTime".into(), GValue::Int32(2004));
                                            tmp2
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(8),
                                        label: "location".into(),
                                        value: Box::new(GValue::String("brussels".into())),
                                        vertex: Some(GID::Int32(1)),
                                        properties: Some({
                                            let mut tmp2 = HashMap::new();
                                            tmp2.insert("startTime".into(), GValue::Int32(2004));
                                            tmp2.insert("endTime".into(), GValue::Int32(2005));
                                            tmp2
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(9),
                                        label: "location".into(),
                                        value: Box::new(GValue::String("santa fe".into())),
                                        vertex: Some(GID::Int32(1)),
                                        properties: Some({
                                            let mut tmp2 = HashMap::new();
                                            tmp2.insert("startTime".into(), GValue::Int32(2005));
                                            tmp2
                                        }),
                                    },
                                ],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Int32(7),
                        label: "person".into(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    id: GID::Int64(1),
                                    value: Box::new(GValue::String("stephen".into())),
                                    vertex: Some(GID::Int32(7)),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                vec![
                                    VertexProperty {
                                        id: GID::Int64(10),
                                        value: Box::new(GValue::String("centreville".into())),
                                        vertex: Some(GID::Int32(7)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(1990));
                                            tmp.insert("endTime".into(), GValue::Int32(2000));
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(11),
                                        value: Box::new(GValue::String("dulles".into())),
                                        vertex: Some(GID::Int32(7)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2000));
                                            tmp.insert("endTime".into(), GValue::Int32(2006));
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(12),
                                        value: Box::new(GValue::String("purcellville".into())),
                                        vertex: Some(GID::Int32(7)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2006));
                                            tmp
                                        }),
                                    },
                                ],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Int32(8),
                        label: "person".into(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    id: GID::Int64(2),
                                    value: Box::new(GValue::String("matthias".into())),
                                    vertex: Some(GID::Int32(8)),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                vec![
                                    VertexProperty {
                                        id: GID::Int64(13),
                                        value: Box::new(GValue::String("bremen".into())),
                                        vertex: Some(GID::Int32(8)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2004));
                                            tmp.insert("endTime".into(), GValue::Int32(2007));
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(14),
                                        value: Box::new(GValue::String("baltimore".into())),
                                        vertex: Some(GID::Int32(8)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2007));
                                            tmp.insert("endTime".into(), GValue::Int32(2011));
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(15),
                                        value: Box::new(GValue::String("oakland".into())),
                                        vertex: Some(GID::Int32(8)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2011));
                                            tmp.insert("endTime".into(), GValue::Int32(2014));
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(16),
                                        value: Box::new(GValue::String("seattle".into())),
                                        vertex: Some(GID::Int32(8)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2014));
                                            tmp
                                        }),
                                    },
                                ],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Int32(9),
                        label: "person".into(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    id: GID::Int64(3),
                                    value: Box::new(GValue::String("daniel".into())),
                                    vertex: Some(GID::Int32(9)),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                vec![
                                    VertexProperty {
                                        id: GID::Int64(17),
                                        value: Box::new(GValue::String("spremberg".into())),
                                        vertex: Some(GID::Int32(9)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(1982));
                                            tmp.insert("endTime".into(), GValue::Int32(2005));
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(18),
                                        value: Box::new(GValue::String("kaiserslautern".into())),
                                        vertex: Some(GID::Int32(9)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2005));
                                            tmp.insert("endTime".into(), GValue::Int32(2009));
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Int64(19),
                                        value: Box::new(GValue::String("aachen".into())),
                                        vertex: Some(GID::Int32(9)),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = HashMap::new();
                                            tmp.insert("startTime".into(), GValue::Int32(2009));
                                            tmp
                                        }),
                                    },
                                ],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Int32(10),
                        label: "software".into(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    id: GID::Int64(4),
                                    value: Box::new(GValue::String("gremlin".into())),
                                    vertex: Some(GID::Int32(10)),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Int32(11),
                        label: "software".into(),
                        properties: {
                            let mut tmp = HashMap::new();
                            tmp.insert(
                                "name".into(),
                                vec![VertexProperty {
                                    id: GID::Int64(5),
                                    value: Box::new(GValue::String("tinkergraph".into())),
                                    vertex: Some(GID::Int32(11)),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp
                        },
                    },
                ],
                edges: vec![
                    Edge {
                        id: GID::Int32(13),
                        label: "develops".to_string(),
                        in_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(1),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("since".into(), Box::new(GValue::Int32(2009))),].into(),
                    },
                    Edge {
                        id: GID::Int32(14),
                        label: "develops".to_string(),
                        in_v: Vertex {
                            id: GID::Int32(11),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(1),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("since".into(), Box::new(GValue::Int32(2010))),].into(),
                    },
                    Edge {
                        id: GID::Int32(15),
                        label: "uses".to_string(),
                        in_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(1),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(4))),].into(),
                    },
                    Edge {
                        id: GID::Int32(16),
                        label: "uses".to_string(),
                        in_v: Vertex {
                            id: GID::Int32(11),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(1),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(5))),].into(),
                    },
                    Edge {
                        id: GID::Int32(17),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(7),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("since".into(), Box::new(GValue::Int32(2010))),].into(),
                    },
                    Edge {
                        id: GID::Int32(18),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Int32(11),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(7),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("since".into(), Box::new(GValue::Int32(2011))),].into(),
                    },
                    Edge {
                        id: GID::Int32(19),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(7),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(5))),].into(),
                    },
                    Edge {
                        id: GID::Int32(20),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Int32(11),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(7),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(4))),].into(),
                    },
                    Edge {
                        id: GID::Int32(21),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(8),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("since".into(), Box::new(GValue::Int32(2012))),].into(),
                    },
                    Edge {
                        id: GID::Int32(22),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(8),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(3))),].into(),
                    },
                    Edge {
                        id: GID::Int32(23),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Int32(11),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(8),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(3))),].into(),
                    },
                    Edge {
                        id: GID::Int32(24),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(9),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(5))),].into(),
                    },
                    Edge {
                        id: GID::Int32(25),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Int32(11),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(9),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Int32(3))),].into(),
                    },
                    Edge {
                        id: GID::Int32(26),
                        label: "traverses".into(),
                        in_v: Vertex {
                            id: GID::Int32(11),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Int32(10),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        properties: Default::default(),
                    },
                ],
            }),
        }
    );
    test!(
        tree,
        V2,
        Test {
            serial: json!({ "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ ] } } ] } } ] } } ]}),
            object: GValue::Tree(Tree {
                branches: vec![Branch {
                    key: Box::new(marko!()),
                    value: Box::new(GValue::Tree(Tree {
                        branches: vec![Branch {
                            key: Box::new(GValue::Vertex(Vertex {
                                id: GID::Int32(10),
                                label: "software".into(),
                                properties: {
                                    let mut tmp = HashMap::new();
                                    tmp.insert(
                                        "name".into(),
                                        vec![VertexProperty {
                                            id: GID::Int64(4),
                                            value: Box::new(GValue::String("gremlin".into())),
                                            vertex: Some(GID::Int32(10)),
                                            label: "name".into(),
                                            properties: None,
                                        }],
                                    );
                                    tmp
                                },
                            })),
                            value: Box::new(GValue::Tree(Tree {
                                branches: vec![Branch {
                                    key: Box::new(GValue::Vertex(Vertex {
                                        id: 11i32.into(),
                                        label: "software".to_string(),
                                        properties: {
                                            let mut tmp = HashMap::new();
                                            tmp.insert(
                                                "name".into(),
                                                vec![VertexProperty {
                                                    id: 5i64.into(),
                                                    value: Box::new(GValue::String(
                                                        "tinkergraph".into(),
                                                    )),
                                                    vertex: Some(GID::Int32(11)),
                                                    label: "name".to_string(),
                                                    properties: Default::default(),
                                                }],
                                            );
                                            tmp
                                        },
                                    })),
                                    value: Box::new(GValue::Tree(Tree { branches: vec![] }))
                                }],
                            }))
                        }]
                    })),
                },],
            }),
        }
    );
    test!(
        vertex,
        V2,
        Test {
            serial: json!({ "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } }}),
            object: marko!(),
        }
    );
    test!(
        vertexproperty,
        V2,
        Test {
            serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" }}),
            object: GValue::VertexProperty(VertexProperty {
                id: GID::Int64(0),
                value: Box::new(GValue::String("marko".to_string())),
                vertex: Some(GID::Int32(1)),
                label: "name".into(),
                properties: None,
            }),
        }
    );
}
mod process {
    pub(self) use super::*;

    test_prelude!();

    test!(
        barrier,
        V2,
        Test {
            serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
            object: GValue::Null,
        }
    );
    test!(
        binding,
        V2,
        Test {
            serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
            object: GValue::Null,
        }
    );
    test!(
        bytecode,
        V2,
        Test {
            serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
            object: GValue::Null,
        }
    );
    test!(
        cardinality,
        V2,
        Test {
            serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
            object: GValue::Null,
        }
    );
    test!(
        column,
        V2,
        Test {
            serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
            object: GValue::Null,
        }
    );
    test!(
        direction,
        V2,
        Test {
            serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
            object: GValue::Null,
        }
    );
    test!(
        operator,
        V2,
        Test {
            serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
            object: GValue::Null,
        }
    );
    test!(
        order,
        V2,
        Test {
            serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
            object: GValue::Null,
        }
    );
    test!(
        pick,
        V2,
        Test {
            serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
            object: GValue::Null,
        }
    );
    test!(
        pop,
        V2,
        Test {
            serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
            object: GValue::Null,
        }
    );
    test!(
        lambda,
        V2,
        Test {
            serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
            object: GValue::Null,
        }
    );
    test!(
        metrics,
        V2,
        Test {
            serial: json!({ "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()", "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } } ] }}),
            object: GValue::Null,
        }
    );
    test!(
        p,
        V2,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
            object: GValue::Null,
        }
    );
    test!(
        p_within,
        V2,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
            object: GValue::Null,
        }
    );
    test!(
        p_without,
        V2,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
            object: GValue::Null,
        }
    );
    test!(
        p_and,
        V2,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        }
    );
    test!(
        p_or,
        V2,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } ] }}),
            object: GValue::Null,
        }
    );
    test!(
        scope,
        V2,
        Test {
            serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
            object: GValue::Null,
        }
    );
    test!(
        t,
        V2,
        Test {
            serial: json!({ "@type" : "g:T", "@value" : "label"}),
            object: GValue::Null,
        }
    );
    test!(
        textp,
        V2,
        Test {
            serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
            object: GValue::Null,
        }
    );
    test!(
        traversalmetrics,
        V2,
        Test {
            serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 0.004 }, "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 13 }, "elementCount" : { "@type" : "g:Int64", "@value" : 13 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "2.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 1 }, "elementCount" : { "@type" : "g:Int64", "@value" : 1 } }, "name" : "TreeStep", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "4.0.0()" } } ] }}),
            object: GValue::Null,
        }
    );
    test!(
        traverser,
        V2,
        Test {
            serial: json!({ "@type" : "g:Traverser", "@value" : { "bulk" : { "@type" : "g:Int64", "@value" : 1 }, "value" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } }}),
            object: GValue::Null,
        }
    );
}
mod request {
    pub(self) use super::*;

    test_prelude!();

    test!(
        authentication_response,
        V2,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "authentication", "processor" : "", "args" : { "saslMechanism" : "PLAIN", "sasl" : "AHN0ZXBocGhlbgBwYXNzd29yZA==" }}),
            object: GValue::Null,
        }
    );
    test!(
        session_eval,
        V2,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        }
    );
    test!(
        session_eval_aliased,
        V2,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        }
    );
    test!(
        session_close,
        V2,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "close", "processor" : "session", "args" : { "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" } }}),
            object: GValue::Null,
        }
    );
    test!(
        sessionless_eval,
        V2,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        }
    );
    test!(
        sessionless_eval_aliased,
        V2,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        }
    );
}
mod response {
    pub(self) use super::*;

    test_prelude!();

    test!(
        authentication_challenge,
        V2,
        Test {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { } }, "result" : { "data" : null, "meta" : { } }}),
            object: GValue::Null,
        }
    );
    test!(
        standard_result,
        V2,
        Test {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 200, "attributes" : { } }, "result" : { "data" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } ], "meta" : { } }}),
            object: GValue::Null,
        }
    );
}
mod extended {
    pub(self) use super::*;

    test_prelude!();

    test!(
        bigdecimal,
        V2,
        Test {
            serial: json!({ "@type" : "gx:BigDecimal", "@value" : 123456789987654321123456789987654321u128}),
            object: GValue::Null,
        }
    );
    test!(
        biginteger,
        V2,
        Test {
            serial: json!({ "@type" : "gx:BigInteger", "@value" : 123456789987654321123456789987654321u128 }),
            object: GValue::Null,
        }
    );
    test!(
        byte,
        V2,
        Test {
            serial: json!({ "@type" : "gx:Byte", "@value" : 1}),
            object: GValue::Null,
        }
    );
    test!(
        bytebuffer,
        V2,
        Test {
            serial: json!({ "@type" : "gx:ByteBuffer", "@value" : "c29tZSBieXRlcyBmb3IgeW91"}),
            object: GValue::Null,
        }
    );
    test!(
        char,
        V2,
        Test {
            serial: json!({ "@type" : "gx:Char", "@value" : "x"}),
            object: GValue::Null,
        }
    );
    test!(
        duration,
        V2,
        Test {
            serial: json!({ "@type" : "gx:Duration", "@value" : "PT120H"}),
            object: GValue::Null,
        }
    );
    test!(
        inetaddress,
        V2,
        Test {
            serial: json!({ "@type" : "gx:InetAddress", "@value" : "localhost"}),
            object: GValue::Null,
        }
    );
    test!(
        instant,
        V2,
        Test {
            serial: json!({ "@type" : "gx:Instant", "@value" : "2016-12-14T16:39:19.349Z"}),
            object: GValue::Null,
        }
    );
    test!(
        localdate,
        V2,
        Test {
            serial: json!({ "@type" : "gx:LocalDate", "@value" : "2016-01-01"}),
            object: GValue::Null,
        }
    );
    test!(
        localdatetime,
        V2,
        Test {
            serial: json!({ "@type" : "gx:LocalDateTime", "@value" : "2016-01-01T12:30"}),
            object: GValue::Null,
        }
    );
    test!(
        localtime,
        V2,
        Test {
            serial: json!({ "@type" : "gx:LocalTime", "@value" : "12:30:45"}),
            object: GValue::Null,
        }
    );
    test!(
        monthday,
        V2,
        Test {
            serial: json!({ "@type" : "gx:MonthDay", "@value" : "--01-01"}),
            object: GValue::Null,
        }
    );
    test!(
        offsetdatetime,
        V2,
        Test {
            serial: json!({ "@type" : "gx:OffsetDateTime", "@value" : "2007-12-03T10:15:30+01:00"}),
            object: GValue::Null,
        }
    );
    test!(
        offsettime,
        V2,
        Test {
            serial: json!({ "@type" : "gx:OffsetTime", "@value" : "10:15:30+01:00"}),
            object: GValue::Null,
        }
    );
    test!(
        period,
        V2,
        Test {
            serial: json!({ "@type" : "gx:Period", "@value" : "P1Y6M15D"}),
            object: GValue::Null,
        }
    );
    test!(
        short,
        V2,
        Test {
            serial: json!({ "@type" : "gx:Int16", "@value" : 100}),
            object: GValue::Null,
        }
    );
    test!(
        year,
        V2,
        Test {
            serial: json!({ "@type" : "gx:Year", "@value" : "2016"}),
            object: GValue::Null,
        }
    );
    test!(
        yearmonth,
        V2,
        Test {
            serial: json!({ "@type" : "gx:YearMonth", "@value" : "2016-06"}),
            object: GValue::Null,
        }
    );
    test!(
        zoneddatetime,
        V2,
        Test {
            serial: json!({ "@type" : "gx:ZonedDateTime", "@value" : "2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}),
            object: GValue::Null,
        }
    );
    test!(
        zoneoffset,
        V2,
        Test {
            serial: json!({ "@type" : "gx:ZoneOffset", "@value" : "+03:06:09"}),
            object: GValue::Null,
        }
    );
}
