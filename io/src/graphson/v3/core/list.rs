//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_list

use crate::graphson::prelude::*;

impl<T> Serializer<List<T>> for V3
where
    V3: Serializer<T>,
    T: Object,
{
    fn serialize(val: &List<T>) -> Result<Value, Leaf> {
        let value = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<Value>, Leaf>>()
            .ctx::<List<T>>()?;
        Ok(json!({
            "@type" : Tag::List,
            "@value" : value
        }))
    }
}

impl<T> Deserializer<List<T>> for V3
where
    V3: Deserializer<T>,
    T: Object,
{
    fn deserialize(val: &Value) -> Result<List<T>, Leaf> {
        if val.to_string().contains("[null]") {
            return Ok(list![]);
        }
        let val = get_value!(val, Value::Array).ctx::<List<T>>()?;
        let _debug_val = val.iter().map(|v| format!("{v:?}")).collect::<Vec<_>>();

        let mut elements = List::with_capacity(val.len());
        for item in val {
            let result = item.deserialize::<Self, T>();
            elements.push(result?);
        }
        Ok(elements)
    }
}
