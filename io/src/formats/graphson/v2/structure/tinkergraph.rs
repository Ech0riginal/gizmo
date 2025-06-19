use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TinkerGraph, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<TinkerGraph, Error> {
        let vertex_values = get_value!(val.ensure("vertices")?, Value::Array)?;
        let edge_values = get_value!(val.ensure("edges")?, Value::Array)?;
        let vertices: List<Vertex> = vertex_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, D, Vertex>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, Error>>()?;
        let edges = edge_values
            .iter()
            .map(|val| match val.typed() {
                Ok(type_) => type_.value.deserialize::<Self, D, Edge>(),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, Error>>()?;

        Ok(TinkerGraph { vertices, edges })
    }
}

impl<D: Dialect> GraphsonSerializer<TinkerGraph, D> for GraphSON<V2> {
    fn serialize(val: &TinkerGraph) -> Result<Value, Error> {
        let vertices = val
            .vertices
            .iter()
            .map(|v| {
                v.serialize::<Self, D>().map(|value| {
                    json!({
                        "@type": D::tag::<Vertex>(),
                        "@value": value,
                    })
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let edges = val
            .edges
            .iter()
            .map(|v| {
                v.serialize::<Self, D>().map(|value| {
                    json!({
                        "@type": D::tag::<Edge>(),
                        "@value": value,
                    })
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(json!({
            "vertices": vertices,
            "edges": edges,
        }))
    }
}
