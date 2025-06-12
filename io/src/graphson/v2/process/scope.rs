use crate::graphson::prelude::*;

impl Serializer<Scope> for V2 {
    fn serialize(val: &Scope) -> Result<Value, Leaf> {
        let v = match val {
            Scope::Global => "global",
            Scope::Local => "local",
        };

        Ok(json!({
            "@type" : Tag::Scope,
            "@value" : v
        }))
    }
}
