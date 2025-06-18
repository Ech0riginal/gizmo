//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_list

use crate::graphson::prelude::*;

// TODO implement Deserializer<List> for V2 just so we have clear IR

impl<T, D: Dialect> GraphsonSerializer<List<T>, D> for GraphSON<V2>
where
    Self: GraphsonSerializer<T, D>,
    T: SerializeExt,
{
    fn serialize(val: &List<T>) -> Result<Value, Error> {
        let value = val
            .iter()
            .map(|v| v.serialize::<Self, D>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(json!(value))
    }
}
