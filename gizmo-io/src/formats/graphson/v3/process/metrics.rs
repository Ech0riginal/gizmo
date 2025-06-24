use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Metrics, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Metrics, Error> {
        let metric = get_value!(val.deserialize::<Self, D, GValue>()?, GValue::Map)?.to_owned();

        let duration = get_value!(metric.ensure("dur")?, GValue::Double)?.clone();

        let counts = get_value!(metric.ensure("counts")?, GValue::Map)?;
        let name = get_value!(metric.ensure("name")?, GValue::String)?;
        let annotations = get_value!(metric.ensure("annotations")?, GValue::Map)
            .map(|map| {
                map.iter().filter_map(|(k, v)| {
                    get_value!(k, GValue::String)
                        .map(|k| (k.clone(), v.clone()))
                        .ok()
                })
            })?
            .collect::<Map<String, GValue>>();
        let id = get_value!(metric.ensure("id")?, GValue::String)?;

        let traversers = get_value!(counts.ensure("traverserCount")?, GValue::Long)?.to_owned();
        let count = get_value!(counts.ensure("elementCount")?, GValue::Long)?.to_owned();

        let nested = if let Ok(nested) = metric.ensure("metrics") {
            get_value!(nested, GValue::List)?
                .iter()
                .map(|gval| get_value!(gval, GValue::Metrics))
                .map(|result| result.map(|metric| metric.to_owned()))
                .collect::<Result<List<_>, _>>()?
        } else {
            list![]
        };

        let metric = Metrics::new(id, name, duration, count, traversers, annotations, nested);

        Ok(metric)
    }
}

impl<D: Dialect> GraphsonSerializer<Metrics, D> for GraphSON<V3> {
    fn serialize(val: &Metrics) -> Result<Value, Error> {
        let mut map = {
            let tmp = json!([
                "dur", GValue::Double(val.duration).serialize::<Self, D>()?,
                "counts", {
                    "@type": D::tag::<Map<(), ()>>(),
                    "@value": [
                        "traverserCount",
                        GValue::Long(val.traversers).serialize::<Self, D>()?,
                        "elementCount",
                        GValue::Long(val.elements).serialize::<Self, D>()?,
                    ],
                },
                "name", val.name,
                "annotations", {
                    "@type": D::tag::<Map<(), ()>>(),
                    "@value": val.annotations.serialize::<Self, D>()?,
                },
                "id", val.id,
            ]);
            get_value!(tmp, Value::Array)?
        };

        if !val.nested.is_empty() {
            let list = GValue::from(val.nested.iter().map(GValue::from).collect::<List<_>>());
            let nested = list.serialize::<Self, D>()?;

            map.push("metrics".into());
            map.push(nested);
        }

        Ok(json!({
            "@type": D::tag::<Map<(), ()>>(),
            "@value": map,
        }))
    }
}
