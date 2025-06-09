use crate::io::graphson::Tag;
use crate::io::graphson::prelude::*;

impl Deserializer<Property> for V3 {
    fn deserialize(val: &Value) -> Result<Property, Error> {
        let val = get_value!(val, Value::Object)?;
        let key = val
            .get("key")
            .ok_or(Error::missing("key"))?
            .deserialize::<Self, String>()?;
        let value = Box::new(
            val.get("value")
                .ok_or(Error::missing("value"))?
                .deserialize::<Self, GValue>()?,
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
    fn serialize(val: &Property) -> Result<Value, Error> {
        Ok(json!({
            "@type": Tag::Property,
            "@value": {
              "key" : val.key.serialize::<Self>()?,
              "value" : (&*val.value).serialize::<Self>()?,
            }
        }))
    }
}
