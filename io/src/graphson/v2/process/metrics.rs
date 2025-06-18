use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Metrics, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let metric = get_value!(val, Value::Object)?.to_owned();
        let duration = metric.ensure("dur")?.deserialize::<Self, D, Double>()?;
        let id = metric.ensure("id")?.deserialize::<Self, D, String>()?;
        let name = metric.ensure("name")?.deserialize::<Self, D, String>()?;
        let counts = get_value!(metric.ensure("counts")?, Value::Object)?;
        let traversers = counts
            .ensure("traverserCount")?
            .deserialize::<Self, D, Long>()?;
        let count = counts
            .ensure("elementCount")?
            .deserialize::<Self, D, Long>()?;
        let annotations = get_value!(
            metric
                .get("annotations")
                .map(|v| v.to_owned())
                .unwrap_or(Value::Object(serde_json::Map::new())),
            Value::Object
        )?;
        let perc_duration = annotations
            .ensure("percentDur")?
            .deserialize::<Self, D, Double>()
            .unwrap_or(Double(0.0));
        let nested = get_value!(metric.ensure("metrics")?, Value::Array)?
            .iter()
            .map(|val| val.deserialize::<Self, D, Metrics>())
            .collect::<Result<List<_>, _>>()?;
        let metric = Metrics::new(id, name, duration, count, traversers, perc_duration, nested);

        Ok(metric)
    }
}

impl<D: Dialect> GraphsonSerializer<Metrics, D> for GraphSON<V2> {
    fn serialize(_val: &Metrics) -> Result<Value, Error> {
        todo!()
    }
}
