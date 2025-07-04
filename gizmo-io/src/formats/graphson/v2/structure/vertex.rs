use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Vertex, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Vertex, Error> {
        let label = val
            .get("label")
            .map(|f| get_value!(f, Value::String).map(Clone::clone))
            .unwrap_or_else(|| Ok(String::from("vertex")))?;
        let id = val["id"].deserialize::<Self, D, GID>()?;
        let properties =
            val["properties"].deserialize::<Self, D, Map<String, List<VertexProperty>>>()?;
        let vertex = Vertex {
            id,
            label,
            properties,
        };

        Ok(vertex)
    }
}

impl<D: Dialect> GraphsonSerializer<Vertex, D> for GraphSON<V2> {
    fn serialize(val: &Vertex) -> Result<Value, Error> {
        let _root = IndexMap::<&'static str, Value>::new();
        let mut value = IndexMap::<&'static str, Value>::new();

        value.insert("id", val.id.serialize::<Self, D>()?);
        value.insert("label", serde_json::to_value(&val.label)?);
        if !val.properties.is_empty() {
            let properties = val
                .properties
                .iter()
                .map(|(label, properties)| {
                    (
                        label.clone(),
                        properties
                            .iter()
                            .flat_map(|vp| {
                                vp.serialize::<Self, D>().map(|value| {
                                    json!({
                                        "@type": D::tag::<VertexProperty>(),
                                        "@value": value,
                                    })
                                })
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<IndexMap<String, Vec<Value>>>();
            value.insert("properties", serde_json::to_value(&properties)?);
        }

        Ok(json!(value))
    }
}
