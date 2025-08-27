use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<VertexProperty, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<VertexProperty, Error> {
        let _debug = val.to_string();
        let mut property = VertexProperty {
            id: val["id"].deserialize::<Self, D, GID>()?,
            value: Box::new(val["value"].deserialize::<Self, D, GValue>()?),
            vertex: None,
            label: val
                .get("label")
                .map(|f| get_value!(f, Value::String).map(Clone::clone))
                .unwrap_or_else(|| {
                    Err(Error::Unexpected {
                        expectation: "VertexProperty label".into(),
                        actual: format!("{val:?}"),
                        location: location!(),
                    })
                })?,
            properties: None,
        };

        if let Some(vertex_id) = val.get("vertex") {
            let vertex = vertex_id.typed()?.value.deserialize::<Self, D, GID>()?;
            property.vertex = Some(vertex);
        }

        property.properties = val
            .get("properties")
            .map(|p| get_value!(p, Value::Object).unwrap())
            .map(|obj| {
                obj.into_iter()
                    .map(|(label, property_value)| {
                        (label, property_value.deserialize::<Self, D, GValue>())
                    })
                    .filter(|(_, v)| v.is_ok())
                    .map(|(k, v)| (k.clone(), v.unwrap()))
                    .collect::<Map<String, GValue>>()
            });

        Ok(property)
    }
}

impl<D: Dialect> GraphsonSerializer<VertexProperty, D> for GraphSON<V2> {
    fn serialize(val: &VertexProperty) -> Result<Value, Error> {
        let mut value = IndexMap::<&'static str, Value>::new();

        value.insert("id", val.id().serialize::<Self, D>()?);
        value.insert("value", (*val.value).serialize::<Self, D>()?);
        value.insert("label", val.label.serialize::<Self, D>()?);
        if let Some(id) = &val.vertex {
            value.insert("vertex", id.serialize::<Self, D>()?);
        }
        if let Some(properties) = &val.properties {
            let map = properties
                .iter()
                .map(|(k, v)| (k, v.serialize::<Self, D>()))
                .map(|(k, result)| match result {
                    Ok(v) => Ok((k.to_string(), v)),
                    Err(e) => Err(e),
                })
                .collect::<Result<IndexMap<String, Value>, Error>>()?;
            value.insert("properties", serde_json::to_value(&map)?);
        }

        Ok(json!(value))
    }
}
