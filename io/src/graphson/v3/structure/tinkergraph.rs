use crate::graphson::prelude::*;
use crate::graphson::tags::Typed;

impl<D: Dialect> GraphsonDeserializer<TinkerGraph, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<TinkerGraph, Error> {
        let _debug = val.to_string();
        let vertex_values = get_value!(val.ensure("vertices")?, Value::Array)?;
        let edge_values = get_value!(val.ensure("edges")?, Value::Array)?;
        let vertices = vertex_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, D, Vertex>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, _>>()?;
        let edges = edge_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, D, Edge>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, _>>()?;

        Ok(TinkerGraph { vertices, edges })
    }
}

impl<D: Dialect> GraphsonSerializer<TinkerGraph, D> for GraphSON<V3> {
    fn serialize(val: &TinkerGraph) -> Result<Value, Error> {
        let vertices = val
            .vertices
            .iter()
            .map(|v| {
                v.serialize::<Self, D>().map(|value| {
                    json!({
                        "@type": D::tag::<Vertex>(),
                        "@value": value
                    })
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let edges = val
            .edges
            .iter()
            .map(|v| {
                v.serialize::<Self, D>().map(|value| {
                    json!({
                        "@type": D::tag::<Edge>(),
                        "@value": value
                    })
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(json!({
            "vertices": vertices,
            "edges": edges,
        }))
    }
}
