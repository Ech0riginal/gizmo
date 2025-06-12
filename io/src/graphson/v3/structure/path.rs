use crate::graphson::prelude::*;

impl Deserializer<Path> for V3 {
    fn deserialize(val: &Value) -> Result<Path, Leaf> {
        let labels = {
            let tmp = val
                .ensure("labels")
                .ctx::<Path>()?
                .deserialize::<Self, GValue>()
                .ctx::<Path>()?;
            Box::new(tmp)
        };
        let objects = {
            let tmp = val
                .ensure("objects")
                .ctx::<Path>()?
                .deserialize::<Self, GValue>()
                .ctx::<Path>()?;
            Box::new(tmp)
        };
        Ok(Path { labels, objects })
    }
}
impl Serializer<Path> for V3 {
    fn serialize(val: &Path) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Path,
            "@value": {
                "labels" : (*val.labels).serialize::<Self>()?,
                "objects" : (*val.objects).serialize::<Self>()?,
            }
        }))
    }
}
