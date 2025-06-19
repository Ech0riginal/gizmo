use crate::formats::graphson::prelude::*;

impl<K, V, D: Dialect> GraphsonSerializer<Map<K, V>, D> for GraphSON<V2>
where
    Self: GraphsonSerializer<K, D> + GraphsonSerializer<V, D>,
    K: SerializeExt + Object,
    V: SerializeExt + Object,
{
    fn serialize(val: &Map<K, V>) -> Result<Value, Error> {
        let mut values = vec![];
        for (k, v) in val.iter() {
            values.push(k.serialize::<Self, D>()?);
            values.push(v.serialize::<Self, D>()?);
        }
        Ok(json!(values))
    }
}

impl<D: Dialect> GraphsonSerializer<Map<Value, Value>, D> for GraphSON<V2> {
    fn serialize(val: &Map<Value, Value>) -> Result<Value, Error> {
        Ok(json!(val.0))
    }
}
