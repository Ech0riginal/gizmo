//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_list

use crate::formats::graphson::prelude::*;

// TODO implement Deserializer<List> for V2 just so we have clear IR

impl<T, D: Dialect> GraphsonDeserializer<List<T>, D> for GraphSON<V2>
where
    Self: GraphsonDeserializer<T, D>,
    T: SerializeExt + Named,
{
    fn deserialize(val: &Value) -> Result<List<T>, Error> {
        let values = get_value!(val, Value::Array)?;
        let collection = values
            .iter()
            .map(|v| v.deserialize::<Self, D, T>())
            .collect::<Result<List<_>, Error>>()?;
        Ok(collection)
    }
}

impl<T, D: Dialect> GraphsonSerializer<List<T>, D> for GraphSON<V2>
where
    Self: GraphsonSerializer<T, D>,
    T: SerializeExt + Named,
{
    fn serialize(val: &List<T>) -> Result<Value, Error> {
        let value = val
            .iter()
            .map(|v| v.serialize::<Self, D>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(json!(value))
    }
}
