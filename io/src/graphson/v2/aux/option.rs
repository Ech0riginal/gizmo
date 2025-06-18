use crate::graphson::prelude::*;

impl<T, D> GraphsonSerializer<Option<T>, D> for GraphSON<V2>
where
    Self: GraphsonSerializer<T, D>,
    D: Dialect,
    T: Object,
{
    fn serialize(val: &Option<T>) -> Result<Value, Error> {
        match val {
            None => Ok(Value::Null),
            Some(inner) => inner.serialize::<Self, D>(),
        }
    }
}