pub(self) use super::macros::*;
pub use crate::GraphSON;
pub use crate::graphson::tags::*;
pub use crate::*;
pub use chrono::TimeZone;
pub use serde_json::json;
pub use std::str::FromStr;

mod core {
    use super::*;

    gvalue_test!(
        class,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Class, "@value" : "java.io.File"}),
            object: GValue::Class("java.io.File".into()),
        }
    );

    gvalue_test!(
        date,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Date, "@value" : 1481750076295i64 }),
            object: GValue::Date(Date(
                chrono::Utc.timestamp_millis_opt(1481750076295i64).unwrap()
            )),
        }
    );
    gvalue_test!(
        timestamp,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Timestamp, "@value" : 1481750076295i64 }),
            object: GValue::Timestamp(Timestamp(1481750076295i64)),
        }
    );
    gvalue_test!(
        double,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Double, "@value" : 100.0f64 }),
            object: GValue::Double(100.0.into()),
        }
    );
    gvalue_test!(
        float,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Float, "@value" : 100.0f32 }),
            object: GValue::Float(100.0.into()),
        }
    );
    gvalue_test!(
        integer,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Integer, "@value" : 100i32 }),
            object: GValue::Integer(100.into()),
        }
    );
    gvalue_test!(
        long,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Long, "@value" : 100u64 }),
            object: GValue::Long(100.into()),
        }
    );
    gvalue_test!(
        uuid,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Uuid, "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"}),
            object: GValue::Uuid(
                ::uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
            ),
        }
    );
}
mod structure {
    use super::*;

