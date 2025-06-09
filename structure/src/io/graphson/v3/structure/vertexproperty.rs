use crate::io::graphson::prelude::*;
use indexmap::IndexMap;

impl Deserializer<VertexProperty> for V3 {
    fn deserialize(val: &Value) -> Result<VertexProperty, Error> {
        let map = get_value!(val, Value::Object)?;
        let id = map
            .get("id")
            .ok_or(Error::missing("id"))?
            .deserialize::<Self, GID>()?;
        let label = map
            .get("label")
            .ok_or(Error::missing("label"))?
            .deserialize::<Self, String>()?;
        let value = {
            let tmp = map
                .get("value")
                .ok_or(Error::missing("value"))?
                .deserialize::<Self, GValue>()?;
            Box::new(tmp)
        };
        Ok(VertexProperty {
            id,
            value,
            vertex: None,
            label,
            properties: None,
        })
    }
}

impl Serializer<VertexProperty> for V3 {
    fn serialize(val: &VertexProperty) -> Result<Value, Error> {
        let mut tmp = IndexMap::new();
        tmp.insert("id", val.id.serialize::<Self>()?);
        tmp.insert("value", (&*val.value).serialize::<Self>()?);

        if let Some(v) = &val.vertex {
            tmp.insert("vertex", v.serialize::<Self>()?);
        }

        tmp.insert("label", val.label.serialize::<Self>()?);

        if let Some(p) = &val.properties {
            let mut tmp_2 = IndexMap::new();
            for (k, v) in p.iter() {
                tmp_2.insert(k, v.serialize::<Self>()?);
            }
            tmp.insert("properties", json!(tmp_2));
        }

        Ok(json!({
            "@type": Tag::VertexProperty,
            "@value": tmp,
        }))
    }
}
