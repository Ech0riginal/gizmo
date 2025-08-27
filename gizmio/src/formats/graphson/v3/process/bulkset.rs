use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<BulkSet, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<GValue, D>,
{
    fn deserialize(val: &Value) -> Result<BulkSet, Error> {
        if val.to_string().contains("[null]") {
            // TODO Gremlin docs!?
            return Ok(BulkSet::default());
        }

        let map = val.deserialize::<Self, D, Map<GValue, GValue>>()?;
        let occurrences = map.len(); // I guess?
        Ok(BulkSet { map, occurrences })
    }
}

impl<D: Dialect> GraphsonSerializer<BulkSet, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(val: &BulkSet) -> Result<Value, Error> {
        val.map.serialize::<Self, D>()
    }
}
