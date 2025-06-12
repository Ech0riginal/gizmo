use crate::graphson::prelude::*;

impl Deserializer<Path> for V3 {
    fn deserialize(val: &Value) -> Result<Path, Error> {
        let labels = val
            .ensure("labels")?
            .deserialize::<Self, GValue>()
            .map(Box::new)?;
        let objects = val
            .ensure("objects")?
            .deserialize::<Self, GValue>()
            .map(Box::new)?;
        Ok(Path { labels, objects })
    }
}
impl Serializer<Path> for V3 {
    fn serialize(val: &Path) -> Result<Value, Error> {
        Ok(json!({
            "@type" : Tag::Path,
            "@value": {
                "labels" : (*val.labels).serialize::<Self>()?,
                "objects" : (*val.objects).serialize::<Self>()?,
            }
        }))
    }
}
