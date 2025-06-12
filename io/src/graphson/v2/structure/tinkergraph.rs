use crate::graphson::prelude::*;

impl Deserializer<TinkerGraph> for V2 {
    fn deserialize(val: &Value) -> Result<TinkerGraph, Error> {
        let vertex_values = get_value!(
            val.ensure("vertices")?,
            Value::Array
        )?;
        let edge_values = get_value!(
            val.ensure("edges")?,
            Value::Array
        )?;
        let vertices = vertex_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Vertex>(),
                Err(e) => Err(e.into()),
            })
            .collect::<Result<List<_>, Error>>()?;
        let edges = edge_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Edge>(),
                Err(e) => Err(e.into()),
            })
            .collect::<Result<List<_>, Error>>()?;

        Ok(TinkerGraph { vertices, edges })
    }
}

impl Serializer<TinkerGraph> for V2 {
    fn serialize(val: &TinkerGraph) -> Result<Value, Leaf> {
        let vertices = val
            .vertices
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?;
        let edges = val
            .edges
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(json!({
            "@type": Tag::TinkerGraph,
            "@value": {
                "vertices": vertices,
                "edges": edges,
            }
        }))
    }
}
