use crate::graphson::prelude::*;
use crate::graphson::tags::Typed;

impl<D: Dialect> GraphsonDeserializer<StarGraph, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<StarGraph, Error> {
        let value = val.ensure("starVertex")?;
        let vertex = value.typed()?.value.deserialize::<Self, D, Vertex>()?;
        let yikes = StarGraph::from(vertex);
        Ok(yikes)
    }
}

impl<D: Dialect> GraphsonSerializer<StarGraph, D> for GraphSON<V2> {
    fn serialize(val: &StarGraph) -> Result<Value, Error> {
        let binding = GValue::Vertex(val.into());
        Ok(json!(binding.serialize::<Self, D>()?))
    }
}
