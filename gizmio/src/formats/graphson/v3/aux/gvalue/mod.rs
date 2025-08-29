mod janus;
mod neptune;
mod sqlg;
mod tinker;

macro_rules! serde {
    (
        $dia:ident,
        $($var:ident -> $val:ty,)*$(,)?
    ) => {
        impl GraphsonSerializer<GValue, $dia> for GraphSON<V3> {
            fn serialize(val: &GValue) -> Result<Value, Error> {
                macro_rules! serialize {
                    ($val_:ident, $var_:ty) => {
                        $val_.serialize::<Self, $dia>()
                            .map(|v| serde_json::json!({ "@type": $dia::tag::<$var_>(), "@value": v }))
                    };
                }

                match val {
                    GValue::Null => Ok(Value::Null),
                    GValue::Bool(b) => Ok(Value::Bool(**b)),
                    GValue::String(string) => Ok(Value::String(string.to_string())),

                    $(
                        GValue::$var(v) => serialize!(v, $val),
                    )*

                    gvalue => Err(Error::unexpected(gvalue, "gvalue object")),
                }
            }
        }

        impl GraphsonDeserializer<GValue, $dia> for GraphSON<V3> {
            fn deserialize(val: &Value) -> Result<GValue, Error> {
                macro_rules! deserialize {
                    ($val_:expr, $var_:ty) => {
                        $val_.deserialize::<Self, $dia, $var_>().map(GValue::from)
                    };
                }

                match val {
                    Value::String(string) => Ok(GValue::from(string)),
                    Value::Number(_) => deserialize!(val, Integer),
                    Value::Object(_) => match val.typed() {
                        Ok(blob) => match blob.tag {
                            $(
                                <$val as AST<$dia>>::tag => deserialize!(blob.value, $val),
                                // TypeTag::$var => deserialize!(blob.value, $val),
                            )*

                            type_tag => Err(Error::unsupported(type_tag)),
                        },
                        Err(err) => Err(err),
                    },
                    Value::Array(_) => {
                        deserialize!(val, List<GValue>)
                        // let collection = values
                        //     .iter()
                        //     .map(|v| v.deserialize::<Self, D, GValue>())
                        //     .collect::<Result<List<_>, _>>()?;
                        // Ok(GValue::List(collection))
                    }
                    Value::Bool(bool) => Ok(Bool(*bool).into()),
                    Value::Null => Ok(GValue::Null),
                }
            }
        }
    };
}

pub(self) use serde;
