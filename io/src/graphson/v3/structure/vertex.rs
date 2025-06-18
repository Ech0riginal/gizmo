use crate::graphson::prelude::*;
use crate::graphson::tags::Typed;

impl<D: Dialect> GraphsonDeserializer<Vertex, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Vertex, Error> {
        let map = get_value!(val, Value::Object)?;
        let id = map.ensure("id")?.deserialize::<Self, D, GID>()?;
        let label = map.ensure("label")?.deserialize::<Self, D, String>()?;
        let properties = if let Some(properties_val) = map.get("properties") {
            let props = get_value!(properties_val, Value::Object)?;
            let mut map = Map::new();

            for (key, value) in props.into_iter() {
                let properties = get_value!(value, Value::Array)?
                    .iter()
                    .map(|item| item.typed())
                    .collect::<Result<List<_>, _>>()?
                    .into_iter()
                    .map(|typed| typed.value.deserialize::<Self, D, VertexProperty>())
                    .collect::<Result<List<_>, _>>()?;
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

impl<D: Dialect> GraphsonSerializer<Vertex, D> for GraphSON<V3> {
    fn serialize(val: &Vertex) -> Result<Value, Error> {
        // 'embedded' Vertices don't include 'properties'?
        // If I have to refactor this whole thing to
        // include a Context I'm naming this flag is_stupid
        if val.properties.is_empty() {
            serialize_small_vertex::<D>(val)
        } else {
            serialize_big_vertex::<D>(val)
        }
    }
}

fn serialize_big_vertex<D: Dialect>(v: &Vertex) -> Result<Value, Error> {
    let mut properties = serde_json::Map::new();
    for (k, v) in v.properties.iter() {
        let mut props = vec![];
        for i in v.iter() {
            let result = i.serialize::<GraphSON<V3>, D>().map(|value| {
                json!({
                    "@type": D::tag::<VertexProperty>(),
                    "@value": value,
                })
            });
            props.push(result?);
        }
        properties.insert(k.to_string(), Value::Array(props));
    }

    Ok(json!({
        "id": v.id.serialize::<GraphSON<V3>, D>()?,
        "label": v.label.serialize::<GraphSON<V3>, D>()?,
        "properties": properties,
    }))
}

fn serialize_small_vertex<D: Dialect>(v: &Vertex) -> Result<Value, Error> {
    Ok(json!({
        "id": v.id.serialize::<GraphSON<V3>, D>()?,
        "label": v.label.serialize::<GraphSON<V3>, D>()?,
    }))
}
