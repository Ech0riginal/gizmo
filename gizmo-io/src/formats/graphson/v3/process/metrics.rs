use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Metrics, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let metric = get_value!(val, Value::Object)?.to_owned();

        let duration = get_value!(
            metric.ensure("dur")?.deserialize::<Self, D, GValue>()?,
            GValue::Double
        )?;
        let id = metric.ensure("id")?.deserialize::<Self, D, String>()?;
        let name = metric.ensure("name")?.deserialize::<Self, D, String>()?;
        let counts = get_value!(metric.ensure("counts")?, Value::Object)?;
        let traversers = get_value!(
            counts
                .ensure("traverserCount")?
                .deserialize::<Self, D, GValue>()?,
            GValue::Long
        )?;
        let count = get_value!(
            counts
                .ensure("elementCount")?
                .deserialize::<Self, D, GValue>()?,
            GValue::Long
        )?;
        let annotations = get_value!(
            metric
                .get("annotations")
                .map(|v| v.to_owned())
                .unwrap_or(Value::Object(serde_json::Map::new())),
            Value::Object
        )?;
        let perc_duration = match annotations
            .ensure("percentDur")?
            .deserialize::<Self, D, GValue>()
        {
            Ok(gval) => get_value!(gval, GValue::Double),
            Err(e) => Err(e),
        }
        .unwrap_or(Double(0.0));
        let nested = if let Ok(metrics) = metric.ensure("metrics") {
            let gval = metrics.deserialize::<Self, D, GValue>()?;
            get_value!(gval, GValue::List)?
                .into_iter()
                .map(|gval| get_value!(gval, GValue::Metrics))
                .collect::<Result<List<_>, Error>>()?
        } else {
            list![]
        };

        let metric = Metrics::new(id, name, duration, count, traversers, perc_duration, nested);

        Ok(metric)
    }
}

impl<D: Dialect> GraphsonSerializer<Metrics, D> for GraphSON<V3> {
    fn serialize(val: &Metrics) -> Result<Value, Error> {
        let mut json = {
            let tmp = json!({
                "dur": GValue::Double(val.duration).serialize::<Self, D>()?,
                "counts": {
                    "traverserCount": GValue::Long(val.traversers).serialize::<Self, D>()?,
                    "elementCount": GValue::Long(val.elements).serialize::<Self, D>()?,
                },
                "name": val.name,
                "annotations": {
                    "percentDur": GValue::Double(val.perc_duration).serialize::<Self, D>()?,
                },
                "id": val.id,
            });
            get_value!(tmp, Value::Object)?
        };

        if !val.nested.is_empty() {
            let nested = val
                .nested
                .iter()
                .map(GValue::from)
                .collect::<List<_>>()
                .serialize::<Self, D>()?;
            json.insert("metrics".into(), nested);
        }

        Ok(Value::Object(json))
    }
}
