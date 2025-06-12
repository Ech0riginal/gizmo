use crate::graphson::prelude::*;
use crate::{GValue, List};

impl Deserializer<TraversalMetrics> for V3 {
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Leaf> {
        let mut metrics = val.deserialize::<Self, Map<GValue, GValue>>()?;

        let duration = metrics
            .remove_ok::<Double, _>("dur")
            .ctx::<TraversalMetrics>()?;
        let m = metrics
            .remove_ok::<List<GValue>, _>("metrics")
            .ctx::<TraversalMetrics>()?
            .drain(..)
            .map(|value| get_value!(value, GValue::Metric))
            .filter_map(Result::ok)
            .collect::<List<_>>();

        Ok(TraversalMetrics::new(duration, m))
    }
}
