//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_list

use crate::io::graphson::prelude::*;

impl Serializer<List> for V2 {
    fn serialize(val: &List) -> Result<Value, Error> {
        let value = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<Value>, Error>>()?;
        Ok(json!(value))
    }
}

impl Serializer<List> for V3 {
    fn serialize(val: &List) -> Result<Value, Error> {
        let value = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<Value>, Error>>()?;
        Ok(json!({
            "@type" : LIST,
            "@value" : value
        }))
    }
}

impl Deserializer<List> for V3 {
    fn deserialize(val: &Value) -> Result<List, Error> {
        get_value!(val, Value::Array)?
            .into_iter()
            .map(|v| v.deserialize::<Self, GValue>())
            .collect::<Result<Vec<_>, Error>>()
            .map(|vec| vec.into())
    }
}
