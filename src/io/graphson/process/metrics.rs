use crate::io::graphson::prelude::*;

impl Deserializer<Metrics> for V2 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let metric = get_value!(val, Value::Object)?.to_owned();
        // metric.remove().ok_or(Error::Missing())
        let duration = metric
            .get("dur")
            .ok_or(Error::Missing("dur"))?
            .deserialize::<Self, Double>()?;
        let id = metric
            .get("id")
            .ok_or(Error::Missing("id"))?
            .deserialize::<Self, String>()?;
        let name = metric
            .get("name")
            .ok_or(Error::Missing("name"))?
            .deserialize::<Self, String>()?;
        let counts = get_value!(
            metric.get("counts").ok_or(Error::Missing("counts"))?,
            Value::Object
        )?;
        let traversers = counts
            .get("traverserCount")
            .ok_or(Error::Missing("traverserCount"))?
            .deserialize::<Self, Long>()?;
        let count = counts
            .get("elementCount")
            .ok_or(Error::Missing("elementCount"))?
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
            .ok_or(Error::Missing("percentDur"))?
            .deserialize::<Self, Double>()
            .unwrap_or(Double(0.0));
        let nested = get_value!(
            metric.get("metrics").ok_or(Error::Missing("metrics"))?,
            Value::Array
        )?
        .into_iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<Vec<_>, Error>>()?;
        let metric = Metrics::new(id, name, duration, count, traversers, perc_duration, nested);

        Ok(metric)
    }
}

impl Serializer<Metrics> for V2 {
    fn serialize(val: &Metrics) -> Result<serde_json::Value, Error> {
        todo!()
    }
}

impl Deserializer<Metrics> for V3 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        fn get<'a>(map: &'a serde_json::Map<String, Value>, key: &'static str) -> Result<&'a Value, Error> {
            map.get(key).ok_or(Error::Missing(key))
        }
        
        let obj = get_value!(val, Value::Object)?;
        let counts = get_value!(get(obj, "counts")?, Value::Object)?;
        let annotations = get_value!(get(obj, "annotations")?, Value::Object)?;
        let metrics = get_value!(get(obj, "metrics")?, Value::Array)?;
        
        let id = get(obj, "id")?.deserialize::<Self, String>()?;
        let name = get(obj,"name")?.deserialize::<Self, String>()?;
        let duration = get(obj, "dur")?.deserialize::<Self, Double>()?;
        let traversers = get(counts, "traverserCount")?.deserialize::<Self, Long>()?;
        let count = get(counts, "elementCount")?.deserialize::<Self, Long>()?;
        let perc_duration = get(annotations, "percentDur")?.deserialize::<Self, Double>()?;
        let nested = metrics
            .into_iter()
            .map(|v| v.deserialize::<Self, Metrics>())
            .collect::<Result<Vec<_>, Error>>()?;
        let metrics = Metrics::new(
            id,
            name,
            duration,
            count,
            traversers,
            perc_duration,
            nested,
        );
        
        Ok(metrics.into())
    }
}
