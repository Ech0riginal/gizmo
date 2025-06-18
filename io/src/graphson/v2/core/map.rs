use crate::graphson::prelude::*;
use indexmap::IndexMap;

impl<K, V, D: Dialect> GraphsonSerializer<Map<K, V>, D> for GraphSON<V2>
where
    Self: GraphsonSerializer<K, D> + GraphsonSerializer<V, D>,
    K: SerializeExt,
    V: SerializeExt,
{
    fn serialize(val: &Map<K, V>) -> Result<Value, Error> {
        let mapd = val
            .iter()
            .map(|(k, v)| (k.serialize::<Self, D>(), v.serialize::<Self, D>()))
            .filter(|(k, v)| k.is_ok() && v.is_ok())
            .map(|(k, v)| (k.unwrap(), v.unwrap()))
            .collect::<IndexMap<_, _>>();
        Ok(json!(mapd))
    }
}

impl<D: Dialect> GraphsonSerializer<Map<Value, Value>, D> for GraphSON<V2> {
    fn serialize(val: &Map<Value, Value>) -> Result<Value, Error> {
        Ok(json!(val.0))
    }
}
