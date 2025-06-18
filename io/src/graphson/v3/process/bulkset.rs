use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<BulkSet, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<BulkSet, Error> {
        if val.to_string().contains("[null]") {
            // TODO Gremlin docs!
            return Ok(BulkSet::default());
        }

        let values = get_value!(val, Value::Array)?;

        if values.len() % 2 != 0 {
            Err(Error::Unexpected {
                expectation: "An array divisible by 2".to_string(),
                actual: format!("{val:?}"),
                location: location!(),
            })?;
        }

        let mut data = vec![];
        let mut counts = vec![];

        for json in values.clone().into_iter() {
            let gval = json.deserialize::<Self, D, GValue>()?;
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
                        map.get(&key).cloned()
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

        let occurrences = hashmap.len();
        Ok(BulkSet {
            map: hashmap,
            occurrences,
        })
    }
}

impl<D: Dialect> GraphsonSerializer<BulkSet, D> for GraphSON<V3> {
    fn serialize(val: &BulkSet) -> Result<Value, Error> {
        todo!()
    }
}
