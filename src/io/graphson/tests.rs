pub struct Test {
    pub serial: serde_json::Value,
    pub object: crate::prelude::GValue,
}

macro_rules! test_prelude {
    () => {
        use crate::io::graphson::tests::{Test, test};
        use crate::prelude::*;
        use chrono::TimeZone;
        use serde_json::json;
    };
}

macro_rules! test {
    ($fun:ident, $engine:ident, $case:expr) => {
        mod $fun {
            pub(self) use super::*;

            lazy_static::lazy_static! {
                pub static ref TEST_CASE: Test = $case;
            }

            mod deserialize {
                pub(self) use super::*;
                use crate::prelude::GraphSONDeserializer;

                #[test]
                fn ok() {
                    let result = $engine::deserialize(&TEST_CASE.serial);
                    match result {
                        Ok(_) => assert!(true),
                        Err(e) => {
                            assert!(false, "Deserialization failed: {:?}", e);
                        }
                    }
                }

                #[test]
                fn accurate() {
                    let result = $engine::deserialize(&TEST_CASE.serial);
                    assert!(result.is_ok(), "Deserialization failed");
                    assert_eq!(
                        TEST_CASE.object,
                        result.unwrap(),
                        "Deserialization doesn't match expectation"
                    );
                }
            }

            mod serialize {
                pub(self) use super::*;
                use crate::prelude::GraphSONSerializer;

                #[test]
                fn ok() {
                    let result = $engine::serialize(&TEST_CASE.object);
                    match result {
                        Ok(_) => assert!(true),
                        Err(e) => {
                            assert!(false, "Serialization failed: {:?}", e);
                        }
                    }
                }

                #[test]
                fn accurate() {
                    let result = $engine::serialize(&TEST_CASE.object);
                    assert!(result.is_ok(), "Serialization failed");
                    assert_eq!(
                        TEST_CASE.serial,
                        result.unwrap(),
                        "Serialization doesn't match expectation"
                    );
                }
            }
        }
    };
}

pub(crate) use {test, test_prelude};

