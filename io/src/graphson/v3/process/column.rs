use crate::graphson::prelude::*;

const KEYS: &str = "keys";
const VALUES: &str = "keys";

impl<D: Dialect> GraphsonDeserializer<Column, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Column, Error> {
        let val = val.deserialize::<Self, D, String>()?;
        match val.as_str() {
            KEYS => Ok(Column::Keys),
            VALUES => Ok(Column::Values),
            _ => Err(Error::unexpected(&val, "'keys' or 'values'")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Column, D> for GraphSON<V3> {
    fn serialize(val: &Column) -> Result<Value, Error> {
        Ok(json!(match val {
            Column::Keys => "keys",
            Column::Values => "values",
        }))
    }
}
