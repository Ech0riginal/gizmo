use crate::io::graphson::prelude::*;

impl Deserializer<Vertex> for V3 {
    fn deserialize(val: &Value) -> Result<Vertex, Error> {
        let map = get_value!(val, Value::Object)?;
        let id = map
            .get("id")
            .ok_or("id".missing())?
            .deserialize::<Self, GID>()?;
        let label = map
            .get("label")
            .ok_or("label".missing())?
            .deserialize::<Self, String>()?;
        let properties = {
            let properties_val = map.get("properties").ok_or("properties".missing())?;
            let props = get_value!(properties_val, Value::Object)?;
            let mut map = Map::new();

            for (key, value) in props.into_iter() {
                let properties = get_value!(value, Value::Array)?
                    .into_iter()
                    .map(|item| item.typed())
                    .collect::<Result<Vec<Type<'_>>, Error>>()?
                    .into_iter()
                    .map(|typed| typed.value.deserialize::<Self, VertexProperty>())
                    .collect::<Result<List<VertexProperty>, Error>>()?;
                map.insert(key.to_string(), properties);
            }

            map
        };

        Ok(Vertex {
            id,
            label,
            properties,
        })
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
