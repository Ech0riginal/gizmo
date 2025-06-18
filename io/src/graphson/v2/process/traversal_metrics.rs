use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TraversalMetrics, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let metrics = get_value!(val, Value::Object)?;

        let duration = metrics.ensure("dur")?.deserialize::<Self, D, Double>()?;
        let metrics = get_value!(metrics.ensure("metrics")?, Value::Array)?
            .iter()
            .map(|val| val.deserialize::<Self, D, Metrics>())
            .collect::<Result<List<_>, Error>>()?;

        Ok(TraversalMetrics::new(duration, metrics))
    }
}

impl<D: Dialect> GraphsonSerializer<TraversalMetrics, D> for GraphSON<V2> {
    fn serialize(_val: &TraversalMetrics) -> Result<Value, Error> {
        todo!()
    }
}
