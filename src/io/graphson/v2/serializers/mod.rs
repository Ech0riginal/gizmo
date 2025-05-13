mod gid;
mod gvalue;
mod request;
mod response;

mod opt {
    use crate::io::{Error, Serialize, Serializer, V2};
    use serde_json::Value;

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
}
