use crate::graphson::prelude::*;

impl Deserializer<Metrics> for V3 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let ty = val.typed()?;
        let mut metrics = val
            .typed()?
            .value
            .deserialize::<Self, Map<GValue, GValue>>()?;

        // Honestly this is a pretty unacceptable amount of boilerplate
        macro_rules! gotta_be_a_better_way {
            ($val:ident, $key:expr, $ty:ty) => {
                $val.ensure($key)?.deserialize::<Self, $ty>()?
            };
        }

        // todo!();

        let duration = metrics.remove_ok::<Double, _>("dur")?;
        let id = metrics.remove_ok::<String, _>("id")?;
        let name = metrics.remove_ok::<String, _>("name")?;
        let mut counts = metrics.remove_ok::<Map<GValue, GValue>, _>("counts")?;
        let traversers = counts.remove_ok::<Long, _>("traverserCount")?;
        let count = counts.remove_ok::<Long, _>("elementCount")?;
        let mut annotations_raw = metrics
            .remove_ok::<GValue, _>("annotations")
            .unwrap_or_else(|_| GValue::Null);
        let mut annotations = get_value!(annotations_raw, GValue::Map)?;
        let perc_duration = annotations
            .remove_ok::<Double, _>("percentDur")
            .unwrap_or(Double(0.0));
        let nested = match metrics.remove("metrics") {
            None => list![],
            Some(gvalue) => get_value!(gvalue, GValue::List)?
                .into_iter()
                .map(|metric| get_value!(metric, GValue::Metrics))
                .collect::<Result<List<_>, _>>()?,
        };

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

impl Serializer<Metrics> for V3 {
    fn serialize(val: &Metrics) -> Result<Value, Error> {
        todo!()
    }
}
