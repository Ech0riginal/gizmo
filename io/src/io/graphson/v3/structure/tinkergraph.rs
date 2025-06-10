use crate::io::graphson::prelude::*;

impl Deserializer<TinkerGraph> for V3 {
    fn deserialize(val: &Value) -> Result<TinkerGraph, Error> {
        let _debug = val.to_string();
        let vertex_values = get_value!(val.ensure("vertices")?, Value::Array)?;
        let edge_values = get_value!(val.ensure("edges")?, Value::Array)?;
        let vertices = vertex_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Vertex>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, Error>>()?;
        let edges = edge_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, Edge>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, Error>>()?;

        Ok(TinkerGraph { vertices, edges })
    }
}

impl Serializer<TinkerGraph> for V3 {
    fn serialize(val: &TinkerGraph) -> Result<Value, Error> {
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
