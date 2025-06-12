use crate::graphson::prelude::*;

impl Deserializer<TinkerGraph> for V3 {
    fn deserialize(val: &Value) -> Result<TinkerGraph, Leaf> {
        let _debug = val.to_string();
        let vertex_values = get_value!(val.ensure("vertices").ctx::<TinkerGraph>()?, Value::Array)
            .ctx::<TinkerGraph>()?;
        let edge_values = get_value!(val.ensure("edges").ctx::<TinkerGraph>()?, Value::Array)
            .ctx::<TinkerGraph>()?;
        let vertices = vertex_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Vertex>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, Leaf>>()
            .ctx::<TinkerGraph>()?;
        let edges = edge_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Edge>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, Leaf>>()
            .ctx::<TinkerGraph>()?;

        Ok(TinkerGraph { vertices, edges })
    }
}

impl Serializer<TinkerGraph> for V3 {
    fn serialize(val: &TinkerGraph) -> Result<Value, Leaf> {
        let vertices = val
            .vertices
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, _>>()
            .ctx::<TinkerGraph>()?;
        let edges = val
            .edges
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, _>>()
            .ctx::<TinkerGraph>()?;

        Ok(json!({
            "@type": Tag::TinkerGraph,
            "@value": {
                "vertices": vertices,
                "edges": edges,
            }
        }))
    }
}
