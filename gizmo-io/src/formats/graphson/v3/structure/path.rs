use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Path, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<GValue, D>,
{
    fn deserialize(val: &Value) -> Result<Path, Error> {
        let labels = val
            .ensure("labels")?
            .deserialize::<Self, D, GValue>()
            .map(Box::new)?;
        let objects = val
            .ensure("objects")?
            .deserialize::<Self, D, GValue>()
            .map(Box::new)?;
        Ok(Path { labels, objects })
    }
}
impl<D: Dialect> GraphsonSerializer<Path, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(val: &Path) -> Result<Value, Error> {
        Ok(json!({
            "labels" : (*val.labels).serialize::<Self, D>()?,
            "objects" : (*val.objects).serialize::<Self, D>()?,
        }))
    }
}
