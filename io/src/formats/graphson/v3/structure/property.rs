use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Property, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Property, Error> {
        let val = get_value!(val, Value::Object)?;
        let key = val.ensure("key")?.deserialize::<Self, D, String>()?;
        let value = Box::new(val.ensure("value")?.deserialize::<Self, D, GValue>()?);
        let mut element = Box::new(GValue::Null);

        if let Some(el) = val.get("element") {
            element = Box::new(el.deserialize::<Self, D, GValue>()?);
        }

        Ok(Property {
            key,
            value,
            element,
        })
    }
}

impl<D: Dialect> GraphsonSerializer<Property, D> for GraphSON<V3> {
    fn serialize(val: &Property) -> Result<Value, Error> {
        Ok(json!({
          "key" : val.key.serialize::<Self, D>()?,
          "value" : (*val.value).serialize::<Self, D>()?,
        }))
    }
}
