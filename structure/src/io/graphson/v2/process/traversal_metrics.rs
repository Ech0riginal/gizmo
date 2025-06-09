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
        let mut metrics = val.deserialize::<Self, Map2>()?;

        let duration = gank!(metrics, "dur", Double)?;
        let m = gank!(metrics, "metrics", List)?.0
            .drain(..)
            .map(|value| get_value!(value, GValue::Metric))
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(TraversalMetrics::new(duration, m).into())
    }
}
