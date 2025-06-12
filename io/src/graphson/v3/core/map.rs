use crate::graphson::Tag;
use crate::graphson::prelude::*;

impl<K, V> Deserializer<Map<K, V>> for V3
where
    Self: Deserializer<K> + Deserializer<V>,
    K: std::hash::Hash + Eq,
    K: Object,
    V: Object,
{
    fn deserialize(val: &Value) -> Result<Map<K, V>, Error> {
        let val = get_value!(val, Value::Array).ctx::<Map<K, V>>()?;
        let mut map = Map::new();
        if !val.is_empty() {
            let mut x = 0;
            while x < val.len() {
                let key = val[x].deserialize::<Self, K>().ctx::<Map<K, V>>()?;
                let value = val[x + 1].deserialize::<Self, V>().ctx::<Map<K, V>>()?;
                map.insert(key, value);
                x += 2;
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
