use crate::graphson::prelude::*;

impl Deserializer<TraversalMetrics> for V2 {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let metrics = get_value!(val, Value::Object)?;

        let duration = metrics
            .ensure("dur")?
            .deserialize::<Self, Double>()?;
        let metrics = get_value!(
            metrics.ensure("metrics")?,
            Value::Array
        )?
        .iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<List<_>, Error>>()?;

        Ok(TraversalMetrics::new(duration, metrics))
    }
}

impl Serializer<TraversalMetrics> for V2 {
    fn serialize(val: &TraversalMetrics) -> Result<Value, Leaf> {
        todo!()
    }
}
