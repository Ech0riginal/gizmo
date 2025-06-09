//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_list

use crate::io::graphson::prelude::*;

// TODO implement Deserializer<List> for V2 just so we have clear IR

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
        if val.to_string().contains("[null]") {
            return Ok(List::new(vec![]));
        }
        let val = get_value!(val, Value::Array)?;
        let _debug_val = val.iter().map(|v| format!("{:?}", v)).collect::<Vec<_>>();

        let mut elements = Vec::with_capacity(val.len());
        for item in val {
            let result = item.deserialize::<Self, GValue>();
            elements.push(result?);
        }
        Ok(elements.into())
    }
}
