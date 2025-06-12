//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_list

use crate::graphson::prelude::*;

// TODO implement Deserializer<List> for V2 just so we have clear IR

impl<T> Serializer<List<T>> for V2
where
    V2: Serializer<T>,
    T: Object,
{
    fn serialize(val: &List<T>) -> Result<Value, Error> {
        let value = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(json!(value))
    }
}
