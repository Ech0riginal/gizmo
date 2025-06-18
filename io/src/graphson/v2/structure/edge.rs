use crate::graphson::prelude::*;

use std::collections::HashMap;

impl<D: Dialect> GraphsonDeserializer<Edge, D> for GraphSON<V2>
where
    GID: SerializeExt,
{
    fn deserialize(val: &Value) -> Result<Edge, Error> {
        let edge_id = val["id"].deserialize::<Self, D, GID>()?;
        let label = val
            .get("label")
            .map(|f| get_value!(f, Value::String).map(Clone::clone))
            .unwrap_or_else(|| Ok(String::from("edge")))?;

        let in_v_id = val["inV"].deserialize::<Self, D, GID>()?;
        // This is intentional, there is no clear guidance on the discrepancies in 2.0.
        // let in_v_label = get_value!(&val["inVLabel"], Value::String)?.clone();
        let in_v_label = val
            .get("inVLabel")
            .map(|label| get_value!(label, Value::String).map(Clone::clone).unwrap())
            .unwrap_or("Unavailable".into());

        let out_v_id = val["outV"].deserialize::<Self, D, GID>()?;
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

impl<D: Dialect> GraphsonSerializer<Edge, D> for GraphSON<V2> {
    fn serialize(val: &Edge) -> Result<Value, Error> {
        serialize_edge::<Self, D>(val, true)
    }
}

pub fn serialize_edge<S, D>(edge: &Edge, serialize_labels: bool) -> Result<Value, Error>
where
    D: Dialect,
    S: Format,
    S::Serial: serde::Serialize,
    S: Serializer<GID, S::Serial, D>,
    S: Serializer<GValue, S::Serial, D>,
    S: Serializer<String, S::Serial, D>,
    S: Serializer<Map<Value, S::Serial>, S::Serial, D>,
{
    let mut value = HashMap::new();
    value.insert("id", edge.id.serialize::<S, D>()?);
    value.insert("label", edge.label.serialize::<S, D>()?);
    if serialize_labels {
        value.insert("inVLabel", edge.in_v.label.serialize::<S, D>()?);
        value.insert("outVLabel", edge.out_v.label.serialize::<S, D>()?);
    }
    value.insert("inV", edge.in_v.id.serialize::<S, D>()?);
    value.insert("outV", edge.out_v.id.serialize::<S, D>()?);
    if !edge.properties.is_empty() {
        let properties = edge
            .properties
            .iter()
            .map(|(label, property)| (json!(label), (**property).serialize::<S, D>()))
            .map(|(label, result)| match result {
                Ok(value) => Ok((label, value)),
                Err(e) => Err(e),
            })
            .collect::<Result<Map<Value, S::Serial>, Error>>()?;
        let props = properties.serialize::<S, D>()?;
        value.insert("properties", props);
    }

    Ok(json!(value))
}
