use crate::graphson::prelude::*;
use indexmap::IndexMap;

impl<K, V> Serializer<Map<K, V>> for V2
where
    Self: Serializer<K> + Serializer<V>,
    K: Object,
    V: Object,
{
    fn serialize(val: &Map<K, V>) -> Result<Value, Error> {
        let mapd = val
            .iter()
            .map(|(k, v)| (k.serialize::<Self>(), v.serialize::<Self>()))
            .filter(|(k, v)| k.is_ok() && v.is_ok())
            .map(|(k, v)| (k.unwrap(), v.unwrap()))
            .collect::<IndexMap<_, _>>();
        Ok(json!(mapd))
    }
}
