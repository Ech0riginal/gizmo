use crate::graphson::Tag;
use crate::graphson::prelude::*;

impl Deserializer<Property> for V3 {
    fn deserialize(val: &Value) -> Result<Property, Leaf> {
        let val = get_value!(val, Value::Object).ctx::<Property>()?;
        let key = val
            .ensure("key")
            .ctx::<Property>()?
            .deserialize::<Self, String>()
            .ctx::<Property>()?;
        let value = Box::new(
            val.ensure("value")
                .ctx::<Property>()?
                .deserialize::<Self, GValue>()
                .ctx::<Property>()?,
        );
        let mut element = Box::new(GValue::Null);

        if let Some(el) = val.get("element") {
            element = Box::new(el.deserialize::<Self, GValue>()?);
        }

        Ok(Property {
            key,
            value,
            element,
        })
    }
}

impl Serializer<Property> for V3 {
    fn serialize(val: &Property) -> Result<Value, Leaf> {
        Ok(json!({
            "@type": Tag::Property,
            "@value": {
              "key" : val.key.serialize::<Self>()?,
              "value" : (*val.value).serialize::<Self>()?,
            }
        }))
    }
}
