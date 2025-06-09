use crate::io::graphson::prelude::*;

impl Serializer<Map2> for V2 {
    fn serialize(val: &Map2) -> Result<Value, Error> {
        let mapd = val
            .iter()
            .map(|(k, v)| (k.to_string(), v.serialize::<Self>()))
            .filter(|(k, v)| v.is_ok())
            .map(|(k, v)| (k, v.unwrap()))
            .collect::<HashMap<_, _>>();
        Ok(json!(mapd))
    }
}

impl Deserializer<Map2> for V3 {
    fn deserialize(val: &Value) -> Result<Map2, Error> {
        let val = get_value!(val, Value::Array)?;
        let mut map = HashMap::new();
        if !val.is_empty() {
            let mut x = 0;
            while x < val.len() {
                let key = val[x].deserialize::<Self, GValue>()?;
                let vald = &val[x + 1];
                let _debug_val = format!("{}", &vald);
                let value = vald.deserialize::<Self, GValue>()?;
                map.insert(key, value);
                x += 2;
            }
        }
        Ok(Map2(map).into())
    }
}

impl Serializer<Map2> for V3 {
    fn serialize(val: &Map2) -> Result<Value, Error> {
        let keys = val
            .keys()
            .map(|k| k.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?
            .into_iter();
        let values = val
            .values()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?
            .into_iter();
        let value = keys.zip(values).collect::<HashMap<Value, Value>>();
        Ok(json!({
            "@type": MAP,
            "@value": value,
        }))
    }
}
