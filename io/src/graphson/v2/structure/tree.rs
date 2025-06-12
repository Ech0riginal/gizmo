use crate::graphson::prelude::*;

impl Deserializer<Tree> for V2 {
    fn deserialize(val: &Value) -> Result<Tree, Error> {
        let array = get_value!(val, Value::Array)?;
        let branches = array
            .iter()
            .map(|val| val.deserialize::<Self, Branch>())
            .collect::<Result<List<_>, Error>>()?;
        Ok(Tree { branches })
    }
}

impl Deserializer<Branch> for V2 {
    fn deserialize(val: &Value) -> Result<Branch, Error> {
        let obj = get_value!(val, Value::Object)?;

        let key = obj
            .get("key")
            .ok_or(Error::UnexpectedJson {
                msg: "Missing 'key' key".to_string(),
                value: val.clone(),
            })
            .map(|value| value.deserialize::<Self, GValue>())??;

        let value = obj
            .get("value")
            .ok_or(Error::UnexpectedJson {
                msg: "Missing 'value' key".to_string(),
                value: val.clone(),
            })
            .map(|value| value.deserialize::<Self, GValue>())??;

        Ok(Branch {
            key: Box::new(key),
            value: Box::new(value),
        })
    }
}

impl Serializer<Tree> for V2 {
    fn serialize(val: &Tree) -> Result<Value, Leaf> {
        let branches = val
            .branches
            .iter()
            .map(|b| b.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(json!({
            "@type": Tag::Tree,
            "@value": branches,
        }))
    }
}
impl Serializer<Branch> for V2 {
    fn serialize(val: &Branch) -> Result<Value, Leaf> {
        Ok(json!({
            "key": (*val.key).serialize::<Self>()?,
            "value": (*val.value).serialize::<Self>()?,
        }))
    }
}
