use crate::io::graphson::Tag;
use crate::io::graphson::prelude::*;

impl<K, V> Deserializer<Map<K, V>> for V3
where
    Self: Deserializer<K> + Deserializer<V>,
    K: std::hash::Hash + Eq,
{
    fn deserialize(val: &Value) -> Result<Map<K, V>, Error> {
        let val = get_value!(val, Value::Array)?;
        let mut map = Map::new();
        if !val.is_empty() {
            let mut x = 0;
            while x < val.len() {
                let key = val[x].deserialize::<Self, K>()?;
                let vald = &val[x + 1];
                let _debug_val = format!("{}", &vald);
                let value = vald.deserialize::<Self, V>()?;
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
