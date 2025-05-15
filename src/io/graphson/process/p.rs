use crate::io::graphson::prelude::*;

impl Serializer<P> for V2 {
    fn serialize(val: &P) -> Result<Value, Error> {
        Ok(json!({
            "@type": P,
            "@value": {
                "predicate": val.operator,
                "value": (&*val.value).serialize::<Self>()?
            }
        }))
    }
}
