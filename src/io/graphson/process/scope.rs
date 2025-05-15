use crate::io::graphson::prelude::*;

impl Serializer<Scope> for V2 {
    fn serialize(val: &Scope) -> Result<Value, Error> {
        let v = match val {
            Scope::Global => "global",
            Scope::Local => "local",
        };

        Ok(json!({
            "@type" : SCOPE,
            "@value" : v
        }))
    }
}
