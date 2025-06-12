//! https://tinkerpop.apache.org/docs/3.7.3/dev/io/#_set

use crate::graphson::prelude::*;

impl Serializer<Set> for V2 {
    fn serialize(val: &Set) -> Result<Value, Leaf> {
        let elements = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<Value>, Error>>()?;
        Ok(json!(elements))
    }
}
