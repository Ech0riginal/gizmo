use crate::io::graphson::prelude::*;

impl Deserializer<TraversalMetrics> for V2 {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let metrics = get_value!(val, Value::Object)?;

        let duration = metrics
            .get("dur")
            .ok_or(Error::Missing("dur"))?
            .deserialize::<Self, Double>()?;
        let metrics = get_value!(
            metrics.get("metrics").ok_or(Error::Missing("metrics"))?,
            Value::Array
        )?
        .into_iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<Vec<_>, Error>>()?;

        Ok(TraversalMetrics::new(duration, metrics).into())
    }
}

impl Serializer<TraversalMetrics> for V2 {
    fn serialize(val: &TraversalMetrics) -> Result<serde_json::Value, Error> {
        todo!()
    }
}

impl Deserializer<TraversalMetrics> for V3 {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let mut metrics = D::deserialize(&val)?.take::<Map>()?;

        let duration = remove_or_else(&mut metrics, "dur", TRAVERSAL_METRICS)?.take::<f64>()?;

        let m = remove_or_else(&mut metrics, "metrics", TRAVERSAL_METRICS)?
            .take::<List>()?
            .take()
            .drain(0..)
            .map(|e| e.take::<Metric>())
            .filter_map(Result::ok)
            .collect();

        Ok(TraversalMetrics::new(duration, m).into())
    }
}
