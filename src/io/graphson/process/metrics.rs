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
        let mut metric = D::deserialize(&val)?.take::<Map>()?;

        let duration = remove_or_else(&mut metric, "dur", METRICS)?.take::<f64>()?;
        let id = remove_or_else(&mut metric, "id", METRICS)?.take::<String>()?;
        let name = remove_or_else(&mut metric, "name", METRICS)?.take::<String>()?;

        let mut counts = remove_or_else(&mut metric, "counts", METRICS)?.take::<Map>()?;
        let traversers = remove_or_else(&mut counts, "traverserCount", METRICS)?.take::<i64>()?;
        let count = remove_or_else(&mut counts, "elementCount", METRICS)?.take::<i64>()?;

        let mut annotations = remove(&mut metric, "annotations", METRICS)
            .map(|e| e.take::<Map>())
            .unwrap_or_else(|| Ok(Map::empty()))?;

        let perc_duration = remove(&mut annotations, "percentDur", METRICS)
            .map(|e| e.take::<f64>())
            .unwrap_or_else(|| Ok(0.0))?;

        let nested: GremlinResult<Vec<Metric>> = remove(&mut metric, "metrics", METRICS)
            .map(|e| e.take::<List>())
            .unwrap_or_else(|| Ok(List::new(vec![])))?
            .take()
            .into_iter()
            .map(|e| e.take::<Metric>())
            .collect();
        Ok(Metrics::new(
            id,
            name,
            duration,
            count,
            traversers,
            perc_duration,
            nested?,
        )
        .into())
    }
}
