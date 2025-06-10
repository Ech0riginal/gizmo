use crate::io::graphson::prelude::*;

impl Deserializer<Path> for V2 {
    fn deserialize(val: &Value) -> Result<Path, Error> {
        let labels = {
            let tmp = val.ensure("labels")?.deserialize::<Self, GValue>()?;
            Box::new(tmp)
        };
        let objects = {
            let tmp = val.ensure("objects")?.deserialize::<Self, GValue>()?;
            Box::new(tmp)
        };
        Ok(Path { labels, objects })
    }
}

impl Serializer<Path> for V2 {
    fn serialize(val: &Path) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : Tag::Path,
            "@value": {
                "labels" : (&*val.labels).serialize::<Self>()?,
                "objects" : (&*val.objects).serialize::<Self>()?,
            }
        }))
    }
}
