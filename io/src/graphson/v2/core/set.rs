//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_set

use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonSerializer<Set, D> for GraphSON<V2>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(val: &Set) -> Result<Value, Error> {
        let elements = val
            .iter()
            .map(|v| v.serialize::<Self, D>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(json!(elements))
    }
}
