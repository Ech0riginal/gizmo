use crate::graphson::prelude::*;

impl Deserializer<Vertex> for V3 {
    fn deserialize(val: &Value) -> Result<Vertex, Error> {
        let map = get_value!(val, Value::Object).ctx::<Vertex>()?;
        let id = map
            .ensure("id")
            .ctx::<Vertex>()?
            .deserialize::<Self, GID>()?;
        let label = map
            .ensure("label")
            .ctx::<Vertex>()?
            .deserialize::<Self, String>()?;
        let properties = if let Some(properties_val) = map.get("properties") {
            let props = get_value!(properties_val, Value::Object).ctx::<Vertex>()?;
            let mut map = Map::new();

            for (key, value) in props.into_iter() {
                let properties = get_value!(value, Value::Array)
                    .ctx::<Vertex>()?
                    .iter()
                    .map(|item| item.typed())
                    .collect::<Result<Vec<Type<'_>>, Error>>()
                    .ctx::<Vertex>()?
                    .into_iter()
                    .map(|typed| typed.value.deserialize::<Self, VertexProperty>())
                    .collect::<Result<List<VertexProperty>, Error>>()
                    .ctx::<Vertex>()?;
                map.insert(key.to_string(), properties);
            }

            map
        } else {
            Default::default()
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
        // 'embedded' Vertices don't include 'properties'?
        // If I have to refactor this whole thing to
        // include a Context I'm naming this flag is_stupid
        if val.properties.is_empty() {
            serialize_small_vertex(val)
        } else {
            serialize_big_vertex(val)
        }
    }
}

fn serialize_big_vertex(v: &Vertex) -> Result<Value, Error> {
    let mut properties = serde_json::Map::new();
    for (k, v) in v.properties.iter() {
        let mut props = vec![];
        for i in v.iter() {
            let result = i.serialize::<V3>();
            props.push(result?);
        }
        properties.insert(k.to_string(), Value::Array(props));
    }

    Ok(json!({
        "@type": Tag::Vertex,
        "@value": {
            "id": v.id.serialize::<V3>()?,
            "label": v.label.serialize::<V3>()?,
            "properties": properties,
        }
    }))
}

fn serialize_small_vertex(v: &Vertex) -> Result<Value, Error> {
    Ok(json!({
        "@type": Tag::Vertex,
        "@value": {
            "id": v.id.serialize::<V3>()?,
            "label": v.label.serialize::<V3>()?,
        }
    }))
}
