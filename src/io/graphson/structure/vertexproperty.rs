use crate::io::graphson::prelude::*;
use std::collections::HashMap;

impl Deserializer<super::VertexProperties> for V2 {
    fn deserialize(val: &Value) -> Result<super::VertexProperties, Error> {
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

impl Serializer<VertexProperty> for V2 {
    fn serialize(val: &VertexProperty) -> Result<serde_json::Value, Error> {
        let mut root = HashMap::<&'static str, Value>::new();
        let mut value = HashMap::<&'static str, Value>::new();

        value.insert("id", val.id().serialize::<Self>()?);
        value.insert("value", (&*val.value).serialize::<Self>()?);
        value.insert("label", serde_json::to_value(&val.label)?);
        if let Some(id) = &val.vertex {
            value.insert("vertex", id.serialize::<Self>()?);
        }
        if let Some(properties) = &val.properties {
            let map = properties
                .iter()
                .map(|(k, v)| (k, v.serialize::<Self>()))
                .map(|(k, result)| match result {
                    Ok(v) => Ok((k, v)),
                    Err(e) => Err(e),
                })
                .collect::<Result<HashMap<&String, Value>, Error>>()?;
            value.insert("properties", serde_json::to_value(&map)?);
        }

        root.insert("@type", Value::String(VERTEX_PROPERTY.into()));
        root.insert("@value", serde_json::to_value(&value)?);

        let json = json!(root);

        Ok(json)
    }
}
