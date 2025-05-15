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
