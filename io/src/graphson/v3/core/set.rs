//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_set

use crate::graphson::prelude::*;
use indexmap::IndexSet;

impl Deserializer<Set> for V3 {
    fn deserialize(val: &Value) -> Result<Set, Error> {
        let set = get_value!(val, Value::Array)
            .ctx::<Set>()?
            .iter()
            .map(|v| v.deserialize::<Self, GValue>())
            .collect::<Result<Vec<_>, _>>()
            .ctx::<Set>()?
            .into_iter()
            .fold(IndexSet::new(), |mut set, item| {
                set.insert(item);
                set
            });
        Ok(set.into())
    }
}

impl Serializer<Set> for V3 {
    fn serialize(val: &Set) -> Result<Value, Error> {
        let elements = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<Value>, _>>()
            .ctx::<Set>()?;

        if elements.is_empty() {
            // Why.
            Ok(json!({
                "@type": Tag::Set,
                "@value": [ ],
            }))
        } else {
            Ok(json!({
                "@type": Tag::Set,
                "@value": elements,
            }))
        }
    }
}
