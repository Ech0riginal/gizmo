use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Barrier, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Barrier, Error> {
        let str = get_value!(val, Value::String)?;
        match str.as_ref() {
            "normSack" | "norm_sack" => Ok(Barrier::NormSack),
            _ => Err(Error::Unexpected {
                actual: str.to_string(),
                expectation: "normSack".to_string(),
                location: location!(),
            }),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Barrier, D> for GraphSON<V2> {
    fn serialize(val: &Barrier) -> Result<Value, Error> {
        Ok(json!(match val {
            Barrier::NormSack => "normSack",
        }))
    }
}
