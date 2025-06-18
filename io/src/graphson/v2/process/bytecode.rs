use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Bytecode, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Bytecode, Error> {
        let obj = get_value!(val, Value::Object)?;
        let mut source_instructions = list![];
        let mut step_instructions = list![];

        if let Some(sourceroonies) = obj.get("source") {
            source_instructions.extend(
                get_value!(sourceroonies, Value::Array)?
                    .iter()
                    .map(|v| v.deserialize::<Self, D, Instruction>()).collect::<Result<List<_>, Error>>()?
                    .drain(..)
            )
        }

        if let Some(steperoonies) = obj.get("step") {
            step_instructions.extend(
                get_value!(steperoonies, Value::Array)?
                    .iter()
                    .map(|v| v.deserialize::<Self, D, Instruction>()).collect::<Result<List<_>, Error>>()?
                    .drain(..)
            )
        }


        Ok(Bytecode {
            source_instructions,
            step_instructions,
        })
    }
}

impl<D: Dialect> GraphsonDeserializer<Instruction, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Instruction, Error> {
        let arr = get_value!(val, Value::Array)?;
        if arr.is_empty() {
            return Err(Error::unexpected(val, "an array with a size of at least 1"))
        }

        let op = arr[0].deserialize::<Self, D, String>()?;
        let args = match arr.len() {
            1 => list![],
            2 => list![arr[1].deserialize::<Self, D, GValue>()?],
            _ => arr[1..]
                .iter()
                .map(|arg| arg.deserialize::<Self, D, GValue>())
                .collect::<Result<List<GValue>, Error>>()?,
        };

        Ok(Instruction {
            op,
            args
        })
    }
}

impl<D: Dialect> GraphsonSerializer<Bytecode, D> for GraphSON<V2> {
    fn serialize(val: &Bytecode) -> Result<Value, Error> {
        let steps = val
            .steps()
            .iter()
            .map(|m| {
                let mut instruction = vec![];
                instruction.push(Value::String(m.op.clone()));

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
                instruction.push(Value::String(m.op.clone()));

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
