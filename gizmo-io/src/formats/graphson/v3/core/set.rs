//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_set

use crate::formats::graphson::prelude::*;
use indexmap::IndexSet;

impl<D: Dialect> GraphsonDeserializer<Set, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<GValue, D>,
{
    fn deserialize(val: &Value) -> Result<Set, Error> {
        let set = get_value!(val, Value::Array)?
            .iter()
            .map(|v| v.deserialize::<Self, D, GValue>())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(IndexSet::new(), |mut set, item| {
                set.insert(item);
                set
            });
        Ok(set.into())
    }
}

impl<D: Dialect> GraphsonSerializer<Set, D> for GraphSON<V3> 
where
    Self: Serializer<GValue, Value, D>
{
    fn serialize(val: &Set) -> Result<Value, Error> {
        let elements = val
            .iter()
            .map(|v| v.serialize::<Self, D>())
            .collect::<Result<Vec<Value>, _>>()?;

        if elements.is_empty() {
            // Why.
            Ok(json!([]))
        } else {
            Ok(json!(elements))
        }
    }
}
