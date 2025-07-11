use crate::formats::graphson::prelude::*;

impl<K, V, D: Dialect> GraphsonDeserializer<Map<K, V>, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<K, D> + GraphsonDeserializer<V, D>,
    K: std::hash::Hash + Eq + Named,
    V: Named,
{
    fn deserialize(val: &Value) -> Result<Map<K, V>, Error> {
        let val = get_value!(val, Value::Array)?.to_owned();
        let mut map = Map::new();
        let mut x = 0;

        if !val.is_empty() {
            loop {
                let key = val[x].deserialize::<Self, D, K>()?;
                x += 1;
                let value = val[x].deserialize::<Self, D, V>()?;
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

impl<K, V, D: Dialect> GraphsonSerializer<Map<K, V>, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<K, D> + GraphsonSerializer<V, D>,
    K: SerializeExt + Named,
    V: SerializeExt + Named,
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
