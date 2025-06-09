use crate::io::graphson::prelude::*;
use indexmap::{IndexSet, indexset};
use std::fmt;

impl Deserializer<Metrics> for V3 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let mut metric = val.deserialize::<Self, Map<GValue, GValue>>()?;

        let duration = metric.remove_ok::<Double, _>("dur")?;
        let id = metric.remove_ok::<String, _>("id")?;
        let name = metric.remove_ok::<String, _>("name")?;

        let mut counts = metric.remove_ok::<Map<GValue, GValue>, _>("counts")?;
        let traversers = counts.remove_ok::<Long, _>("traverserCount")?;
        let count = counts.remove_ok::<Long, _>("elementCount")?;

        let mut annotations = counts
            .remove_ok::<Map<GValue, GValue>, _>("annotations")
            .unwrap_or_else(|_| Map::new());
        let perc_duration = annotations
            .remove_ok::<Double, _>("percentDur")
            .unwrap_or_else(|_| Double(0.0));
        let nested = metric
            .remove_ok::<List<GValue>, _>(Tag::Metrics)
            .unwrap_or_else(|_| list![])
            .into_iter()
            .map(|v| get_value!(v, GValue::Metric))
            .collect::<Result<List<Metrics>, Error>>()
            .unwrap_or_else(|e| {
                tracing::warn!("Deserializing nested metrics signaled an error.");
                tracing::warn!("{:?}", e);
                list![]
            });

        Ok(Metrics {
            id,
            duration,
            name,
            count,
            traversers,
            perc_duration,
            nested,
        })
    }
}
