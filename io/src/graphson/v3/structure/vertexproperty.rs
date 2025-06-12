use crate::graphson::prelude::*;
use indexmap::IndexMap;
use snafu::location;
impl Deserializer<VertexProperty> for V3 {
    fn deserialize(val: &Value) -> Result<VertexProperty, Error> {
        let map = get_value!(val, Value::Object).ctx::<VertexProperty>()?;
        let id = map
            .ensure("id")
            .ctx::<VertexProperty>()?
            .deserialize::<Self, GID>()
            .ctx::<VertexProperty>()?;
        let label = map
            .ensure("label")
            .ctx::<VertexProperty>()?
            .deserialize::<Self, String>()
            .ctx::<VertexProperty>()?;
        let value = {
            let tmp = map
                .ensure("value")
                .ctx::<VertexProperty>()?
                .deserialize::<Self, GValue>()
                .ctx::<VertexProperty>()?;
            Box::new(tmp)
        };
        let mut properties = None;

        if let Some(props) = map.get("properties") {
            let mut tmp = Map::new();
            let prop_map = get_value!(props, Value::Object).ctx::<VertexProperty>()?;
            for (key, value) in prop_map.into_iter() {
                let value = value
                    .deserialize::<Self, GValue>()
                    .ctx::<VertexProperty>()?;
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
        tmp.insert("value", (*val.value).serialize::<Self>()?);

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
