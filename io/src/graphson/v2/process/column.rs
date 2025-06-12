use crate::graphson::prelude::*;

impl Serializer<Column> for V2 {
    fn serialize(val: &Column) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Column,
            "@value" : match val {
                Column::Keys => "keys",
                Column::Values => "values",
            },
        }))
    }
}
