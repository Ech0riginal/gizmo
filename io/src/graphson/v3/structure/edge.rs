use crate::graphson::prelude::*;
use std::collections::HashMap;

impl<D: Dialect> GraphsonDeserializer<Edge, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Edge, Error> {
        let map = get_value!(val, Value::Object)?;
        let id = map.ensure("id")?.deserialize::<Self, D, GID>()?;
        let label = map.ensure("label")?.deserialize::<Self, D, String>()?;
        let in_v = map.ensure("inV")?.deserialize::<Self, D, GID>()?;
        let in_v_label = map.ensure("inVLabel")?.deserialize::<Self, D, String>()?;
        let out_v = map.ensure("outV")?.deserialize::<Self, D, GID>()?;
        let out_v_label = map.ensure("outVLabel")?.deserialize::<Self, D, String>()?;
        let properties = if let Some(properties_val) = map.get("properties") {
            let map = get_value!(properties_val, Value::Object)?;
            let mut indexed = Map::new();
            for (k, v) in map.iter() {
                indexed.insert(k.to_string(), Box::new(v.deserialize::<Self, D, GValue>()?));
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

impl<D: Dialect> GraphsonSerializer<Edge, D> for GraphSON<V3> {
    fn serialize(val: &Edge) -> Result<Value, Error> {
        let mut value = HashMap::new();
        value.insert("id", val.id.serialize::<Self, D>()?);
        value.insert("label", val.label.serialize::<Self, D>()?);
        // if serialize_labels {
        value.insert("inVLabel", val.in_v.label.serialize::<Self, D>()?);
        value.insert("outVLabel", val.out_v.label.serialize::<Self, D>()?);
        // }
        value.insert("inV", val.in_v.id.serialize::<Self, D>()?);
        value.insert("outV", val.out_v.id.serialize::<Self, D>()?);
        if !val.properties.is_empty() {
            let properties = val
                .properties
                .iter()
                .map(|(label, property)| (label.to_string(), (**property).serialize::<Self, D>()))
                .map(|(label, result)| match result {
                    Ok(value) => Ok((label, value)),
                    Err(e) => Err(e),
                })
                .collect::<Result<Map<_, _>, Error>>()?;
            value.insert("properties", json!(properties.0));
        }

        Ok(json!(value))
    }
}

// pub fn serialize_edge<S, D>(edge: &Edge, serialize_labels: bool) -> Result<Value, Error>
// where
//     S: Format,
//     S::Serial: serde::Serialize,
//     D: Dialect,
//     S: GraphsonSerializer<GID, D>,
//     S: GraphsonSerializer<GValue, D>,
//     S: GraphsonSerializer<String, D>,
// {
//     let mut value = HashMap::new();
//     value.insert("id", edge.id.serialize::<S, D>()?);
//     value.insert("label", edge.label.serialize::<S, D>()?);
//     if serialize_labels {
//         value.insert("inVLabel", edge.in_v.label.serialize::<S, D>()?);
//         value.insert("outVLabel", edge.out_v.label.serialize::<S, D>()?);
//     }
//     value.insert("inV", edge.in_v.id.serialize::<S, D>()?);
//     value.insert("outV", edge.out_v.id.serialize::<S, D>()?);
//     if !edge.properties.is_empty() {
//         let properties = edge
//             .properties
//             .iter()
//             .map(|(label, property)| (label, (**property).serialize::<S, D>()))
//             .map(|(label, result)| match result {
//                 Ok(value) => Ok((label, value)),
//                 Err(e) => Err(e),
//             })
//             .collect::<Result<Map<_, _>, Error>>()?;
//         let json = properties.serialize::<S, D>()?;
//
//         value.insert("properties", json);
//     }
//
//     Ok(json!(value))
// }
