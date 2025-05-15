use crate::io::graphson::prelude::*;

impl Deserializer<BulkSet> for V3 {
    fn deserialize(val: &Value) -> Result<BulkSet, Error> {
        if val.to_string().contains("[null]") {
            // TODO Gremlin docs!
            return Ok(GValue::List(List::new(vec![])));
        }

        let val = get_value!(val, Value::Array)?;

        if val.len() % 2 != 0 {
            return Err(GremlinError::Cast(
                "Cannot construct BulkSet from value.".to_string(),
            ));
        }

        let mut data = vec![];
        let mut counts = vec![];

        for gval in val.clone().into_iter() {
            if data.len() > counts.len() {
                counts.push(D::deserialize(&gval)?);
            } else {
                data.push(D::deserialize(&gval)?);
            }
        }

        let hashmap = data
            .clone()
            .into_iter()
            .flat_map(|val| {
                let key_opt = match val {
                    GValue::Map(ref map) => match map.get("id") {
                        Some(GValue::Long(i)) => Some(GKey::String(i.to_string())),
                        Some(GValue::Integer(i)) => Some(GKey::String(i.to_string())),
                        Some(GValue::String(s)) => Some(GKey::String(s.to_string())),
                        _ => None,
                    },
                    _ => None,
                };
                // TODO very bad practice but it works so .. :D
                if key_opt.is_some() {
                    Some((key_opt.unwrap(), val))
                } else {
                    None
                }
            })
            .collect::<HashMap<GKey, GValue>>();

        // Ok(GValue::BulkSet(BulkSet::from(hashmap)))
        todo!()
    }
}
