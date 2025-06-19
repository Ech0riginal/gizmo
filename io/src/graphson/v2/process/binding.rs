use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Binding, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Binding, Error> {
        let obj = get_value!(val, Value::Object)?;
        let key = get_value!(obj.ensure("key")?, Value::String)?.clone();
        let value = obj
            .ensure("value")?
            .deserialize::<Self, D, GValue>()?
            .boxed();
        Ok(Binding { key, value })
    }
}

impl<D: Dialect> GraphsonSerializer<Binding, D> for GraphSON<V2> {
    fn serialize(val: &Binding) -> Result<Value, Error> {
        val.value.serialize::<Self, D>().map(|value| {
            json!({
                "key": val.key,
                "value": value,
            })
        })
    }
}
