use crate::graphson::prelude::*;

impl Deserializer<StarGraph> for V2 {
    fn deserialize(val: &Value) -> Result<StarGraph, Error> {
        let value = val.ensure("starVertex")?;
        let vertex = value.typed()?.value.deserialize::<Self, Vertex>()?;
        let yikes = StarGraph::from(vertex);
        Ok(yikes)
    }
}

impl Serializer<StarGraph> for V2 {
    fn serialize(val: &StarGraph) -> Result<Value, Error> {
        let binding = GValue::Vertex(val.into());
        Ok(json!({"starVertex": binding.serialize::<Self>()?,}))
    }
}