    macro_rules! marko {
        () => {
            GValue::Vertex(Vertex {
                id: GID::Integer(1.into()),
                label: "person".into(),
                properties: {
                    let mut tmp = Map::<String, List<VertexProperty>>::new();
                    tmp.insert(
                        "name".into(),
                        list![VertexProperty {
                            id: GID::Long(0.into()),
                            label: "name".into(),
                            value: Box::new(GValue::String("marko".into())),
                            vertex: Some(GID::Integer(1.into())),
                            properties: None,
                        }],
                    );
                    tmp.insert(
                        "location".into(),
                        list![
                            VertexProperty {
                                id: GID::Long(6.into()),
                                value: Box::new(GValue::String("san diego".into())),
                                label: "location".into(),
                                vertex: Some(GID::Integer(1.into())),
                                properties: Some({
                                    let mut tmp2 = Map::<String, GValue>::new();
                                    tmp2.insert("startTime".into(), GValue::Integer(1997.into()));
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
                                    let mut tmp2 = Map::<String, GValue>::new();
                                    tmp2.insert("startTime".into(), GValue::Integer(2001.into()));
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
                                    let mut tmp2 = Map::<String, GValue>::new();
                                    tmp2.insert("startTime".into(), GValue::Integer(2004.into()));
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
                                    let mut tmp2 = Map::<String, GValue>::new();
                                    tmp2.insert("startTime".into(), GValue::Integer(2005.into()));
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

    gvalue_test!(
        edge,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } }}),
            object: GValue::Edge(Edge {
                id: GID::Integer(13.into()),
                label: "develops".to_string(),
                in_v: Vertex {
                    id: GID::Integer(10.into()),
                    label: "software".into(),
                    properties: Map::new(),
                },
                out_v: Vertex {
                    id: GID::Integer(1.into()),
                    label: "person".into(),
                    properties: Map::new(),
                },
                properties: [("since".into(), Box::new(GValue::Integer(2009.into()))),].into(),
            }),
        }
    );
    gvalue_test!(
        path,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({"@type":"g:Path","@value":{"labels":[[],[],[]],"objects":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":4},"value":"gremlin","vertex":{"@type":"g:Int32","@value":10},"label":"name"}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":5},"value":"tinkergraph","vertex":{"@type":"g:Int32","@value":11},"label":"name"}}]}}}]}}),
            object: GValue::Path(Path {
                labels: Box::new(GValue::List(List(vec![
                    GValue::List(list![]),
                    GValue::List(list![]),
                    GValue::List(list![])
                ]))),

                objects: Box::new(GValue::List(list![
                    GValue::Vertex(Vertex {
                        id: 1i32.into(),
                        label: "person".to_string(),
                        properties: Map::new(),
                    }),
                    GValue::Vertex(Vertex {
                        id: 10i32.into(),
                        label: "software".to_string(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    label: "name".to_string(),
                                    id: 4i64.into(),
                                    value: Box::new(GValue::String("gremlin".into())),
                                    vertex: Some(GID::Integer(10.into())),
                                    properties: None,
                                }],
                            );
                            tmp
                        },
                    }),
                    GValue::Vertex(Vertex {
                        id: 11i32.into(),
                        label: "software".to_string(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    id: 5i64.into(),
                                    value: Box::new(GValue::String("tinkergraph".into())),
                                    vertex: Some(GID::Integer(11.into())),
                                    label: "name".to_string(),
                                    properties: None,
                                }],
                            );
                            tmp
                        },
                    }),
                ]))
            }),
        }
    );
    gvalue_test!(
        property,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 }, "element" : { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "outV" : { "@type" : "g:Int32", "@value" : 1 }, "inV" : { "@type" : "g:Int32", "@value" : 10 } } } }}),
            object: GValue::Property(Property {
                key: "since".into(),
                value: Box::new(GValue::Integer(2009.into())),
                element: Box::new(GValue::Edge(Edge {
                    id: GID::Integer(13.into()),
                    label: "develops".to_string(),
                    in_v: Vertex {
                        id: GID::Integer(10.into()),
                        label: "software".into(),
                        properties: Map::new(),
                    },
                    out_v: Vertex {
                        id: GID::Integer(1.into()),
                        label: "person".into(),
                        properties: Map::new(),
                    },
                    properties: Map::new()
                }))
            }),
        }
    );
    gvalue_test!(
        stargraph,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({"starVertex":{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","vertex":{"@type":"g:Int32","@value":1},"label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}}),
            object: GValue::StarGraph(StarGraph::from(match marko!() {
                GValue::Vertex(v) => v,
                _ => panic!("at the disco!"),
            })),
        }
    );

    gvalue_test!(
        tinkergraph,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "tinker:graph", "@value" : { "vertices" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 1 }, "value" : "stephen", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 10 }, "value" : "centreville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1990 }, "endTime" : { "@type" : "g:Int32", "@value" : 2000 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 11 }, "value" : "dulles", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2000 }, "endTime" : { "@type" : "g:Int32", "@value" : 2006 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 12 }, "value" : "purcellville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2006 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 2 }, "value" : "matthias", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 13 }, "value" : "bremen", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2007 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 14 }, "value" : "baltimore", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2007 }, "endTime" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 15 }, "value" : "oakland", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2011 }, "endTime" : { "@type" : "g:Int32", "@value" : 2014 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 16 }, "value" : "seattle", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2014 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 3 }, "value" : "daniel", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 17 }, "value" : "spremberg", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1982 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 18 }, "value" : "kaiserslautern", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 }, "endTime" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 19 }, "value" : "aachen", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2009 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } } ], "edges" : [ { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 14 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 15 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 16 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 17 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 18 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 19 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 20 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 21 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2012 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 22 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 23 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 24 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 25 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 26 }, "label" : "traverses", "inVLabel" : "software", "outVLabel" : "software", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::TinkerGraph(TinkerGraph {
                vertices: list![
                    Vertex {
                        id: GID::Integer(1.into()),
                        label: "person".into(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    id: GID::Long(0.into()),
                                    label: "name".into(),
                                    value: Box::new(GValue::String("marko".into())),
                                    vertex: Some(GID::Integer(1.into())),
                                    properties: None,
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                list![
                                    VertexProperty {
                                        id: GID::Long(6.into()),
                                        value: Box::new(GValue::String("san diego".into())),
                                        label: "location".into(),
                                        vertex: Some(GID::Integer(1.into())),
                                        properties: Some({
                                            let mut tmp2 = Map::<String, GValue>::new();
                                            tmp2.insert(
                                                "startTime".into(),
                                                GValue::Integer(1997.into()),
                                            );
                                            tmp2.insert(
                                                "endTime".into(),
                                                GValue::Integer(2001.into()),
                                            );
                                            tmp2
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(7.into()),
                                        label: "location".into(),
                                        value: Box::new(GValue::String("santa cruz".into())),
                                        vertex: Some(GID::Integer(1.into())),
                                        properties: Some({
                                            let mut tmp2 = Map::<String, GValue>::new();
                                            tmp2.insert(
                                                "startTime".into(),
                                                GValue::Integer(2001.into()),
                                            );
                                            tmp2.insert(
                                                "endTime".into(),
                                                GValue::Integer(2004.into()),
                                            );
                                            tmp2
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(8.into()),
                                        label: "location".into(),
                                        value: Box::new(GValue::String("brussels".into())),
                                        vertex: Some(GID::Integer(1.into())),
                                        properties: Some({
                                            let mut tmp2 = Map::<String, GValue>::new();
                                            tmp2.insert(
                                                "startTime".into(),
                                                GValue::Integer(2004.into()),
                                            );
                                            tmp2.insert(
                                                "endTime".into(),
                                                GValue::Integer(2005.into()),
                                            );
                                            tmp2
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(9.into()),
                                        label: "location".into(),
                                        value: Box::new(GValue::String("santa fe".into())),
                                        vertex: Some(GID::Integer(1.into())),
                                        properties: Some({
                                            let mut tmp2 = Map::<String, GValue>::new();
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
                    },
                    Vertex {
                        id: GID::Integer(7.into()),
                        label: "person".into(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    id: GID::Long(1.into()),
                                    value: Box::new(GValue::String("stephen".into())),
                                    vertex: Some(GID::Integer(7.into())),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                list![
                                    VertexProperty {
                                        id: GID::Long(10.into()),
                                        value: Box::new(GValue::String("centreville".into())),
                                        vertex: Some(GID::Integer(7.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(1990.into()),
                                            );
                                            tmp.insert(
                                                "endTime".into(),
                                                GValue::Integer(2000.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(11.into()),
                                        value: Box::new(GValue::String("dulles".into())),
                                        vertex: Some(GID::Integer(7.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2000.into()),
                                            );
                                            tmp.insert(
                                                "endTime".into(),
                                                GValue::Integer(2006.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(12.into()),
                                        value: Box::new(GValue::String("purcellville".into())),
                                        vertex: Some(GID::Integer(7.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2006.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                ],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Integer(8.into()),
                        label: "person".into(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    id: GID::Long(2.into()),
                                    value: Box::new(GValue::String("matthias".into())),
                                    vertex: Some(GID::Integer(8.into())),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                list![
                                    VertexProperty {
                                        id: GID::Long(13.into()),
                                        value: Box::new(GValue::String("bremen".into())),
                                        vertex: Some(GID::Integer(8.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2004.into()),
                                            );
                                            tmp.insert(
                                                "endTime".into(),
                                                GValue::Integer(2007.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(14.into()),
                                        value: Box::new(GValue::String("baltimore".into())),
                                        vertex: Some(GID::Integer(8.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2007.into()),
                                            );
                                            tmp.insert(
                                                "endTime".into(),
                                                GValue::Integer(2011.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(15.into()),
                                        value: Box::new(GValue::String("oakland".into())),
                                        vertex: Some(GID::Integer(8.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2011.into()),
                                            );
                                            tmp.insert(
                                                "endTime".into(),
                                                GValue::Integer(2014.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(16.into()),
                                        value: Box::new(GValue::String("seattle".into())),
                                        vertex: Some(GID::Integer(8.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2014.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                ],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Integer(9.into()),
                        label: "person".into(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    id: GID::Long(3.into()),
                                    value: Box::new(GValue::String("daniel".into())),
                                    vertex: Some(GID::Integer(9.into())),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp.insert(
                                "location".into(),
                                list![
                                    VertexProperty {
                                        id: GID::Long(17.into()),
                                        value: Box::new(GValue::String("spremberg".into())),
                                        vertex: Some(GID::Integer(9.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(1982.into()),
                                            );
                                            tmp.insert(
                                                "endTime".into(),
                                                GValue::Integer(2005.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(18.into()),
                                        value: Box::new(GValue::String("kaiserslautern".into())),
                                        vertex: Some(GID::Integer(9.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2005.into()),
                                            );
                                            tmp.insert(
                                                "endTime".into(),
                                                GValue::Integer(2009.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                    VertexProperty {
                                        id: GID::Long(19.into()),
                                        value: Box::new(GValue::String("aachen".into())),
                                        vertex: Some(GID::Integer(9.into())),
                                        label: "location".into(),
                                        properties: Some({
                                            let mut tmp = Map::<String, GValue>::new();
                                            tmp.insert(
                                                "startTime".into(),
                                                GValue::Integer(2009.into()),
                                            );
                                            tmp
                                        }),
                                    },
                                ],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Integer(10.into()),
                        label: "software".into(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    id: GID::Long(4.into()),
                                    value: Box::new(GValue::String("gremlin".into())),
                                    vertex: Some(GID::Integer(10.into())),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp
                        },
                    },
                    Vertex {
                        id: GID::Integer(11.into()),
                        label: "software".into(),
                        properties: {
                            let mut tmp = Map::<String, List<VertexProperty>>::new();
                            tmp.insert(
                                "name".into(),
                                list![VertexProperty {
                                    id: GID::Long(5.into()),
                                    value: Box::new(GValue::String("tinkergraph".into())),
                                    vertex: Some(GID::Integer(11.into())),
                                    label: "name".into(),
                                    properties: None,
                                }],
                            );
                            tmp
                        },
                    },
                ],
                edges: list![
                    Edge {
                        id: GID::Integer(13.into()),
                        label: "develops".to_string(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("since".into(), Box::new(GValue::Integer(2009.into()))),]
                            .into(),
                    },
                    Edge {
                        id: GID::Integer(14.into()),
                        label: "develops".to_string(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("since".into(), Box::new(GValue::Integer(2010.into()))),]
                            .into(),
                    },
                    Edge {
                        id: GID::Integer(15.into()),
                        label: "uses".to_string(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(4.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(16.into()),
                        label: "uses".to_string(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(5.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(17.into()),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("since".into(), Box::new(GValue::Integer(2010.into()))),]
                            .into(),
                    },
                    Edge {
                        id: GID::Integer(18.into()),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("since".into(), Box::new(GValue::Integer(2011.into()))),]
                            .into(),
                    },
                    Edge {
                        id: GID::Integer(19.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(5.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(20.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(4.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(21.into()),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(8.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("since".into(), Box::new(GValue::Integer(2012.into()))),]
                            .into(),
                    },
                    Edge {
                        id: GID::Integer(22.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(8.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(3.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(23.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(8.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(3.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(24.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(9.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(5.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(25.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(9.into()),
                            label: "person".into(),
                            properties: Map::new(),
                        },
                        properties: [("skill".into(), Box::new(GValue::Integer(3.into()))),].into(),
                    },
                    Edge {
                        id: GID::Integer(26.into()),
                        label: "traverses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Map::new(),
                        },
                        properties: Map::new(),
                    },
                ],
            }),
        }
    );
    gvalue_test!(
        vertex,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } }}),
            object: marko!(),
        }
    );
    gvalue_test!(
        tree,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ ] } } ] } } ] } } ]}),
            object: GValue::Tree(Tree {
                branches: list![Branch {
                    key: Box::new(marko!()),
                    value: Box::new(GValue::Tree(Tree {
                        branches: list![Branch {
                            key: Box::new(GValue::Vertex(Vertex {
                                id: GID::Integer(10.into()),
                                label: "software".into(),
                                properties: {
                                    let mut tmp = Map::<String, List<VertexProperty>>::new();
                                    tmp.insert(
                                        "name".into(),
                                        list![VertexProperty {
                                            id: GID::Long(4.into()),
                                            value: Box::new(GValue::String("gremlin".into())),
                                            vertex: Some(GID::Integer(10.into())),
                                            label: "name".into(),
                                            properties: None,
                                        }],
                                    );
                                    tmp
                                },
                            })),
                            value: Box::new(GValue::Tree(Tree {
                                branches: list![Branch {
                                    key: Box::new(GValue::Vertex(Vertex {
                                        id: 11i32.into(),
                                        label: "software".to_string(),
                                        properties: {
                                            let mut tmp =
                                                Map::<String, List<VertexProperty>>::new();
                                            tmp.insert(
                                                "name".into(),
                                                list![VertexProperty {
                                                    id: 5i64.into(),
                                                    value: Box::new(GValue::String(
                                                        "tinkergraph".into(),
                                                    )),
                                                    vertex: Some(GID::Integer(11.into())),
                                                    label: "name".to_string(),
                                                    properties: None,
                                                }],
                                            );
                                            tmp
                                        },
                                    })),
                                    value: Box::new(GValue::Tree(Tree { branches: list![] }))
                                }],
                            }))
                        }]
                    })),
                },],
            }),
        }
    );
    gvalue_test!(
        vertexproperty,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" }}),
            object: GValue::VertexProperty(VertexProperty {
                id: GID::Long(0.into()),
                value: Box::new(GValue::String("marko".to_string())),
                vertex: Some(GID::Integer(1.into())),
                label: "name".into(),
                properties: None,
            }),
        }
    );
}
mod process {
    use super::*;

    gvalue_test!(
        barrier,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        binding,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        bytecode,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        cardinality,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        column,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        direction,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        operator,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        order,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        pick,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        pop,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        lambda,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        metrics,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()", "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_within,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_without,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_and,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_or,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        scope,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        t,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:T", "@value" : "label"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        textp,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        traversalmetrics,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 0.004 }, "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 13 }, "elementCount" : { "@type" : "g:Int64", "@value" : 13 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "2.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 1 }, "elementCount" : { "@type" : "g:Int64", "@value" : 1 } }, "name" : "TreeStep", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "4.0.0()" } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        traverser,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Traverser", "@value" : { "bulk" : { "@type" : "g:Int64", "@value" : 1 }, "value" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } }}),
            object: GValue::Null,
        }
    );
}
mod request {
    use super::*;

    request_test!(
        authentication_response,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "authentication", "processor" : "", "args" : { "saslMechanism" : "PLAIN", "sasl" : "AHN0ZXBocGhlbgBwYXNzd29yZA==" }}),
            object: Request {
                id: ::uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
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
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "session",
                args: Args::new()
                    .arg("gremlin", "g.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg(
                        "session",
                        GValue::Uuid(
                            ::uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
                        )
                    )
                    .arg("bindings", {
                        let mut tmp = Map::new();
                        tmp.insert("x".into(), GValue::Integer(1.into()));
                        GValue::Map(tmp)
                    }),
            },
        }
    );
    request_test!(
        session_eval_aliased,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "session",
                args: Args::new()
                    .arg("gremlin", "social.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg("aliases", {
                        let mut tmp = Map::<GValue, GValue>::new();
                        tmp.insert("g".into(), GValue::String("social".into()));
                        GValue::Map(tmp)
                    })
                    .arg(
                        "session",
                        GValue::Uuid(
                            ::uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
                        )
                    )
                    .arg("bindings", {
                        let mut tmp = Map::<GValue, GValue>::new();
                        tmp.insert("x".into(), GValue::Integer(1.into()));
                        GValue::Map(tmp)
                    }),
            },
        }
    );
    request_test!(
        session_close,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "close", "processor" : "session", "args" : { "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" } }}),
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
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "",
                args: Args::new()
                    .arg("gremlin", "g.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg("bindings", {
                        let mut tmp = Map::<GValue, GValue>::new();
                        tmp.insert("x".into(), GValue::Integer(1.into()));
                        GValue::Map(tmp)
                    }),
            },
        }
    );
    request_test!(
        sessionless_eval_aliased,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: Request {
                id: uuid::Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "",
                args: Args::new()
                    .arg("gremlin", "social.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg("aliases", {
                        let mut tmp = Map::<GValue, GValue>::new();
                        tmp.insert("g".into(), GValue::from("social"));
                        GValue::Map(tmp)
                    })
                    .arg("bindings", {
                        let mut tmp = Map::<GValue, GValue>::new();
                        tmp.insert("x".into(), GValue::Integer(1.into()));
                        GValue::Map(tmp)
                    }),
            },
        }
    );
}
mod response {
    use super::*;

    response_test!(
        authentication_challenge,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { } }, "result" : { "data" : null, "meta" : { } }}),
            object: crate::Response {
                id: uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap(),
                status: crate::Status {
                    code: 407,
                    message: Default::default(),
                    attributes: serde_json::Value::Object(serde_json::Map::new()),
                },
                data: GValue::Null,
                meta: Map::<String, GValue>::new(),
            },
        }
    );
    response_test!(
        standard_result,
        GraphSON<V2>,
        SQLg,
        Test {
            serial: json!({"requestId":"41d2e28a-20a4-4ab0-b379-d810dede3786","status":{"message":"","code":200,"attributes":{}},"result":{"data":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","vertex":{"@type":"g:Int32","@value":1},"label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","vertex":{"@type":"g:Int32","@value":1},"label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}],"meta":{}}}),
            object: crate::Response {
                id: uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap(),
                status: crate::Status {
                    code: 200,
                    message: None,
                    attributes: serde_json::Value::Object(serde_json::Map::new()),
                },
                meta: Map::<String, GValue>::new(),
                data: GValue::List(list![GValue::Vertex(Vertex {
                    id: GID::Integer(1.into()),
                    label: "person".into(),
                    properties: {
                        let mut tmp = Map::new();
                        tmp.insert(
                            "name".into(),
                            list![VertexProperty {
                                id: GID::Long(0.into()),
                                label: "name".into(),
                                value: Box::new(GValue::String("marko".into())),
                                vertex: Some(GID::Integer(1.into())),
                                properties: None,
                            }],
                        );
                        tmp.insert(
                            "location".into(),
                            list![
                                VertexProperty {
                                    id: GID::Long(6.into()),
                                    value: Box::new(GValue::String("san diego".into())),
                                    label: "location".into(),
                                    vertex: Some(GID::Integer(1.into())),
                                    properties: Some({
                                        let mut tmp2 = Map::new();
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
                                        let mut tmp2 = Map::new();
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
                                        let mut tmp2 = Map::new();
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
                                        let mut tmp2 = Map::new();
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
                })])
            },
        }
    );
}
// mod extended {
//     use super::*;
//
//     gvalue_test!(
//         bigdecimal,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:BigDecimal", "@value" : 123456789987654321123456789987654321u128}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         biginteger,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:BigInteger", "@value" : 123456789987654321123456789987654321u128 }),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         byte,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:Byte", "@value" : 1}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         bytebuffer,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:ByteBuffer", "@value" : "c29tZSBieXRlcyBmb3IgeW91"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         char,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:Char", "@value" : "x"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         duration,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:Duration", "@value" : "PT120H"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         inetaddress,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:InetAddress", "@value" : "localhost"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         instant,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:Instant", "@value" : "2016-12-14T16:39:19.349Z"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         localdate,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:LocalDate", "@value" : "2016-01-01"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         localdatetime,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:LocalDateTime", "@value" : "2016-01-01T12:30"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         localtime,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:LocalTime", "@value" : "12:30:45"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         monthday,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:MonthDay", "@value" : "--01-01"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         offsetdatetime,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:OffsetDateTime", "@value" : "2007-12-03T10:15:30+01:00"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         offsettime,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:OffsetTime", "@value" : "10:15:30+01:00"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         period,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:Period", "@value" : "P1Y6M15D"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         short,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:Int16", "@value" : 100}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         year,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:Year", "@value" : "2016"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         yearmonth,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:YearMonth", "@value" : "2016-06"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         zoneddatetime,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:ZonedDateTime", "@value" : "2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}),
//             object: GValue::Null,
//         }
//     );
//     gvalue_test!(
//         zoneoffset,
//         GraphSON<V2>,
//         SQLg,
//         Test {
//             serial: json!({ "@type" : "gx:ZoneOffset", "@value" : "+03:06:09"}),
//             object: GValue::Null,
//         }
//     );
// }
