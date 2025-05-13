use crate::GValue;
use crate::io::graphson::de::{Type, Typed};
use crate::io::graphson::types::v2::*;
use crate::io::serde::Deserialize;
use crate::io::{
    Deserializer, Error, IOHelpers, Response, Status, V2, expect_double, expect_float, expect_i32,
    expect_i64, get_value,
};
use crate::structure::*;
use chrono::{TimeZone, Utc};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

impl V2 {
    fn core_deserializer<'a>(blob: Type<'a>) -> Result<GValue, Error> {
        let key = get_value!(blob.tag, Value::String)?;
        macro_rules! deserialize {
            ($_type:ty) => {
                blob.value.deserialize::<Self, $_type>().map(GValue::from)
            };
        }

        match key.as_ref() {
            CLASS => deserialize!(Class),
            INT => deserialize!(Integer),
            LONG => deserialize!(Long),
            FLOAT => deserialize!(Float),
            DOUBLE => deserialize!(Double),
            DATE => deserialize!(Date),
            TIMESTAMP => deserialize!(Timestamp),
            UUID => deserialize!(Uuid),
            EDGE => deserialize!(Edge),
            PATH => deserialize!(Path),
            PROPERTY => deserialize!(Property),
            TINKER_GRAPH => deserialize!(TinkerGraph),
            TREE => deserialize!(Tree),
            VERTEX => deserialize!(Vertex),
            VERTEX_PROPERTY => deserialize!(VertexProperty),
            // BARRIER => todo!("support"),
            // BINDING => todo!("support"),
            // BYTECODE => todo!("support"),
            // CARDINALITY => todo!("support"),
            // COLUMN => todo!("support"),
            DIRECTION => deserialize!(Direction),
            // DT => todo!("support"),
            // LAMBDA => todo!("support"),
            // MERGE => todo!("support"),
            // METRICS => todo!("support"),
            // OPERATOR => todo!("support"),
            // ORDER => todo!("support"),
            // P => todo!("support"),
            // PICK => todo!("support"),
            // POP => todo!("support"),
            // SCOPE => todo!("support"),
            T => deserialize!(Token),
            // TEXT_P => todo!("support"),
            TRAVERSAL_METRICS => deserialize!(TraversalMetrics),
            TRAVERSER => deserialize!(Traverser),
            //
            type_tag => Err(Error::Unsupported(type_tag.to_string())),
        }
    }

    fn special_deserializer<'a>(value: &Value) -> Result<GValue, Error> {
        match value {
            val if is_stargraph(val) => value.deserialize::<Self, StarGraph>().map(GValue::from),
            // val if is_response(val) =>{
            //     value.deserialize::<Self, Response>()
            // },
            _ => Err(Error::UnexpectedJson {
                msg: "Special case".into(),
                value: value.clone(),
            }),
        }
    }
}

fn is_stargraph(val: &Value) -> bool {
    val.get("starVertex").is_some()
}

fn is_response(val: &Value) -> bool {
    val.get("requestId").is_some() && val.get("status").is_some()
}

impl Deserializer<Response> for V2 {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        let id: Uuid = {
            let _id = Self::get(value, "request_id")?.clone();
            serde_json::from_value(_id)?
        };
        let result: GValue = {
            let result = Self::get(value, "result")?;
            let data = Self::get(result, "data")?;
            Self::deserialize(data)?
        };
        let status: Status = {
            let status = Self::get(value, "status")?;
            Self::deserialize(status)?
        };

