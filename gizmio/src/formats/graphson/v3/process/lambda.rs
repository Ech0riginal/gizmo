use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Lambda, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Lambda, Error> {
        let val = get_value!(val, Value::Object)?;
        let script = val.ensure("script")?.deserialize::<Self, D, String>()?;
        let language = val.ensure("language")?.deserialize::<Self, D, String>()?;
        let arguments = {
            let tmp = val.ensure("arguments")?;
            get_value!(tmp, Value::Number)?
                .as_i64()
                .ok_or(Error::unexpected(tmp, "`arguments` to be a number"))?
        };

        Ok(Lambda {
            script,
            language,
            arguments,
        })
    }
}

impl<D: Dialect> GraphsonSerializer<Lambda, D> for GraphSON<V3> {
    fn serialize(val: &Lambda) -> Result<Value, Error> {
        Ok(json!({
            "script": val.script,
            "language": val.language,
            "arguments": val.arguments,
        }))
    }
}
