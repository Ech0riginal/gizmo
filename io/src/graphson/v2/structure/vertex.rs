use crate::graphson::prelude::*;

impl Deserializer<Vertex> for V2 {
    fn deserialize(val: &Value) -> Result<Vertex, Error> {
        let label = val
            .get("label")
            .map(|f| get_value!(f, Value::String).map(Clone::clone))
            .unwrap_or_else(|| Ok(String::from("vertex")))?;
        let id = val["id"].deserialize::<Self, GID>()?;
        let properties = val["properties"].deserialize::<Self, super::VertexProperties>()?;
        let vertex = Vertex {
            id,
            label,
            properties,
        };

        Ok(vertex)
    }
}

impl Serializer<Vertex> for V2 {
    fn serialize(val: &Vertex) -> Result<Value, Leaf> {
        let mut root = IndexMap::<&'static str, Value>::new();
        let mut value = IndexMap::<&'static str, Value>::new();

        value.insert("id", val.id().serialize::<Self>()?);
        value.insert("label", serde_json::to_value(val.label())?);
        if !val.properties.is_empty() {
            let properties = val
                .iter()
                .map(|(label, properties)| {
                    (
                        label.clone(),
                        properties
                            .iter()
                            .flat_map(|vp| vp.serialize::<Self>())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<IndexMap<String, Vec<Value>>>();
            value.insert("properties", serde_json::to_value(&properties)?);
        }
        root.insert("@type", Value::String(Tag::Vertex.into()));
        root.insert("@value", serde_json::to_value(&value)?);

        let json = json!(root);
        let _debug_info = serde_json::to_string_pretty(&json)?;

        Ok(json)
    }
}
