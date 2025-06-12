use crate::graphson::prelude::*;

use std::collections::HashMap;

impl Deserializer<Edge> for V2 {
    fn deserialize(val: &Value) -> Result<Edge, Error> {
        let edge_id = val["id"].deserialize::<Self, GID>()?;
        let label = val
            .get("label")
            .map(|f| get_value!(f, Value::String).map(Clone::clone))
            .unwrap_or_else(|| Ok(String::from("edge")))?;

        let in_v_id = val["inV"].deserialize::<Self, GID>()?;
        // This is intentional, there is no clear guidance on the discrepancies in 2.0.
        // let in_v_label = get_value!(&val["inVLabel"], Value::String)?.clone();
        let in_v_label = val
            .get("inVLabel")
            .map(|label| get_value!(label, Value::String).map(Clone::clone).unwrap())
            .unwrap_or("Unavailable".into());

        let out_v_id = val["outV"].deserialize::<Self, GID>()?;
        // If we don't account for it, we can't ser/de Property types.
        let out_v_label = val
            .get("outVLabel")
            .map(|label| get_value!(label, Value::String).map(Clone::clone).unwrap())
            .unwrap_or("Unavailable".into());
        Ok(Edge {
            id: edge_id,
            label,
            in_v: Vertex {
                id: in_v_id,
                label: in_v_label,
                properties: Default::default(),
            },
            out_v: Vertex {
                id: out_v_id,
                label: out_v_label,
                properties: Default::default(),
            },
            properties: Map::new(),
        })
    }
}

impl Serializer<Edge> for V2 {
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
            .collect::<Result<Vec<_>, Error>>()?
            .into_iter()
            .collect::<HashMap<&String, Value>>();
        value.insert("properties", serde_json::to_value(&properties)?);
    }

    Ok(json!({
        "@type": Tag::Edge,
        "@value": value
    }))
}
