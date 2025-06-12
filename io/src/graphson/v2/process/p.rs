use crate::graphson::prelude::*;

impl Serializer<P> for V2 {
    fn serialize(val: &P) -> Result<Value, Leaf> {
        Ok(json!({
            "@type": Tag::P,
            "@value": {
                "predicate": val.operator,
                "value": (*val.value).serialize::<Self>()?
            }
        }))
    }
}
