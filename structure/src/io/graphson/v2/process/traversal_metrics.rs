use crate::io::graphson::prelude::*;
use indexmap::IndexSet;

impl Deserializer<TraversalMetrics> for V2 {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let metrics = get_value!(val, Value::Object)?;

        let duration = metrics
            .get("dur")
            .ok_or(Error::missing("dur"))?
            .deserialize::<Self, Double>()?;
        let metrics = get_value!(
            metrics.get("metrics").ok_or(Error::missing("metrics"))?,
            Value::Array
        )?
        .into_iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<List<_>, Error>>()?;

        Ok(TraversalMetrics::new(duration, metrics).into())
    }
}

impl Serializer<TraversalMetrics> for V2 {
    fn serialize(val: &TraversalMetrics) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
