use crate::io::graphson::prelude::*;

impl Deserializer<Path> for V2 {
    fn deserialize(val: &Value) -> Result<Path, Error> {
        let labels = val["labels"].deserialize::<Self, GValue>()?;
        let objects = val["objects"].deserialize::<Self, GValue>()?;
        Ok(Path::new(labels, objects).into())
    }
}

impl Serializer<Path> for V2 {
    fn serialize(val: &Path) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : PATH,
            "@value": {
                "labels" : (&*val.labels).serialize::<Self>()?,
                "objects" : (&*val.objects).serialize::<Self>()?,
            }
        }))
    }
}
