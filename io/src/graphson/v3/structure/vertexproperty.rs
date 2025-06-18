use crate::graphson::prelude::*;
use indexmap::IndexMap;
use snafu::location;
impl<D: Dialect> GraphsonDeserializer<VertexProperty, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<VertexProperty, Error> {
        let map = get_value!(val, Value::Object)?;
        let id = map.ensure("id")?.deserialize::<Self, D, GID>()?;
        let label = map.ensure("label")?.deserialize::<Self, D, String>()?;
        let value = map
            .ensure("value")?
            .deserialize::<Self, D, GValue>()
            .map(Box::new)?;
        let mut properties = None;

        if let Some(props) = map.get("properties") {
            let mut tmp = Map::new();
            let prop_map = get_value!(props, Value::Object)?;
            for (key, value) in prop_map.into_iter() {
                let value = value.deserialize::<Self, D, GValue>()?;
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

impl<D: Dialect> GraphsonSerializer<VertexProperty, D> for GraphSON<V3> {
    fn serialize(val: &VertexProperty) -> Result<Value, Error> {
        let mut tmp = IndexMap::new();
        tmp.insert("id", val.id.serialize::<Self, D>()?);
        tmp.insert("value", (*val.value).serialize::<Self, D>()?);

        if let Some(v) = &val.vertex {
            tmp.insert("vertex", v.serialize::<Self, D>()?);
        }

        tmp.insert("label", val.label.serialize::<Self, D>()?);

        if let Some(p) = &val.properties {
            let mut tmp_2 = IndexMap::new();
            for (k, v) in p.iter() {
                tmp_2.insert(k, v.serialize::<Self, D>()?);
            }
            tmp.insert("properties", json!(tmp_2));
        }

        Ok(json!(tmp))
    }
}
