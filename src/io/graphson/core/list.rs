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
        if val.to_string().contains("[null]") {
            // TODO Speak to the sKG lads about this
            return Ok(GValue::List(List::new(vec![])));
        }
        let val = get_value!(val, Value::Array)?;
        let _debug_val = val.iter().map(|v| format!("{:?}", v)).collect::<Vec<_>>();

        let mut elements = Vec::with_capacity(val.len());
        for item in val {
            let deserialized = D::deserialize(item)?;
            elements.push(deserialized);
        }
        Ok(elements.into())
    }
}
