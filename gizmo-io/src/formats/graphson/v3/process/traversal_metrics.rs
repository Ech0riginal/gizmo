use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TraversalMetrics, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let metrics = get_value!(val, Value::Object)?;
        let duration = metrics
            .ensure("dur")?
            .deserialize::<Self, D, GValue>()
            .map(|gval| get_value!(gval, GValue::Double))??;
        let metrics = get_value!(metrics.ensure("metrics")?, Value::Array)?
            .iter()
            .map(|val| val.deserialize::<Self, D, GValue>())
            .map(|result| match result {
                Ok(gval) => get_value!(gval, GValue::Metrics),
                Err(e) => Err(e),
            })
            .collect::<Result<List<_>, Error>>()?;

        Ok(TraversalMetrics::new(duration, metrics))
    }
}

impl<D: Dialect> GraphsonSerializer<TraversalMetrics, D> for GraphSON<V3> {
    fn serialize(val: &TraversalMetrics) -> Result<Value, Error> {
        Ok(json!({
            "dur": val.duration.gvalue().serialize::<Self, D>()?,
            "metrics": val.metrics.iter().map(|item| item.gvalue().serialize::<Self, D>()).collect::<Result<Vec<_>, _>>()?,
        }))
    }
}
