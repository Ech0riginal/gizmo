use crate::io::graphson::prelude::*;
use indexmap::{IndexMap, indexset};

impl Deserializer<Vertex> for V3 {
    fn deserialize(val: &Value) -> Result<Vertex, Error> {
        todo!()
    }
}

impl Serializer<Vertex> for V3 {
    fn serialize(val: &Vertex) -> Result<Value, Error> {
        let mut properties = serde_json::Map::new();

        for (k, v) in val.properties.iter() {
            let mut props = vec![];
            for i in v.iter() {
                let result = i.serialize::<Self>();
                props.push(result?);
            }
            properties.insert(k.to_string(), Value::Array(props));
        }

        Ok(json!({
            "@type": Tag::Vertex,
            "@value": {
                "id": val.id.serialize::<Self>()?,
                "label": val.label.serialize::<Self>()?,
                "properties": properties,
            }
        }))
    }
}
