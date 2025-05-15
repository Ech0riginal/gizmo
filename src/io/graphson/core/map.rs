use crate::io::graphson::prelude::*;

impl Serializer<Map> for V2 {
    fn serialize(val: &Map) -> Result<Value, Error> {
        let mapd = val
            .iter()
            .map(|(k, v)| (String::try_from(k), v.serialize::<Self>()))
            .filter(|(k, v)| k.is_ok() && v.is_ok())
            .map(|(k, v)| (k.unwrap(), v.unwrap()))
            .collect::<HashMap<_, _>>();
        Ok(json!(mapd))
    }
}

impl Deserializer<Map> for V3 {
    fn deserialize(val: &Value) -> Result<Map, Error> {
        let val = get_value!(val, Value::Array)?;
        let mut map = HashMap::new();
        if !val.is_empty() {
            let mut x = 0;
            while x < val.len() {
                let key_value = D::deserialize(&val[x])?;
                let key: GKey = GKey::from(key_value);
                let vald = &val[x + 1];
                let _debug_val = format!("{}", &vald);
                let value = D::deserialize(vald)?;
                map.insert(key, value);
                x += 2;
            }
        }
        Ok(Map(map).into())
    }
}

impl Serializer<Map> for V3 {
    fn serialize(val: &Map) -> Result<Value, Error> {
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
