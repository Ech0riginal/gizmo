use crate::io::graphson::prelude::*;

impl Deserializer<StarGraph> for V2 {
    fn deserialize(val: &Value) -> Result<StarGraph, Error> {
        let value = val.get("starVertex").ok_or(Error::UnexpectedJson {
            msg: "Malformed StarGraph.".to_string(),
            value: val.clone(),
        })?;
        let vertex = value.typed()?.value.deserialize::<Self, Vertex>()?;
        let yikes = StarGraph::from(vertex);
        Ok(yikes)
    }
}

impl Serializer<StarGraph> for V2 {
    fn serialize(val: &StarGraph) -> Result<serde_json::Value, Error> {
        let binding = GValue::Vertex(val.into());
        Ok(json!({"starVertex": binding.serialize::<Self>()?,}))
    }
}
