use crate::formats::graphson::prelude::*;

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
        let props = val
            .properties
            .iter()
            .map(|(key, val)| {
                (
                    key.to_string(),
                    val.iter()
                        .map(|vp| GValue::from(vp.clone()))
                        .collect::<List<GValue>>(),
                )
            })
            .collect::<Map<String, List<GValue>>>();
        Ok(json!({
            "@type": D::tag::<Vertex>(),
            "@value": {
                "id": val.id.serialize::<Self, D>()?,
                "label": val.label.serialize::<Self, D>()?,
                "properties": props.serialize::<Self, D>()?,
            }
        }))
    }
}
