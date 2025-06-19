//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_list

use crate::formats::graphson::prelude::*;

impl<T: SerializeExt, D: Dialect> GraphsonSerializer<List<T>, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<T, D>,
    T: Object,
{
    fn serialize(val: &List<T>) -> Result<Value, Error> {
        let value = val
            .iter()
            .map(|v| v.serialize::<Self, D>())
            .collect::<Result<Vec<Value>, Error>>()?;
        Ok(json!(value))
    }
}

impl<T, D: Dialect> GraphsonDeserializer<List<T>, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<T, D>,
    T: Object,
{
    fn deserialize(val: &Value) -> Result<List<T>, Error> {
        if val.to_string().contains("[null]") {
            return Ok(list![]);
        }
        let val = get_value!(val, Value::Array)?;
        let _debug_val = val.iter().map(|v| format!("{v:?}")).collect::<Vec<_>>();

        let mut elements = List::with_capacity(val.len());
        for item in val {
            let result = item.deserialize::<Self, D, T>();
            elements.push(result?);
        }
        Ok(elements)
    }
}
