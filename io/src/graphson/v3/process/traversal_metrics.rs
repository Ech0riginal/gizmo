use crate::graphson::prelude::*;
use crate::{GValue, List};

impl Deserializer<TraversalMetrics> for V3 {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let mut metrics = val.deserialize::<Self, Map<GValue, GValue>>()?;

        let duration = metrics.remove_ok::<Double, _>("dur")?;
        let m = metrics
            .remove_ok::<List<GValue>, _>("metrics")?
            .drain(..)
            .map(|value| get_value!(value, GValue::Metrics))
            .filter_map(Result::ok)
            .collect::<List<_>>();

        Ok(TraversalMetrics::new(duration, m))
    }
}

impl Serializer<TraversalMetrics> for V3 {
    fn serialize(val: &TraversalMetrics) -> Result<Value, Error> {
        todo!()
    }
}
