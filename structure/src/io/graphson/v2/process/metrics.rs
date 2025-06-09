use std::fmt;
use crate::io::graphson::prelude::*;

impl Deserializer<Metrics> for V2 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let metric = get_value!(val, Value::Object)?.to_owned();
        // metric.remove().ok_or(Error::Missing())
        let duration = metric
            .get("dur")
            .ok_or(Error::Missing("dur"))?
            .deserialize::<Self, Double>()?;
        let id = metric
            .get("id")
            .ok_or(Error::Missing("id"))?
            .deserialize::<Self, String>()?;
        let name = metric
            .get("name")
            .ok_or(Error::Missing("name"))?
            .deserialize::<Self, String>()?;
        let counts = get_value!(
            metric.get("counts").ok_or(Error::Missing("counts"))?,
            Value::Object
        )?;
        let traversers = counts
            .get("traverserCount")
            .ok_or(Error::Missing("traverserCount"))?
            .deserialize::<Self, Long>()?;
        let count = counts
            .get("elementCount")
            .ok_or(Error::Missing("elementCount"))?
            .deserialize::<Self, Long>()?;
        let annotations = get_value!(
            metric
                .get("annotations")
                .map(|v| v.to_owned())
                .unwrap_or(Value::Object(serde_json::Map::new())),
            Value::Object
        )?;
        let perc_duration = annotations
            .get("percentDur")
            .ok_or(Error::Missing("percentDur"))?
            .deserialize::<Self, Double>()
            .unwrap_or(Double(0.0));
        let nested = get_value!(
            metric.get("metrics").ok_or(Error::Missing("metrics"))?,
            Value::Array
        )?
        .into_iter()
        .map(|val| val.deserialize::<Self, Metrics>())
        .collect::<Result<Vec<_>, Error>>()?;
        let metric = Metrics::new(id, name, duration, count, traversers, perc_duration, nested);

        Ok(metric)
    }
}

impl Serializer<Metrics> for V2 {
    fn serialize(val: &Metrics) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
impl Deserializer<Metrics> for V3 {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let gvalue = val.deserialize::<Self, GValue>()?;
        let mut metric = get_value!(gvalue, GValue::Map)?;

        let duration = gank!(metric, "dur", Double)?;
        let id = gank!(metric, "id", String)?;
        let name = gank!(metric, "name", String)?;

        let mut counts = gank!(metric, "counts", Map)?;
        let traversers = gank!(counts, "traverserCount", Long)?;
        let count = gank!(counts, "elementCount", Long)?;

        let mut annotations = gank!(counts, "annotations", Map)
            .unwrap_or_else(|_| Map2::new());
        let perc_duration = gank!(annotations, "percentDur", Double)
            .unwrap_or_else(|_| Double(0.0));
        let nested = gank!(metric, METRICS, List)
                .unwrap_or_else(|_| List::new(vec![]))
                .into_iter()
                .map(|v| get_value!(v, GValue::Metric))
                .collect::<Result<Vec<Metrics>, Error>>()
                .unwrap_or_else(|e| {
                    tracing::warn!("Deserializing nested metrics signaled an error.");
                    tracing::warn!("{:?}", e);
                    vec![]
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
