#[allow(unused_imports)]
pub use super::macros::*;
pub use crate::api::V3;
pub use crate::graphson::tags::*;
pub use chrono::TimeZone;
pub use indexmap::{IndexMap, indexset};
pub use serde_json::json;
pub use std::str::FromStr;

mod core {
    use super::*;

    gvalue_test!(
        class,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Class, "@value" : "java.io.File"}),
            object: GValue::Class("java.io.File".into()),
        }
    );
    gvalue_test!(
        date,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Date, "@value" : 1481750076295i64 }),
            object: GValue::Date(Date(
                chrono::Utc.timestamp_millis_opt(1481750076295i64).unwrap()
            )),
        }
    );
    gvalue_test!(
        double,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Double, "@value" : 100.0f64 }),
            object: Double(100.0).into(),
        }
    );
    gvalue_test!(
        float,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Float, "@value" : 100.0f32 }),
            object: Float(100.0).into(),
        }
    );
    gvalue_test!(
        integer,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Integer, "@value" : 100i32 }),
            object: Integer(100).into(),
        }
    );
    gvalue_test!(
        list,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, "person", true ]}),
            object: GValue::List(list![
                GValue::Integer(Integer(1)),
                GValue::String("person".into()),
                GValue::Bool(Bool(true)),
            ]),
        }
    );
    gvalue_test!(
        long,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Long, "@value" : 100u64 }),
            object: Long(100).into(),
        }
    );
    gvalue_test!(
        map,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({
                "@type" : Tag::Map,
                "@value" : [
                    {"@type" : "g:Date", "@value" : 1481750076295u64 }, "red",

                    {
                        "@type" : "g:List",
                        "@value" : [
                            { "@type" : "g:Int32", "@value" : 1 },
                            { "@type" : "g:Int32", "@value" : 2 },
                            { "@type" : "g:Int32", "@value" : 3 }
                        ]
                    }, { "@type" : "g:Date", "@value" : 1481750076295u64 },

                    "test", { "@type" : "g:Int32", "@value" : 123 }
                ]
            }),
            object: GValue::Map(Map({
                let mut tmp = IndexMap::new();
                tmp.insert(
                    chrono::Utc
                        .timestamp_millis_opt(1481750076295)
                        .unwrap()
                        .into(),
                    GValue::String(String::from("red")),
                );
                tmp.insert(
                    list![Integer(1).into(), Integer(2).into(), Integer(3).into(),].into(),
                    chrono::Utc
                        .timestamp_millis_opt(1481750076295)
                        .unwrap()
                        .into(),
                );
                tmp.insert("test".into(), Integer(123).into());
                tmp
            })),
        }
    );
    gvalue_test!(
        set,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : Tag::Set, "@value" : [ { "@type" : Tag::Integer, "@value" : 1 }, "person", true ]}),
            object: GValue::Set(
                indexset![
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
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Timestamp", "@value" : 1481750076295i64 }),
            object: GValue::Timestamp(Timestamp(1481750076295i64)),
        }
    );
    gvalue_test!(
        uuid,
        GraphSON<V3>,
        SQLg,
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
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":1},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}}}}),
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
                properties: [(
                    "since".into(),
                    Box::new(GValue::Property(Property {
                        key: "since".into(),
                        value: Box::new(GValue::Integer(2009.into())),
                        element: Box::new(GValue::Null),
                    }))
                )]
                .into()
            }),
        }
    );
    gvalue_test!(
        path,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"@type":"g:Path","@value":{"labels":{"@type":"g:List","@value":[{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]}]},"objects":{"@type":"g:List","@value":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"software"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software"}}]}}}),
            object: GValue::Path(Path {
                labels: Box::new(GValue::List(list![
                    GValue::Set(Set::new()),
                    GValue::Set(Set::new()),
                    GValue::Set(Set::new())
                ])),
                objects: Box::new(GValue::List(list![
                    GValue::Vertex(Vertex {
                        id: 1i32.into(),
                        label: "person".to_string(),
                        properties: Default::default(),
                    }),
                    GValue::Vertex(Vertex {
                        id: 10i32.into(),
                        label: "software".to_string(),
                        properties: Default::default(),
                    }),
                    GValue::Vertex(Vertex {
                        id: 11i32.into(),
                        label: "software".to_string(),
                        properties: Default::default(),
                    }),
                ]))
            })
        }
    );
    gvalue_test!(
        property,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } }}),
            object: GValue::Property(Property {
                key: "since".into(),
                value: Box::new(GValue::Integer(2009.into())),
                element: Box::new(GValue::Null)
            }),
        }
    );
    gvalue_test!(
        tinkergraph,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"@type":"tinker:graph","@value":{"vertices":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":7},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":1},"value":"stephen","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":10},"value":"centreville","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1990},"endTime":{"@type":"g:Int32","@value":2000}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":11},"value":"dulles","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2000},"endTime":{"@type":"g:Int32","@value":2006}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":12},"value":"purcellville","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2006}}}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":8},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":2},"value":"matthias","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":13},"value":"bremen","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2007}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":14},"value":"baltimore","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2007},"endTime":{"@type":"g:Int32","@value":2011}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":15},"value":"oakland","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2011},"endTime":{"@type":"g:Int32","@value":2014}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":16},"value":"seattle","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2014}}}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":9},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":3},"value":"daniel","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":17},"value":"spremberg","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1982},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":18},"value":"kaiserslautern","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005},"endTime":{"@type":"g:Int32","@value":2009}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":19},"value":"aachen","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2009}}}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":4},"value":"gremlin","label":"name"}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":5},"value":"tinkergraph","label":"name"}}]}}}],"edges":[{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":1},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":14},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":11},"outV":{"@type":"g:Int32","@value":1},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2010}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":15},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":1},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":4}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":16},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":11},"outV":{"@type":"g:Int32","@value":1},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":5}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":17},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":7},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2010}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":18},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":11},"outV":{"@type":"g:Int32","@value":7},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2011}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":19},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":7},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":5}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":20},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":11},"outV":{"@type":"g:Int32","@value":7},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":4}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":21},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":8},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2012}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":22},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":8},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":3}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":23},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":11},"outV":{"@type":"g:Int32","@value":8},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":3}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":24},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":9},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":5}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":25},"label":"uses","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":11},"outV":{"@type":"g:Int32","@value":9},"properties":{"skill":{"@type":"g:Property","@value":{"key":"skill","value":{"@type":"g:Int32","@value":3}}}}}},{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":26},"label":"traverses","inVLabel":"software","outVLabel":"software","inV":{"@type":"g:Int32","@value":11},"outV":{"@type":"g:Int32","@value":10}}}]}}),
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
                                    properties: Default::default(),
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
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "since".into(),
                            Box::new(GValue::Property(Property {
                                key: "since".to_string(),
                                value: Box::new(Integer(2009).into()),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(14.into()),
                        label: "develops".to_string(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "since".into(),
                            Box::new(GValue::Property(Property {
                                key: "since".to_string(),
                                value: Box::new(Integer(2010).into()),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(15.into()),
                        label: "uses".to_string(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(4.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(16.into()),
                        label: "uses".to_string(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(1.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(5.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(17.into()),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "since".into(),
                            Box::new(GValue::Property(Property {
                                key: "since".to_string(),
                                value: Box::new(GValue::Integer(2010.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(18.into()),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "since".into(),
                            Box::new(GValue::Property(Property {
                                key: "since".to_string(),
                                value: Box::new(GValue::Integer(2011.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(19.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(5.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(20.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(7.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(4.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(21.into()),
                        label: "develops".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(8.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "since".into(),
                            Box::new(GValue::Property(Property {
                                key: "since".to_string(),
                                value: Box::new(GValue::Integer(2012.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(22.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(8.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(3.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(23.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(8.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(3.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(24.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(9.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(5.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(25.into()),
                        label: "uses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(9.into()),
                            label: "person".into(),
                            properties: Default::default(),
                        },
                        properties: [(
                            "skill".into(),
                            Box::new(GValue::Property(Property {
                                key: "skill".to_string(),
                                value: Box::new(GValue::Integer(3.into())),
                                element: Box::new(GValue::Null),
                            }))
                        ),]
                        .into(),
                    },
                    Edge {
                        id: GID::Integer(26.into()),
                        label: "traverses".into(),
                        in_v: Vertex {
                            id: GID::Integer(11.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        out_v: Vertex {
                            id: GID::Integer(10.into()),
                            label: "software".into(),
                            properties: Default::default(),
                        },
                        properties: Default::default(),
                    },
                ],
            }),
        }
    );
    gvalue_test!(
        vertex,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({
            "@type": "g:Vertex",
            "@value": {
                "id": {
                  "@type": "g:Int32",
                  "@value": 1
                },
                "label": "person",
                "properties": {
                  "name": [
                    {
                      "@type": "g:VertexProperty",
                      "@value": {
                        "id": {
                          "@type": "g:Int64",
                          "@value": 0
                        },
                        "value": "marko",
                        "label": "name"
                      }
                    }
                  ],
                  "location": [
                    {
                      "@type": "g:VertexProperty",
                      "@value": {
                        "id": {
                          "@type": "g:Int64",
                          "@value": 6
                        },
                        "value": "san diego",
                        "label": "location",
                        "properties": {
                          "startTime": {
                            "@type": "g:Int32",
                            "@value": 1997
                          },
                          "endTime": {
                            "@type": "g:Int32",
                            "@value": 2001
                          }
                        }
                      }
                    },
                    {
                      "@type": "g:VertexProperty",
                      "@value": {
                        "id": {
                          "@type": "g:Int64",
                          "@value": 7
                        },
                        "value": "santa cruz",
                        "label": "location",
                        "properties": {
                          "startTime": {
                            "@type": "g:Int32",
                            "@value": 2001
                          },
                          "endTime": {
                            "@type": "g:Int32",
                            "@value": 2004
                          }
                        }
                      }
                    },
                    {
                      "@type": "g:VertexProperty",
                      "@value": {
                        "id": {
                          "@type": "g:Int64",
                          "@value": 8
                        },
                        "value": "brussels",
                        "label": "location",
                        "properties": {
                          "startTime": {
                            "@type": "g:Int32",
                            "@value": 2004
                          },
                          "endTime": {
                            "@type": "g:Int32",
                            "@value": 2005
                          }
                        }
                      }
                    },
                    {
                      "@type": "g:VertexProperty",
                      "@value": {
                        "id": {
                          "@type": "g:Int64",
                          "@value": 9
                        },
                        "value": "santa fe",
                        "label": "location",
                        "properties": {
                          "startTime": {
                            "@type": "g:Int32",
                            "@value": 2005
                          }
                        }
                      }
                    }
                  ]
                }
            }
            }),
            object: GValue::Vertex(Vertex {
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
                            vertex: None,
                            properties: Default::default(),
                        }],
                    );
                    tmp.insert(
                        "location".into(),
                        list![
                            VertexProperty {
                                id: GID::Long(6.into()),
                                value: Box::new(GValue::String("san diego".into())),
                                label: "location".into(),
                                vertex: None,
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
                                vertex: None,
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
                                vertex: None,
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
                                vertex: None,
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
            }),
        }
    );
    gvalue_test!(
        vertexproperty,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" }}),
            object: GValue::VertexProperty(VertexProperty {
                id: GID::Long(0.into()),
                value: Box::new(GValue::String("marko".to_string())),
                vertex: None,
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
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
            object: GValue::Barrier(Barrier::NormSack),
        }
    );
    gvalue_test!(
        binding,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
            object: GValue::Binding(Binding {
                key: "x".into(),
                value: GValue::Integer(1.into()).boxed()
            }),
        }
    );
    gvalue_test!(
        bulkset,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:BulkSet", "@value" : [ "drake", { "@type" : "g:Int64", "@value" : 1 }, "josh", { "@type" : "g:Int64", "@value" : 2 } ]}),
            object: GValue::BulkSet(BulkSet {
                map: {
                    let mut tmp = Map::new();
                    tmp.insert("drake".into(), GValue::Long(1.into()));
                    tmp.insert("josh".into(), GValue::Long(2.into()));
                    tmp
                },
                occurrences: 2, // i guess? TODO look into how to populate while deserializing
            }),
        }
    );
    gvalue_test!(
        bytecode,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
            object: GValue::Bytecode(Bytecode {
                source_instructions: list![Instruction {
                    operator: "V".into(),
                    args: list!["hasLabel".into(), "person".into()],
                }],
                step_instructions: list![],
            }),
        }
    );
    gvalue_test!(
        cardinality,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        column,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        direction,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        operator,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        order,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        pick,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        pop,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        lambda,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        metrics,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":4},"elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()","metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":7},"elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}}]}]}}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_within,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_without,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_and,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        p_or,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } } ] }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        scope,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        t,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:T", "@value" : "label"}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        textp,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
            object: GValue::Null,
        }
    );
    gvalue_test!(
        traversalmetrics,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 0.004 }, "metrics", { "@type" : "g:List", "@value" : [ { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 4 }, "elementCount", { "@type" : "g:Int64", "@value" : 4 } ] }, "name", "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "7.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 13 }, "elementCount", { "@type" : "g:Int64", "@value" : 13 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "2.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 7 }, "elementCount", { "@type" : "g:Int64", "@value" : 7 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "3.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 1 }, "elementCount", { "@type" : "g:Int64", "@value" : 1 } ] }, "name", "TreeStep", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "4.0.0()" ] } } ] } ] }}),
            object: GValue::TraversalMetrics(TraversalMetrics::new(
                Double(0.004),
                list![
                    Metrics::new(
                        "7.0.0()",
                        "TinkerGraphStep(vertex,[~label.eq(person)])",
                        100.0,
                        4,
                        4,
                        25.0,
                        list![],
                    ),
                    Metrics::new(
                        "2.0.0()",
                        "VertexStep(OUT,vertex)",
                        100.0,
                        13,
                        13,
                        25.0,
                        list![],
                    ),
                    Metrics::new(
                        "3.0.0()",
                        "VertexStep(OUT,vertex)",
                        100.0,
                        7,
                        7,
                        25.0,
                        list![],
                    ),
                    Metrics::new("4.0.0()", "TreeStep", 100.0, 1, 1, 25.0, list![]),
                ],
            )),
        }
    );
    gvalue_test!(
        traverser,
        GraphSON<V3>,
        SQLg,
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
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"requestId":"cb682578-9d92-4499-9ebc-5c6aa73c5397","op":"authentication","processor":"","args":{"@type":"g:Map","@value":["saslMechanism","PLAIN","sasl","AHN0ZXBocGhlbgBwYXNzd29yZA=="]}}),
            object: Request {
                id: Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "authentication",
                proc: "",
                args: Args::new()
                    .arg("saslMechanism", "PLAIN")
                    .arg("sasl", "AHN0ZXBocGhlbgBwYXNzd29yZA=="),
            }
        }
    );
    request_test!(
        session_eval,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"requestId":"cb682578-9d92-4499-9ebc-5c6aa73c5397","op":"eval","processor":"session","args":{"@type":"g:Map","@value":["gremlin","g.V(x)","language","gremlin-groovy","session",{"@type":"g:UUID","@value":"41d2e28a-20a4-4ab0-b379-d810dede3786"},"bindings",{"@type":"g:Map","@value":["x",{"@type":"g:Int32","@value":1}]}]}}),
            object: Request {
                id: Uuid::from_str("cb682578-9d92-4499-9ebc-5c6aa73c5397").unwrap(),
                op: "eval",
                proc: "session",
                args: Args::new()
                    .arg("bindings", {
                        let mut tmp = IndexMap::new();
                        tmp.insert("x", GValue::Integer(1.into()));
                        tmp
                    })
                    .arg("gremlin", "g.V(x)")
                    .arg("language", "gremlin-groovy")
                    .arg(
                        "session",
                        GValue::Uuid(
                            Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
                        )
                    ),
            },
        }
    );
    request_test!(
        session_eval_aliased,
        GraphSON<V3>,
        SQLg,
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
                        let mut tmp = IndexMap::new();
                        tmp.insert("g".into(), GValue::String("social".into()));
                        GValue::Map(Map::from(tmp))
                    })
                    .arg(
                        "session",
                        GValue::Uuid(
                            ::uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()
                        )
                    )
                    .arg("bindings", {
                        let mut tmp = IndexMap::new();
                        tmp.insert("x".into(), GValue::Integer(1.into()));
                        GValue::Map(Map::from(tmp))
                    }),
            },
        }
    );
    request_test!(
        session_close,
        GraphSON<V3>,
        SQLg,
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
        GraphSON<V3>,
        SQLg,
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
                        let mut tmp = IndexMap::new();
                        tmp.insert("x".into(), GValue::Integer(1.into()));
                        GValue::Map(Map::from(tmp))
                    }),
            },
        }
    );
    request_test!(
        sessionless_eval_aliased,
        GraphSON<V3>,
        SQLg,
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
                        let mut tmp = IndexMap::new();
                        tmp.insert("g".into(), GValue::from("social"));
                        GValue::Map(Map::from(tmp))
                    })
                    .arg("bindings", {
                        let mut tmp = IndexMap::new();
                        tmp.insert("x".into(), GValue::Integer(1.into()));
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
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"requestId":"41d2e28a-20a4-4ab0-b379-d810dede3786","status":{"message":"","code":407,"attributes":{"@type":"g:Map","@value":[]}},"result":{"data":null,"meta":{"@type":"g:Map","@value":[]}}}),
            object: crate::Response {
                id: uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap(),
                status: crate::Status {
                    code: 407,
                    message: Default::default(),
                    attributes: serde_json::Value::Object(serde_json::Map::new()),
                },
                data: GValue::Null,
                meta: Map::new(),
            },
        }
    );
    response_test!(
        standard_result,
        GraphSON<V3>,
        SQLg,
        Test {
            serial: json!({"requestId":"41d2e28a-20a4-4ab0-b379-d810dede3786","status":{"message":"","code":200,"attributes":{"@type":"g:Map","@value":[]}},"result":{"data":{"@type":"g:List","@value":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}]},"meta":{"@type":"g:Map","@value":[]}}}),
            object: crate::Response {
                id: uuid::Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap(),
                status: crate::Status {
                    code: 200,
                    message: None,
                    attributes: serde_json::Value::Object(serde_json::Map::new()),
                },
                meta: Map::new(),
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
                                properties: Default::default(),
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
