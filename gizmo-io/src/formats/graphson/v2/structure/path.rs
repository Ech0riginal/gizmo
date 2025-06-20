use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Path, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Path, Error> {
        let labels = {
            let tmp = val.ensure("labels")?.deserialize::<Self, D, GValue>()?;
            Box::new(tmp)
        };
        let objects = {
            let tmp = val.ensure("objects")?.deserialize::<Self, D, GValue>()?;
            Box::new(tmp)
        };
        Ok(Path { labels, objects })
    }
}

impl<D: Dialect> GraphsonSerializer<Path, D> for GraphSON<V2> {
    fn serialize(val: &Path) -> Result<Value, Error> {
        Ok(json!({
            "labels" : (*val.labels).serialize::<Self, D>()?,
            "objects" : (*val.objects).serialize::<Self, D>()?,
        }))
    }
}
