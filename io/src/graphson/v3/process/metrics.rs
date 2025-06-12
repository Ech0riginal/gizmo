use crate::graphson::prelude::*;
use snafu::IntoError;

impl Deserializer<Metrics> for V3 {
    fn deserialize(val: &Value) -> Result<Metrics, Leaf> {
        let metrics = get_value!(val, Value::Object).ctx::<Metrics>()?;
        let mut metric = val
            .deserialize::<Self, Map<GValue, GValue>>()
            .ctx::<Metrics>()?;

        // Honestly this is a pretty unacceptable amount of boilerplate
        macro_rules! gotta_be_a_better_way {
            ($val:ident, $key:expr, $ty:ty) => {
                $val.ensure($key)
                    .ctx::<Metrics>()?
                    .deserialize::<Self, $ty>()
                    .ctx::<Metrics>()?;
            };
        }

        let duration = gotta_be_a_better_way!(metrics, "dur", Double);
        let id = gotta_be_a_better_way!(metrics, "id", String);
        let name = gotta_be_a_better_way!(metrics, "name", String);

        let mut counts = metric
            .remove_ok::<Map<GValue, GValue>, _>("counts")
            .ctx::<Metrics>()?;
        let traversers = counts
            .remove_ok::<Long, _>("traverserCount")
            .ctx::<Metrics>()?;
        let count = counts
            .remove_ok::<Long, _>("elementCount")
            .ctx::<Metrics>()?;

        let mut annotations = counts
            .remove_ok::<Map<GValue, GValue>, _>("annotations")
            .unwrap_or_else(|_| Map::new());
        let perc_duration = annotations
            .remove_ok::<Double, _>("percentDur")
            .unwrap_or(Double(0.0));
        let nested = metric
            .remove_ok::<List<GValue>, _>(Tag::Metrics)
            .unwrap_or_else(|_| list![])
            .into_iter()
            .map(|v| get_value!(v, GValue::Metric))
            .collect::<Result<List<Metrics>, Leaf>>()
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
