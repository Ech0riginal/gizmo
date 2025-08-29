use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TraversalMetrics, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<GValue, D>,
{
    fn deserialize(val: &Value) -> Result<TraversalMetrics, Error> {
        let typed = val.typed()?;
        let mapped = match typed.tag {
            <Map<(), ()> as AST<D>>::tag => {
                typed.value.deserialize::<Self, D, Map<String, GValue>>()
            }
            _ => Err(Error::unexpected(val, "something else")),
        }?;

        let duration = mapped
            .ensure("dur")
            .map(|gval| get_value!(gval, GValue::Double))??;
        let metrics = mapped
            .ensure("metrics")
            .map(|gval| get_value!(gval, GValue::List))??
            .to_owned()
            .into_iter()
            .map(|gval| get_value!(gval, GValue::Metrics))
            .collect::<Result<List<_>, Error>>()?;
        Ok(TraversalMetrics::new(*duration, metrics))
    }
}

impl<D: Dialect> GraphsonSerializer<TraversalMetrics, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(val: &TraversalMetrics) -> Result<Value, Error> {
        let mut tmp = Map::new();
        tmp.insert(GValue::String("dur".into()), val.duration.gvalue());
        tmp.insert(
            GValue::String("metrics".into()),
            val.metrics
                .iter()
                .map(|item| item.gvalue())
                .collect::<List<_>>()
                .gvalue(),
        );

        tmp.gvalue().serialize::<Self, D>()
    }
}
