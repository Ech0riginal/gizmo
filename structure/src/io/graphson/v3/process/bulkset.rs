use crate::io::graphson::prelude::*;

impl Deserializer<BulkSet> for V3 {
    fn deserialize(val: &Value) -> Result<BulkSet, Error> {
        if val.to_string().contains("[null]") {
            // TODO Gremlin docs!
            return Ok(BulkSet::default());
        }

        let values = get_value!(val, Value::Array)?;

        if values.len() % 2 != 0 {
            return Err(Error::UnexpectedJson {
                msg: "Malformed BulkSet".to_string(),
                value: val.to_owned(),
            });
        }

        let mut data = vec![];
        let mut counts = vec![];

        for json in values.clone().into_iter() {
            let gval = json.deserialize::<Self, GValue>()?;
            if data.len() > counts.len() {
                counts.push(gval);
            } else {
                data.push(gval);
            }
        }

        let hashmap = data
            .clone()
            .into_iter()
            .flat_map(|val| {
                let key_opt = match val {
                    GValue::Map(ref map) => {
                        let key = GValue::from("id");
                        map.get(&key).map(|v| v.clone())
                    }
                    _ => None,
                };
                if key_opt.is_some() {
                    Some((key_opt.unwrap(), val))
                } else {
                    None
                }
            })
            .collect::<Map<GValue, GValue>>();

        // TODO populate BulkSet occurrences

        Ok(BulkSet {
            map: hashmap,
            occurrences: 0,
        })
    }
}
