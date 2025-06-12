use crate::graphson::Tag;
use crate::graphson::prelude::*;
use std::arch::breakpoint;

impl<K, V> Deserializer<Map<K, V>> for V3
where
    Self: Deserializer<K> + Deserializer<V>,
    K: std::hash::Hash + Eq,
    K: Object,
    V: Object,
{
    fn deserialize(val: &Value) -> Result<Map<K, V>, Error> {
        let mut val = get_value!(val, Value::Array)?.to_owned();
        let mut map = Map::new();
        let mut x = 0;

        if !val.is_empty() {
            loop {
                let key = val[x].deserialize::<Self, K>()?;
                x += 1;
                let value = val[x].deserialize::<Self, V>()?;
                x += 1;
                map.insert(key, value);
                if x >= val.len() {
                    break;
                }
            }
        }

        Ok(map)
    }
}

impl<K, V> Serializer<Map<K, V>> for V3
where
    Self: Serializer<K> + Serializer<V>,
    K: Object,
    V: Object,
{
    fn serialize(val: &Map<K, V>) -> Result<Value, Error> {
        let mut values = vec![];
        for (k, v) in val.iter() {
            values.push(k.serialize::<Self>()?);
            values.push(v.serialize::<Self>()?);
        }
        Ok(json!({
            "@type": Tag::Map,
            "@value": values,
        }))
    }
}
