use crate::formats::graphson::prelude::*;

impl<K, V, D: Dialect> GraphsonSerializer<Map<K, V>, D> for GraphSON<V2>
where
    Self: GraphsonSerializer<K, D> + GraphsonSerializer<V, D>,
    K: SerializeExt + Named,
    V: SerializeExt + Named,
{
    fn serialize(val: &Map<K, V>) -> Result<Value, Error> {
        let mut values_alt = Map::new();
        let mut values = vec![];
        for (k, v) in val.iter() {
            values_alt.insert(k.serialize::<Self, D>()?, v.serialize::<Self, D>()?);
            values.push(k.serialize::<Self, D>()?);
            values.push(v.serialize::<Self, D>()?);
        }
        Ok(json!(values_alt.0))
    }
}

impl<D: Dialect> GraphsonSerializer<Map<Value, Value>, D> for GraphSON<V2> {
    fn serialize(val: &Map<Value, Value>) -> Result<Value, Error> {
        Ok(json!(val.0))
    }
}

impl<D: Dialect> GraphsonDeserializer<Map<String, GValue>, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Map<String, GValue>, Error> {
        match val {
            Value::Array(array_map) => {
                let mut map = Map::new();

                for (k, v) in array_map
                    .chunks(2)
                    .map(|key_value| (&key_value[0], &key_value[1]))
                {
                    map.insert(
                        k.deserialize::<Self, D, String>()?,
                        v.deserialize::<Self, D, GValue>()?,
                    );
                }

                Ok(map)
            }
            Value::Object(obj_map) => {
                let mut map = Map::new();
                for (k, v) in obj_map.iter() {
                    map.insert(k.to_owned(), v.deserialize::<Self, D, GValue>()?);
                }
                Ok(map)
            }
            _ => Err(Error::unexpected(val, "an array or map")),
        }
    }
}

impl<D: Dialect> GraphsonDeserializer<Map<String, List<VertexProperty>>, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Map<String, List<VertexProperty>>, Error> {
        match val {
            Value::Object(o) => {
                let mut p = Map::new();
                for (k, v) in o {
                    match v {
                        Value::Array(arr) => {
                            let list = arr
                                .iter()
                                .map(|e| e.typed())
                                .collect::<Result<List<_>, _>>()?
                                .into_iter()
                                .map(|tt| tt.value.deserialize::<Self, D, VertexProperty>())
                                .collect::<Result<List<_>, _>>()?;
                            p.insert(k.clone(), list);
                        }
                        value => {
                            return Err(Error::Unexpected {
                                expectation: "array for properties".to_string(),
                                actual: format!("{value}"),
                                location: location!(),
                            });
                        }
                    };
                }
                Ok(p)
            }
            Value::Null => Ok(Map::new()),
            value => Err(Error::Unexpected {
                expectation: "object or null for properties".into(),
                actual: format!("{value}"),
                location: location!(),
            }),
        }
    }
}
