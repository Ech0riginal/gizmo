use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Tree, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Tree, Error> {
        let array = get_value!(val, Value::Array)?;
        let branches = array
            .iter()
            .map(|val| val.deserialize::<Self, D, Branch>())
            .collect::<Result<List<_>, _>>()?;
        Ok(Tree { branches })
    }
}

impl<D: Dialect> GraphsonDeserializer<Branch, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Branch, Error> {
        let obj = get_value!(val, Value::Object)?;
        let key = obj
            .ensure("key")
            .map(|value| value.deserialize::<Self, D, GValue>())??;
        let value = obj
            .ensure("value")
            .map(|value| value.deserialize::<Self, D, GValue>())??;
        Ok(Branch {
            key: Box::new(key),
            value: Box::new(value),
        })
    }
}

impl<D: Dialect> GraphsonSerializer<Tree, D> for GraphSON<V2> {
    fn serialize(val: &Tree) -> Result<Value, Error> {
        let branches = val
            .branches
            .iter()
            .map(|b| b.serialize::<Self, D>())
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(json!(branches))
    }
}
impl<D: Dialect> GraphsonSerializer<Branch, D> for GraphSON<V2> {
    fn serialize(val: &Branch) -> Result<Value, Error> {
        Ok(json!({
            "key": (*val.key).serialize::<Self, D>()?,
            "value": (*val.value).serialize::<Self, D>()?,
        }))
    }
}