/*

use super::*;
use crate::prelude::{edge, vertex};
use serde_json::json;

use crate::io::serde::tests::cases::TestCase;
use crate::io::GraphSONDeserializer;
use crate::structure::{
    GValue, Map, Metric, Path, Property, Token, TraversalMetrics, Vertex, VertexProperty, GID,
};
use std::collections::HashMap;

mod cases {
    use crate::prelude::{GValue, GraphSON, GID};
    use crate::structure::{Edge, Property, Vertex, VertexProperty};
    use chrono::{TimeZone, Utc};
    use serde_json::{json, Value};
    use std::collections::HashMap;

    pub struct TestCase {
        pub serial: Value,
        pub object: GValue,
    }

    impl TestCase {
        pub fn test<DS: GraphSON>(&self) {
            self.deserialize::<DS>();
            self.serialialize::<DS>();
        }

        pub fn deserialize<DS: GraphSON>(&self) {
            let result = DS::deserialize(&self.serial);
            assert!(result.is_ok(), "Deserialization failed");
            assert_eq!(
                self.object,
                result.unwrap(),
                "Deserialization doesn't match expectation"
            );
        }

        /// I had a stroke typing this but its great so it stays
        pub fn serialialize<DS: GraphSON>(&self) {
            let result = DS::serialize(&self.object);
            assert!(result.is_ok(), "Serialization failed");
            assert_eq!(
                self.serial,
                result.unwrap(),
                "Serialization doesn't match expectation"
            );
        }
    }

    // Primitives
    lazy_static::lazy_static! {
        pub static ref INT32: TestCase = TestCase {
            serial: json!({
                "@type": "g:Int32",
                "@value": 42u32
            }),
            object: GValue::Int32(42),
        };

        pub static ref INT64: TestCase = TestCase {
            serial: json!({
                "@type": "g:Int64",
                "@value": 42293485672093u64
            }),
            object: GValue::Int64(42293485672093i64),
        };

        // #[allow()]
        pub static ref FLOAT: TestCase = TestCase {
            serial: json!({
                "@type": "g:Float",
                "@value": 3.141592654f32,
            }),
            object: GValue::Float(3.141592654f32),
        };

        pub static ref DOUBLE: TestCase = TestCase {
            serial: json!({
                "@type": "g:Double",
                "@value": 3.141592654f64,
            }),
            object: GValue::Double(3.141592654f64),
        };

        pub static ref STRING: TestCase = TestCase {
            serial: json!("Hi how are ya"),
            object: GValue::String("Hi how are ya".to_string()),
        };
    }

    // Bit less primitive
    lazy_static::lazy_static! {
        pub static ref LIST: TestCase = TestCase {
            serial: json!({
                "@type": "g:List",
                "@value": [
                    {"@type": "g:Int32", "@value": 1},
                    {"@type": "g:Int32", "@value": 2},
                    "3",
                ]
            }),
            object: GValue::List(
                vec![
                    GValue::Int32(1),
                    GValue::Int32(2),
                    GValue::String(String::from("3")),
                ]
                    .into()
            )
        };

        pub static ref SET: TestCase = TestCase {
            serial: json!({
                "@type": "g:Set",
                "@value": [
                    {"@type": "g:Int32", "@value": 1},
                    {"@type": "g:Int32", "@value": 2},
                    {"@type": "g:Float", "@value": 2.0},
                    "3",
                ]
            }),
            object: GValue::Set(
                vec![
                    GValue::Int32(1),
                    GValue::Int32(2),
                    GValue::Float(2.0),
                    GValue::String(String::from("3")),
                ]
                    .into()
            )
        };

        pub static ref DATE: TestCase = TestCase {
            serial: json!({
                "@type": "g:Date",
                "@value": 1551825863
            }),
            object: GValue::Date(Utc.timestamp_millis_opt(1551825863).unwrap()),
        };

        pub static ref UUID: TestCase = TestCase {
            serial: json!({
                "@type" : "g:UUID",
                "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786",
            }),
            object: GValue::Uuid(uuid::Uuid::parse_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap())
        };
    }

    // Vertex Property
    lazy_static::lazy_static! {
        pub static ref PROPERTY: TestCase = TestCase {
            serial: json!({
                "@type": "g:VertexProperty",
                "@value": {
                    "id": {
                        "@type": "g:Int64",
                        "@value": 9
                    },
                    "value": "santa fe",
                    "label": "location",
                }
            }),
            object: GValue::VertexProperty(VertexProperty::new(GID::Int64(9), "location", "santa fe"))
        };
    }

    // V2 Vertex
    lazy_static::lazy_static! {
        pub static ref VERTEX_V2: TestCase = TestCase {
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
                        "vertex": {
                          "@type": "g:Int32",
                          "@value": 1
                        },
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
                        "vertex": {
                          "@type": "g:Int32",
                          "@value": 1
                        },
                        "label": "location",
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
                        "vertex": {
                          "@type": "g:Int32",
                          "@value": 1
                        },
                        "label": "location",
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
                        "vertex": {
                          "@type": "g:Int32",
                          "@value": 1
                        },
                        "label": "location",
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
                        "vertex": {
                          "@type": "g:Int32",
                          "@value": 1
                        },
                        "label": "location",
                      }
                    }
                  ]
                }
              }
            }),
            object: GValue::Vertex(Vertex {
                id: GID::Int32(1),
                label: "person".to_string(),
                properties: {
                    let mut tmp = HashMap::new();
                    let v_id = GID::Int32(1);
                    tmp.insert("name".to_string(), vec![
                        VertexProperty {
                            id: GID::Int64(0),
                            label: "name".to_string(),
                            value: Box::new(GValue::String("marko".to_string())),
                            vertex: Some(v_id.clone())
                        }
                    ]);
                    tmp.insert(
                        "location".to_string(),
                        vec![
                            VertexProperty {
                                id: GID::Int64(6),
                                label: "location".to_string(),
                                value: Box::new(GValue::String("san diego".to_string())),
                                vertex: Some(v_id.clone()),
                            },
                            VertexProperty {
                                id: GID::Int64(7),
                                label: "location".to_string(),
                                value: Box::new(GValue::String("santa cruz".to_string())),
                                vertex: Some(v_id.clone()),
                            },
                            VertexProperty {
                                id: GID::Int64(8),
                                label: "location".to_string(),
                                value: Box::new(GValue::String("brussels".to_string())),
                                vertex: Some(v_id.clone()),
                            },
                            VertexProperty {
                                id: GID::Int64(9),
                                label: "location".to_string(),
                                value: Box::new(GValue::String("santa fe".to_string())),
                                vertex: Some(v_id.clone()),
                            },
                        ]
                    );
                    tmp
                },
            })
        };
    }

    // V3 Vertex + Edge
    lazy_static::lazy_static! {
        pub static ref VERTEX_V3: TestCase = TestCase {
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
                      }
                    }
                  ]
                }
              }
            }),
            object: GValue::Vertex(Vertex {
                id: GID::Int32(1),
                label: "person".to_string(),
                properties: {
                    let mut tmp = HashMap::new();
                    tmp.insert("name".to_string(), vec![VertexProperty::new(GID::Int64(0), "name", "marko")]);
                    tmp.insert(
                        "location".to_string(),
                        vec![
                            VertexProperty::new(GID::Int64(6), "location", "san diego"),
                            VertexProperty::new(GID::Int64(7), "location", "santa cruz"),
                            VertexProperty::new(GID::Int64(8), "location", "brussels"),
                            VertexProperty::new(GID::Int64(9), "location", "santa fe")
                        ]
                    );
                    tmp
                },
            })
        };

        pub static ref EDGE_V3: TestCase = TestCase {
            serial: json!({
              "@type" : "g:Edge",
              "@value" : {
                "id" : {
                  "@type" : "g:Int32",
                  "@value" : 13
                },
                "label" : "develops",
                "inVLabel" : "software",
                "outVLabel" : "person",
                "inV" : {
                  "@type" : "g:Int32",
                  "@value" : 10
                },
                "outV" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "properties" : {
                  "since" : {
                    "@type" : "g:Property",
                    "@value" : {
                      "key" : "since",
                      "value" : {
                        "@type" : "g:Int32",
                        "@value" : 2009
                      }
                    }
                  }
                }
              }
            }),
            object: GValue::Edge(Edge {
                id: GID::Int32(13),
                label: "develops".to_string(),
                in_v: Vertex {
                    id: GID::Int32(10),
                    label: String::from("software"),
                    properties: Default::default(),
                },
                out_v: Vertex {
                    id: GID::Int32(1),
                    label: String::from("person"),
                    properties: Default::default(),
                },
                properties: {
                    let mut tmp = HashMap::new();
                    tmp.insert("since".to_string(), Property::new("since", GValue::Int32(2009)));
                    tmp
                },
            })
        };
    }

    // TODO geometry tests
}


mod int32 {
    test!(v2, V2, INT32);
    test!(v3, V3, INT32);
    test!(v3g, V3g, INT32);
}

mod int64 {
    test!(v2, V2, INT64);
    test!(v3, V3, INT64);
    test!(v3g, V3g, INT64);
}

mod float {
    test!(v2, V2, FLOAT);
    test!(v3, V3, FLOAT);
    test!(v3g, V3g, FLOAT);
}

mod double {
    test!(v2, V2, DOUBLE);
    test!(v3, V3, DOUBLE);
    test!(v3g, V3g, DOUBLE);
}

mod string {
    test!(v2, V2, STRING);
    test!(v3, V3, STRING);
    test!(v3g, V3g, STRING);
}

mod set {
    test!(v2, V2, SET);
    test!(v3, V3, SET);
    test!(v3g, V3g, SET);
}

mod list {
    test!(v2, V2, LIST);
    test!(v3, V3, LIST);
    test!(v3g, V3g, LIST);
}

mod date {
    test!(v2, V2, DATE);
    test!(v3, V3, DATE);
    test!(v3g, V3g, DATE);
}

mod path {
    use super::*;

    #[test]
    fn v2() {
        vertex_a().test::<V2>();
        vertex_b().test::<V2>();
        vertex_c().test::<V2>();
        labels().test::<V2>();
        objects().test::<V2>();
        path().test::<V2>();
    }

    #[test]
    fn v3() {
        vertex_a().test::<V3>();
        vertex_b().test::<V3>();
        vertex_c().test::<V3>();
        labels().test::<V3>();
        objects().test::<V3>();
        path().test::<V3>();
    }

    #[test]
    fn v3g() {
        vertex_a().test::<V3g>();
        vertex_b().test::<V3g>();
        vertex_c().test::<V3g>();
        labels().test::<V3g>();
        objects().test::<V3g>();
        path().test::<V3g>();
    }

    fn vertex_a() -> TestCase {
        TestCase {
            serial: json!({
                "@type":"g:Vertex",
                "@value":{
                    "id":{
                        "@type":"g:Int32",
                        "@value":1
                    },
                    "label":"person"
                }
            }),
            object: Vertex::new(1.into(), "person", HashMap::new()).into(),
        }
    }

    fn vertex_b() -> TestCase {
        TestCase {
            serial: json!({
                "@type": "g:Vertex",
                "@value": {
                    "id": {
                        "@type": "g:Int32",
                        "@value": 10
                    },
                   "label":"software"
                }
            }),
            object: Vertex::new(10.into(), "software", HashMap::new()).into(),
        }
    }

    fn vertex_c() -> TestCase {
        TestCase {
            serial: json!({
                "@type": "g:Vertex",
                "@value": {
                    "id": {
                        "@type":"g:Int32",
                        "@value":11
                    },
                   "label":"software"
                }
            }),
            object: Vertex::new(11.into(), "software", HashMap::new()).into(),
        }
    }

    fn labels() -> TestCase {
        TestCase {
            serial: json!({
                "@type":"g:List",
                "@value":[
                    {"@type":"g:Set","@value":[]},
                    {"@type":"g:Set","@value":[]},
                    {"@type":"g:Set","@value":[]}
                ]
            }),
            object: GValue::List(
                vec![
                    GValue::Set(vec![].into()),
                    GValue::Set(vec![].into()),
                    GValue::Set(vec![].into()),
                ]
                .into(),
            ),
        }
    }

    fn objects() -> TestCase {
        TestCase {
            serial: json!({
                "@type":"g:List",
                "@value":[
                    vertex_a().serial,
                    vertex_b().serial,
                    vertex_c().serial,
                ]
            }),
            object: GValue::List(
                vec![vertex_a().object, vertex_b().object, vertex_c().object].into(),
            ),
        }
    }

    fn path() -> TestCase {
        TestCase {
            serial: json!({
                "@type":"g:Path",
                "@value":{
                    "labels": labels().serial,
                    "objects": objects().serial,
                }
            }),
            object: GValue::Path(Path::new(labels().object, objects().object)),
        }
    }
}

mod property {
    test!(v2, V2, PROPERTY);
    test!(v3, V3, PROPERTY);
    test!(v3g, V3g, PROPERTY);
}

mod vertex {
    test!(v2, V2, VERTEX_V2);
    test!(v3, V3, VERTEX_V3);
    test!(v3g, V3g, VERTEX_V3);
}

mod edge {
    // TODO v2 edge test - the test_edge fn are fairly comprehensive so
    // we'll consider it safe for now
    // test!(v2, V2, EDGE_V2);
    test!(v3, V3, EDGE_V3);
    test!(v3g, V3g, EDGE_V3);
}

#[test]
fn test_properties() {
    let value = json!({"@type":"g:VertexProperty", "@value":{"id":{"@type":"g:Int32","@value":1},"label":"name","value":"marko"}});

    let result = V3::deserialize(&value).expect("Failed to deserialize a VertexProperty");

    assert_eq!(
        result,
        VertexProperty::new(
            GID::Int32(1),
            String::from("name"),
            GValue::String(String::from("marko"))
        )
        .into()
    );

    let value = json!({"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}});

    let result = V3::deserialize(&value).expect("Failed to deserialize a VertexProperty");

    assert_eq!(
        result,
        Property::new(String::from("since"), GValue::Int32(2009)).into()
    );
}

#[test]
fn test_vertex() {
    let value = json!({"@type":"g:Vertex", "@value":{"id":{"@type":"g:Int32","@value":45}}});

    let result = V2::deserialize(&value).expect("Failed to deserialize a Vertex");

    assert_eq!(
        result,
        Vertex::new(GID::Int32(45), String::from("vertex"), HashMap::new()).into()
    );

    let value = r#"{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}"#;

    let val = serde_json::from_str(&value).expect("Failed to serialize");

    let result = V2::deserialize(&val).expect("Failed to deserialize a vertex");

    assert_eq!(
        result,
        vertex!({
                id => 1,
                label => "person",
                properties => {
                    "name" => [ { id => 0 as i64 , value => "marko"}],
                    "location" => [{ id => 6 as i64, value => "san diego"},{ id => 7  as i64 , value => "santa cruz"},{ id => 8  as i64, value => "brussels"},{ id => 9  as i64, value => "santa fe"}]
                }
            }).into()
    );
}

#[test]
fn test_edge() {
    let value = json!({"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":1},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}}}});

    let result = V2::deserialize(&value).expect("Failed to deserialize an Edge");

    assert_eq!(
        result,
        edge!({
            id => 13,
            label=> "develops",
            inV => {
                id => 10,
                label => "software"
            },
            outV => {
                id => 1,
                label => "person"
            },
            properties => {

            }
        })
        .into()
    );
}

#[test]
fn test_traversal_metrics() {
    let value = serde_json::from_str(r#"{"@type":"g:TraversalMetrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":0.004},"metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":4},"elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":13},"elementCount",{"@type":"g:Int64","@value":13}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","2.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":7},"elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":1},"elementCount",{"@type":"g:Int64","@value":1}]},"name","TreeStep","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","4.0.0()"]}}]}]}}"#).expect("Error parsing json");

    let result = V3::deserialize(&value).expect("Failed to deserialize a TraversalMetrics");

    let traversal_metrics = TraversalMetrics::new(
        0.004,
        vec![
            Metric::new(
                "7.0.0()",
                "TinkerGraphStep(vertex,[~label.eq(person)])",
                100.0,
                4,
                4,
                25.0,
                vec![],
            ),
            Metric::new(
                "2.0.0()",
                "VertexStep(OUT,vertex)",
                100.0,
                13,
                13,
                25.0,
                vec![],
            ),
            Metric::new(
                "3.0.0()",
                "VertexStep(OUT,vertex)",
                100.0,
                7,
                7,
                25.0,
                vec![],
            ),
            Metric::new("4.0.0()", "TreeStep", 100.0, 1, 1, 25.0, vec![]),
        ],
    );

    assert_eq!(result, traversal_metrics.into());
}

#[test]
fn test_token() {
    let value = json!({
        "@type": "g:T",
        "@value": "id"
    });
    let result = V3::deserialize(&value).expect("Failed to deserialize a Token");

    assert_eq!(result, GValue::Token(Token::new("id")));
}

#[test]
fn test_map_with_token() {
    let value = json!({
        "@type": "g:Map",
         "@value": [
            {"@type": "g:T","@value": "label"},
            "person",
            "name",
            {"@type": "g:List","@value": ["marko"]}
         ]
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize a Token");

    let value_map: Map = [
        ("label".into(), GValue::String(String::from("person"))),
        (
            "name".into(),
            GValue::List(vec![String::from("marko").into()].into()),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(result, GValue::Map(value_map));
}
*/
