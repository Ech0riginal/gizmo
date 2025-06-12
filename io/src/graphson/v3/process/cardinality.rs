use crate::graphson::prelude::*;

impl Deserializer<Cardinality> for V3 {
    fn deserialize(val: &Value) -> Result<Cardinality, Error> {
        todo!()
    }
}

impl Serializer<Cardinality> for V3 {
    fn serialize(val: &Cardinality) -> Result<Value, Error> {
        todo!()
    }
}