        Ok(Response { id, result, status })
    }
}
impl Deserializer<Status> for V2 {
    fn deserialize(value: &Value) -> Result<Status, Error> {
        let code = Self::get(value, "code").map(|code| code.as_i64().unwrap() as i16)?;
        let message = Self::get(value, "message")
            .ok()
            .map(|value| value.as_str().unwrap().to_string());

        Ok(Status { code, message })
    }
}
impl Deserializer<GID> for V2 {
    fn deserialize(val: &Value) -> Result<GID, Error> {
        let gvalue = val.deserialize::<Self, GValue>()?;
        match gvalue {
            GValue::String(d) => Ok(GID::String(d)),
            GValue::Integer(d) => Ok(GID::Integer(d)),
            GValue::Long(d) => Ok(GID::Long(d)),
            value => Err(Error::UnexpectedGValue {
                msg: "Ineligible for GKey".into(),
                value,
            }),
        }
    }
}
impl Deserializer<GValue> for V2 {
    fn deserialize(value: &Value) -> Result<GValue, Error> {
        match value {
            Value::String(_) => value.deserialize::<Self, String>().map(GValue::from),
            Value::Number(_) => value.deserialize::<Self, Integer>().map(GValue::from),
            Value::Object(_obj) => match value.typed() {
                Ok(blob) => Self::core_deserializer(blob),
                Err(err) => match err {
                    Error::Missing(_) => Self::special_deserializer(value),
                    _ => panic!(),
                },
            },
            Value::Array(values) => {
                let collection = values
                    .iter()
                    .map(Self::deserialize)
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(GValue::List(List(collection)))
            }
            Value::Bool(_) => value.deserialize::<Self, Bool>().map(GValue::from),
            Value::Null => Ok(GValue::Null),
        }
    }
}
impl Deserializer<Bool> for V2 {
    fn deserialize(val: &Value) -> Result<Bool, Error> {
        let bool = get_value!(val, Value::Bool)?;
        Ok(Bool(*bool).into())
    }
}
impl Deserializer<Class> for V2 {
    fn deserialize(val: &Value) -> Result<Class, Error> {
        let class = get_value!(val, Value::String)?;
        Ok(class.into())
    }
}
impl Deserializer<Date> for V2 {
    fn deserialize(val: &Value) -> Result<Date, Error> {
        let val = expect_i64!(val);
        let datetime = Utc.timestamp_millis_opt(val).unwrap();
        let date = Date(datetime);
        Ok(date.into())
    }
}
impl Deserializer<Double> for V2 {
    fn deserialize(val: &Value) -> Result<Double, Error> {
        let val = expect_double!(val);
        Ok(Double(val))
    }
}
impl Deserializer<Float> for V2 {
    fn deserialize(val: &Value) -> Result<Float, Error> {
        let val = expect_float!(val);
        Ok(Float(val))
    }
}
impl Deserializer<Integer> for V2 {
    fn deserialize(val: &Value) -> Result<Integer, Error> {
        let val = expect_i32!(val);
        Ok(Integer(val))
    }
}
impl Deserializer<Long> for V2 {
    fn deserialize(val: &Value) -> Result<Long, Error> {
        let val = expect_i64!(val);
        Ok(Long(val))
    }
}
impl Deserializer<Timestamp> for V2 {
    fn deserialize(val: &Value) -> Result<Timestamp, Error> {
        let val = expect_i64!(val);
        let ms_since_epoch = Timestamp(val);
        Ok(ms_since_epoch)
    }
}
impl Deserializer<Uuid> for V2 {
    fn deserialize(val: &Value) -> Result<Uuid, Error> {
        let val = get_value!(val, Value::String)?;
        let uuid = uuid::Uuid::parse_str(&val)?;
        Ok(uuid)
    }
}
impl Deserializer<Edge> for V2 {
    fn deserialize(val: &Value) -> Result<Edge, Error> {
        let edge_id = val["id"].deserialize::<Self, GID>()?;
        let label = val
            .get("label")
            .map(|f| get_value!(f, Value::String).map(Clone::clone))
            .unwrap_or_else(|| Ok(String::from("edge")))?;

        let in_v_id = val["inV"].deserialize::<Self, GID>()?;
        // This is intentional, there is no clear guidance on the discrepancies in 2.0.
        // let in_v_label = get_value!(&val["inVLabel"], Value::String)?.clone();
        let in_v_label = val
            .get("inVLabel")
            .map(|label| get_value!(label, Value::String).map(Clone::clone).unwrap())
            .unwrap_or("Unavailable".into());

        let out_v_id = (&val["outV"]).deserialize::<Self, GID>()?;
        // If we don't account for it, we can't ser/de Property types.
        let out_v_label = val
            .get("outVLabel")
            .map(|label| get_value!(label, Value::String).map(Clone::clone).unwrap())
            .unwrap_or("Unavailable".into());
        Ok(Edge::new(
            edge_id,
            label,
            in_v_id,
            in_v_label,
            out_v_id,
            out_v_label,
            HashMap::new(),
        ))
    }
}
impl Deserializer<Path> for V2 {
    fn deserialize(val: &Value) -> Result<Path, Error> {
        let labels = val["labels"].deserialize::<Self, GValue>()?;
        let objects = val["objects"].deserialize::<Self, GValue>()?;
        Ok(Path::new(labels, objects).into())
    }
}
impl Deserializer<Property> for V2 {
    fn deserialize(val: &Value) -> Result<Property, Error> {
        let key = val
            .get("key")
            .map(|v| get_value!(v, Value::String).map(Clone::clone))
            .ok_or(Error::UnexpectedJson {
                msg: "Missing Property 'key' key".into(),
                value: val.clone(),
            })??;
        let value = val.get("value").ok_or(Error::UnexpectedJson {
            msg: "Missing Property 'value' key".into(),
            value: val.clone(),
        })?;
        let element = val.get("element").ok_or(Error::UnexpectedJson {
            msg: "Missing Property 'element' key".into(),
            value: val.clone(),
        })?;

        let value_obj = value.deserialize::<Self, GValue>()?;
        let element_obj = element.deserialize::<Self, GValue>()?;
        let property = Property {
            key,
            value: Box::new(value_obj),
            element: Box::new(element_obj),
        };

        Ok(property)
    }
}
impl Deserializer<StarGraph> for V2 {
    fn deserialize(val: &Value) -> Result<StarGraph, Error> {
        let value = val.get("starVertex").ok_or(Error::UnexpectedJson {
            msg: "Malformed StarGraph.".to_string(),
            value: val.clone(),
        })?;
        let vertex = value.typed()?.value.deserialize::<Self, Vertex>()?;
        let yikes = vertex.into();
        Ok(yikes)
    }
}
impl Deserializer<TinkerGraph> for V2 {
    fn deserialize(val: &Value) -> Result<TinkerGraph, Error> {
        let _debug = val.to_string();
        let vertex_values = get_value!(
            val.get("vertices").ok_or(Error::UnexpectedJson {
                msg: "TinkerGraph missing 'vertices' key".into(),
                value: val.clone(),
            })?,
            Value::Array
        )?;
        let edge_values = get_value!(
            val.get("edges").ok_or(Error::UnexpectedJson {
                msg: "TinkerGraph missing 'edges' key".into(),
                value: val.clone(),
            })?,
            Value::Array
        )?;
        let vertices = vertex_values
            .into_iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Vertex>(),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let edges = edge_values
            .into_iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Edge>(),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(TinkerGraph { vertices, edges })
    }
}
impl Deserializer<Tree> for V2 {
    fn deserialize(val: &Value) -> Result<Tree, Error> {
        let array = get_value!(val, Value::Array)?;
        let branches = array
            .into_iter()
            .map(|val| val.deserialize::<Self, Branch>())
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Tree { branches })
    }
}
impl Deserializer<Branch> for V2 {
    fn deserialize(val: &Value) -> Result<Branch, Error> {
        let obj = get_value!(val, Value::Object)?;

        let key = obj
            .get("key")
            .ok_or(Error::UnexpectedJson {
                msg: "Missing 'key' key".to_string(),
                value: val.clone(),
            })
            .map(|value| value.deserialize::<Self, GValue>())??;

        let value = obj
            .get("value")
            .ok_or(Error::UnexpectedJson {
                msg: "Missing 'value' key".to_string(),
                value: val.clone(),
            })
            .map(|value| value.deserialize::<Self, GValue>())??;

        Ok(Branch {
            key: Box::new(key),
            value: Box::new(value),
        })
    }
}
impl Deserializer<Vertex> for V2 {
    fn deserialize(val: &Value) -> Result<Vertex, Error> {
        let label = val
            .get("label")
            .map(|f| get_value!(f, Value::String).map(Clone::clone))
            .unwrap_or_else(|| Ok(String::from("vertex")))?;
        let id = val["id"].deserialize::<Self, GID>()?;
        let properties = val["properties"].deserialize::<Self, VertexProperties>()?;
        let vertex = Vertex {
            id,
            label,
            properties,
        };

        Ok(vertex)
    }
}
type VertexProperties = HashMap<String, Vec<VertexProperty>>;
impl Deserializer<VertexProperties> for V2 {
    fn deserialize(val: &Value) -> Result<VertexProperties, Error> {
        match val {
            Value::Object(o) => {
                let mut p = HashMap::new();
                for (k, v) in o {
                    match v {
                        Value::Array(arr) => {
                            let mut vec = vec![];
                            for elem in arr {
                                let vp =
                                    elem.typed()?.value.deserialize::<Self, VertexProperty>()?;
                                vec.push(vp);
                            }
                            p.insert(k.clone(), vec);
                        }
                        value => {
                            return Err(Error::UnexpectedJson {
                                msg: "Expected array for properties".into(),
                                value: value.clone(),
                            });
                        }
                    };
                }
                Ok(p)
            }
            Value::Null => Ok(HashMap::new()),
            value => Err(Error::UnexpectedJson {
                msg: "Expected object or null for properties".into(),
                value: value.clone(),
            }),
        }
    }
}
impl Deserializer<VertexProperty> for V2 {
    fn deserialize(val: &Value) -> Result<VertexProperty, Error> {
        let _debug = val.to_string();
        let mut property = VertexProperty {
            id: val["id"].deserialize::<Self, GID>()?,
            value: Box::new(val["value"].deserialize::<Self, GValue>()?),
            vertex: None,
            label: val
                .get("label")
                .map(|f| get_value!(f, Value::String).map(Clone::clone))
                .unwrap_or_else(|| {
                    Err(Error::UnexpectedJson {
                        msg: "Missing VertexProperty label".into(),
                        value: val.clone(),
                    })
                })?,
            properties: None,
        };

        if let Some(vertex_id) = val.get("vertex") {
            let vertex = vertex_id.typed()?.value.deserialize::<Self, GID>()?;
            property.vertex = Some(vertex);
        }

        property.properties = val
            .get("properties")
            .map(|p| get_value!(p, Value::Object).unwrap())
            .map(|obj| {
                obj.into_iter()
                    .map(|(label, property_value)| {
                        (label, property_value.deserialize::<Self, GValue>())
                    })
                    .filter(|(_, v)| v.is_ok())
                    .map(|(k, v)| (k.clone(), v.unwrap()))
                    .collect::<HashMap<String, GValue>>()
            });

        Ok(property)
    }
}
impl Deserializer<Bytecode> for V2 {
    fn deserialize(_val: &Value) -> Result<Bytecode, Error> {
        todo!()
    }
}
impl Deserializer<Cardinality> for V2 {
    fn deserialize(val: &Value) -> Result<Cardinality, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_str() {
            "list" => Ok(Cardinality::List),
            "set" => Ok(Cardinality::Set),
            "single" => Ok(Cardinality::Single),
            _ => Err(Error::UnexpectedJson {
                msg: "".into(),
                value: val.clone(),
            }),
        }
    }
}
impl Deserializer<Direction> for V2 {
    fn deserialize(val: &Value) -> Result<Direction, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_str() {
            "OUT" => Ok(Direction::Out),
            "IN" => Ok(Direction::In),
            _ => Err(Error::UnexpectedJson {
                msg: "Json's wonky.".to_string(),
                value: val.clone(),
            }),
        }
    }
}
impl Deserializer<Token> for V2 {
    fn deserialize(val: &Value) -> Result<Token, Error> {
        let val = get_value!(val, Value::String)?;
        let token = Token::new(val.clone());
        Ok(token)
    }
}
impl Deserializer<Metrics> for V2 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let metric = get_value!(val, Value::Object)?.to_owned();
        // metric.remove().ok_or(Error::Missing())
        let duration = metric
            .get("dur")
            .ok_or(Error::Missing("dur"))?
            .deserialize::<Self, Double>()?;
        let id = metric
            .get("id")
            .ok_or(Error::Missing("id"))?
            .deserialize::<Self, String>()?;
        let name = metric
            .get("name")
            .ok_or(Error::Missing("name"))?
            .deserialize::<Self, String>()?;
        let counts = get_value!(
            metric.get("counts").ok_or(Error::Missing("counts"))?,
            Value::Object
        )?;
        let traversers = counts
            .get("traverserCount")
            .ok_or(Error::Missing("traverserCount"))?
            .deserialize::<Self, Long>()?;
        let count = counts
            .get("elementCount")
            .ok_or(Error::Missing("elementCount"))?
            .deserialize::<Self, Long>()?;
        let annotations = get_value!(
            metric
                .get("annotations")
                .map(|v| v.to_owned())
                .unwrap_or(Value::Object(serde_json::Map::new())),
            Value::Object
        )?;
        let perc_duration = annotations
            .get("percentDur")
            .ok_or(Error::Missing("percentDur"))?
            .deserialize::<Self, Double>()
            .unwrap_or(Double(0.0));
        let nested = get_value!(
            metric.get("metrics").ok_or(Error::Missing("metrics"))?,
            Value::Array
        )?
        .into_iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<Vec<_>, Error>>()?;
        let metric = Metrics::new(id, name, duration, count, traversers, perc_duration, nested);

        Ok(metric)
    }
}
impl Deserializer<HashMap<String, GValue>> for V2 {
    fn deserialize(val: &Value) -> Result<HashMap<String, GValue>, Error> {
        Ok(get_value!(val, Value::Object)?
            .into_iter()
            .map(|(k, v)| (k, v.deserialize::<Self, GValue>()))
            .map(|(k, result)| match result {
                Ok(v) => Ok((k.clone(), v)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<(_, _)>, Error>>()?
            .into_iter()
            .collect::<HashMap<_, _>>())
    }
}
impl Deserializer<String> for V2 {
    fn deserialize(val: &Value) -> Result<String, Error> {
        let str = get_value!(val, Value::String)?;
        Ok(str.into())
    }
}
impl Deserializer<TraversalMetrics> for V2 {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let metrics = get_value!(val, Value::Object)?;

        let duration = metrics
            .get("dur")
            .ok_or(Error::Missing("dur"))?
            .deserialize::<Self, Double>()?;
        let metrics = get_value!(
            metrics.get("metrics").ok_or(Error::Missing("metrics"))?,
            Value::Array
        )?
        .into_iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<Vec<_>, Error>>()?;

        Ok(TraversalMetrics::new(duration, metrics).into())
    }
}
impl Deserializer<Traverser> for V2 {
    fn deserialize(_val: &Value) -> Result<Traverser, Error> {
        todo!()
    }
}
