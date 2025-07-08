use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TextP, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<GValue, D>,
{
    fn deserialize(val: &Value) -> Result<TextP, Error> {
        let map = get_value!(val, Value::Object)?;
        let value = map
            .ensure("value")?
            .deserialize::<Self, D, GValue>()?
            .boxed();
        let predicate = map.ensure("predicate")?.deserialize::<Self, D, Text>()?;
        Ok(TextP { predicate, value })
    }
}

impl<D: Dialect> GraphsonDeserializer<Text, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Text, Error> {
        let repr = val.deserialize::<Self, D, String>()?;
        match repr.as_str() {
            Text::CONTAINING => Ok(Text::Containing),
            Text::ENDING_WITH => Ok(Text::EndingWith),
            Text::STARTING_WITH => Ok(Text::StartingWith),
            Text::NOT_CONTAINING => Ok(Text::NotContaining),
            Text::NOT_ENDING_WITH => Ok(Text::NotEndingWith),
            Text::NOT_STARTING_WITH => Ok(Text::NotStartingWith),
            _ => Err(Error::unexpected(val, "a valid Text predicate")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<TextP, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(val: &TextP) -> Result<Value, Error> {
        Ok(json!({
            "predicate" : match val.predicate {
                Text::Containing => Text::CONTAINING,
                Text::EndingWith => Text::ENDING_WITH,
                Text::StartingWith => Text::STARTING_WITH,
                Text::NotContaining => Text::NOT_CONTAINING,
                Text::NotEndingWith => Text::NOT_ENDING_WITH,
                Text::NotStartingWith => Text::NOT_STARTING_WITH,
            },
            "value" : val.value().serialize::<Self, D>()?
        }))
    }
}
