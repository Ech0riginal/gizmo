use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Property, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Property, Error> {
        let key = val
            .ensure("key")
            .map(|v| get_value!(v, Value::String).map(Clone::clone))??;
        let value = val.ensure("value")?;
        let element = val.ensure("element")?;
        let value_obj = value.deserialize::<Self, D, GValue>()?;
        let element_obj = element.deserialize::<Self, D, GValue>()?;
        let property = Property {
            key,
            value: Box::new(value_obj),
            element: Box::new(element_obj),
        };

        Ok(property)
    }
}

impl<D: Dialect> GraphsonSerializer<Property, D> for GraphSON<V2> {
    fn serialize(val: &Property) -> Result<Value, Error> {
        Ok(json!({
            "key": val.key,
            "value": (*val.value).serialize::<Self, D>()?,
            "element": match &*val.element {
                GValue::Edge(edge) => super::edge::serialize_edge::<Self, D>(edge, false)
                    .map(|value| json!({
                        "@type": D::tag::<Edge>(),
                        "@value": value,
                    }))?,
                element => element.serialize::<Self, D>()?,
            }
        }))
    }
}
