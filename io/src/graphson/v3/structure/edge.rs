use crate::graphson::prelude::*;
use std::collections::HashMap;

impl Deserializer<Edge> for V3 {
    fn deserialize(val: &Value) -> Result<Edge, Leaf> {
        let map = get_value!(val, Value::Object).ctx::<Edge>()?;
        let id = map.ensure("id")?.deserialize::<Self, GID>().ctx::<Edge>()?;
        let label = map
            .ensure("label")?
            .deserialize::<Self, String>()
            .ctx::<Edge>()?;
        let in_v = map
            .ensure("inV")?
            .deserialize::<Self, GID>()
            .ctx::<Edge>()?;
        let in_v_label = map
            .ensure("inVLabel")?
            .deserialize::<Self, String>()
            .ctx::<Edge>()?;
        let out_v = map
            .ensure("outV")?
            .deserialize::<Self, GID>()
            .ctx::<Edge>()?;
        let out_v_label = map
            .ensure("outVLabel")?
            .deserialize::<Self, String>()
            .ctx::<Edge>()?;
        let properties = if let Some(properties_val) = map.get("properties") {
            let map = get_value!(properties_val, Value::Object).ctx::<Edge>()?;
            let mut indexed = Map::new();
            for (k, v) in map.iter() {
                indexed.insert(
                    k.to_string(),
                    Box::new(v.deserialize::<Self, GValue>().ctx::<Edge>()?),
                );
            }
            indexed
        } else {
            Default::default()
        };

        Ok(Edge {
            id,
            label,
            in_v: Vertex {
                id: in_v,
                label: in_v_label,
                properties: Default::default(),
            },
            out_v: Vertex {
                id: out_v,
                label: out_v_label,
                properties: Default::default(),
            },
            properties,
        })
    }
}

impl Serializer<Edge> for V3 {
    fn serialize(val: &Edge) -> Result<Value, Leaf> {
        serialize_edge::<Self>(val, true)
    }
}

pub fn serialize_edge<S>(edge: &Edge, serialize_labels: bool) -> Result<Value, Leaf>
where
    S: Serializer<GID>,
    S: Serializer<GValue>,
    S: Serializer<String>,
{
    let mut value = HashMap::new();
    value.insert("id", edge.id.serialize::<S>()?);
    value.insert("label", edge.label.serialize::<S>()?);
    if serialize_labels {
        value.insert("inVLabel", edge.in_v.label().serialize::<S>()?);
        value.insert("outVLabel", edge.out_v.label().serialize::<S>()?);
    }
    value.insert("inV", edge.in_v.id().serialize::<S>()?);
    value.insert("outV", edge.out_v.id().serialize::<S>()?);
    if !edge.properties.is_empty() {
        let properties = edge
            .properties
            .iter()
            .map(|(label, property)| (label, (**property).serialize::<S>()))
            .map(|(label, result)| match result {
                Ok(value) => Ok((label, value)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<_>, _>>()
            .ctx::<Edge>()?
            .into_iter()
            .collect::<HashMap<&String, Value>>();
        // let debug = format!("{:?}", &properties);
        value.insert(
            "properties",
            serde_json::to_value(&properties).ctx::<Edge>()?,
        );
    }

    Ok(json!({
        "@type": Tag::Edge,
        "@value": value
    }))
}
