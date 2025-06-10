use crate::io::graphson::prelude::*;

impl Serializer<Pop> for V2 {
    fn serialize(val: &Pop) -> Result<Value, Error> {
        let str = match val {
            Pop::All => "all",
            Pop::First => "first",
            Pop::Last => "last",
            Pop::Mixed => "mixed",
        };
        Ok(json!({
            "@type": Tag::Pop,
            "@value": str,
        }))
    }
}
