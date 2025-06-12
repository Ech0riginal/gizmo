use crate::graphson::prelude::*;

impl Deserializer<Property> for V2 {
    fn deserialize(val: &Value) -> Result<Property, Error> {
        let key = val
            .get("key")
            .map(|v| get_value!(v, Value::String).map(Clone::clone))
            .ok_or(Error::UnexpectedJson {
                msg: "Missing Property 'key' key".into(),
                value: val.clone(),
            })??;
        let value = val.get("value").ok_or(Error::UnexpectedJson {
            msg: "Missing Property 'value' key".into(),
            value: val.clone(),
        })?;
        let element = val.get("element").ok_or(Error::UnexpectedJson {
            msg: "Missing Property 'element' key".into(),
            value: val.clone(),
        })?;

        let value_obj = value.deserialize::<Self, GValue>()?;
        let element_obj = element.deserialize::<Self, GValue>()?;
        let property = Property {
            key,
            value: Box::new(value_obj),
            element: Box::new(element_obj),
        };

        Ok(property)
    }
}

impl Serializer<Property> for V2 {
    fn serialize(val: &Property) -> Result<Value, Leaf> {
        Ok(json!({
            "@type": Tag::Property,
            "@value": {
                "key": val.key,
                "value": (*val.value).serialize::<Self>()?,
                "element": match &*val.element {
                    GValue::Edge(edge) => super::edge::serialize_edge::<Self>(edge, false)?,
                    element => element.serialize::<Self>()?,
                }
            }
        }))
    }
}
