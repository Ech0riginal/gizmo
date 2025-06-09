use crate::io::graphson::prelude::*;

impl Deserializer<Metrics> for V2 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let metric = get_value!(val, Value::Object)?.to_owned();
        let duration = metric
            .get("dur")
            .ok_or(Error::missing("dur"))?
            .deserialize::<Self, Double>()?;
        let id = metric
            .get("id")
            .ok_or(Error::missing("id"))?
            .deserialize::<Self, String>()?;
        let name = metric
            .get("name")
            .ok_or(Error::missing("name"))?
            .deserialize::<Self, String>()?;
        let counts = get_value!(
            metric.get("counts").ok_or(Error::missing("counts"))?,
            Value::Object
        )?;
        let traversers = counts
            .get("traverserCount")
            .ok_or(Error::missing("traverserCount"))?
            .deserialize::<Self, Long>()?;
        let count = counts
            .get("elementCount")
            .ok_or(Error::missing("elementCount"))?
            .deserialize::<Self, Long>()?;
        let annotations = get_value!(
            metric
                .get("annotations")
                .map(|v| v.to_owned())
                .unwrap_or(Value::Object(serde_json::Map::new())),
            Value::Object
        )?;
        let perc_duration = annotations
            .get("percentDur")
            .ok_or(Error::missing("percentDur"))?
            .deserialize::<Self, Double>()
            .unwrap_or(Double(0.0));
        let nested = get_value!(
            metric.get("metrics").ok_or(Error::missing("metrics"))?,
            Value::Array
        )?
        .into_iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<List<_>, Error>>()?;
        let metric = Metrics::new(id, name, duration, count, traversers, perc_duration, nested);

        Ok(metric)
    }
}

impl Serializer<Metrics> for V2 {
    fn serialize(val: &Metrics) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
