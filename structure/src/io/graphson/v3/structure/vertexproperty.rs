use crate::io::error::Missing;
use crate::io::graphson::prelude::*;
use indexmap::IndexMap;

impl Deserializer<VertexProperty> for V3 {
    fn deserialize(val: &Value) -> Result<VertexProperty, Error> {
        let map = get_value!(val, Value::Object)?;
        let id = map
            .get("id")
            .ok_or("id".missing())?
            .deserialize::<Self, GID>()?;
        let label = map
            .get("label")
            .ok_or("label".missing())?
            .deserialize::<Self, String>()?;
        let value = {
            let tmp = map
                .get("value")
                .ok_or("value".missing())?
                .deserialize::<Self, GValue>()?;
            Box::new(tmp)
        };
        let mut properties = None;

        if let Some(props) = map.get("properties") {
            let mut tmp = Map::new();
            let prop_map = get_value!(props, Value::Object)?;
            for (key, value) in prop_map.into_iter() {
                let value = value.deserialize::<Self, GValue>()?;
                tmp.insert(key.to_string(), value);
            }
            properties = Some(tmp);
        }

        Ok(VertexProperty {
            id,
            value,
            vertex: None,
            label,
            properties,
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
