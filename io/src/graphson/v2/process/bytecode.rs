use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Bytecode, D> for GraphSON<V2> {
    fn deserialize(_val: &Value) -> Result<Bytecode, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Bytecode, D> for GraphSON<V2> {
    fn serialize(val: &Bytecode) -> Result<Value, Error> {
        let steps = val
            .steps()
            .iter()
            .map(|m| {
                let mut instruction = vec![];
                instruction.push(Value::String(m.operator.clone()));

                let arguments: Result<Vec<Value>, Error> =
                    m.args.iter().map(|a| a.serialize::<Self, D>()).collect();

                instruction.extend(arguments?);
                Ok(Value::Array(instruction))
            })
            .collect::<Result<Vec<Value>, Error>>()?;
        let sources = val
            .sources()
            .iter()
            .map(|m| {
                let mut instruction = vec![];
                instruction.push(Value::String(m.operator.clone()));

                let arguments: Result<Vec<Value>, Error> =
                    m.args.iter().map(|a| a.serialize::<Self, D>()).collect();

                instruction.extend(arguments?);
                Ok(Value::Array(instruction))
            })
            .collect::<Result<Vec<Value>, Error>>()?;
        Ok(json!({
            "step" : steps,
            "source" : sources,
        }))
    }
}
