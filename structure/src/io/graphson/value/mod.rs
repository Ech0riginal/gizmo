mod v2;
mod v3;

use crate::io::graphson::prelude::*;
impl<T> Serializer<Option<T>> for V2
where
    V2: Serializer<T>,
{
    fn serialize(val: &Option<T>) -> Result<Value, Error> {
        match val {
            None => Ok(Value::Null),
            Some(inner) => inner.serialize::<Self>(),
        }
    }
}
