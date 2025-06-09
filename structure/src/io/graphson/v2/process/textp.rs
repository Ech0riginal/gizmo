use crate::io::graphson::prelude::*;

impl Serializer<TextP> for V2 {
    fn serialize(val: &TextP) -> Result<Value, Error> {
        Ok(json!({
            "@type" : Tag::TextP,
            "@value" : {
                "predicate" : val.operator(),
                "value" : val.value().serialize::<Self>()?
            }
        }))
    }
}
