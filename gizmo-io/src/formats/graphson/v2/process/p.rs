//! P expects a single value of a List of values. There is special handling for List values when it
//! comes to `within`, `without`, `inside`, `outside` and `between`. For `inside`, `outside` and
//! `between`, the expectation is that the collection contain two objects (the rest will be ignored)
//! and those two objects become the arguments to those methods. For `within` and `without`, these
//! methods will accept an arbitrary number of objects in the collection.

use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<P, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<P, Error> {
        let map = get_value!(val, Value::Object)?;
        let predicate = map
            .ensure("predicate")?
            .deserialize::<Self, D, Predicate>()?;
        let value = {
            let value = map.ensure("value")?;
            let gvalue = if let Ok(list) = get_value!(value, Value::Array) {
                list.iter()
                    .map(|v| v.deserialize::<Self, D, GValue>())
                    .collect::<Result<List<_>, _>>()
                    .map(GValue::from)?
            } else if let Ok(blob) = value.typed() {
                blob.value.deserialize::<Self, D, GValue>()?
            } else {
                Err(Error::unexpected(value, "a List or typed value"))?
            };
            gvalue.boxed()
        };

        Ok(P { predicate, value })
    }
}

impl<D: Dialect> GraphsonDeserializer<Predicate, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Predicate, Error> {
        let string_repr = get_value!(val, Value::String)?;
        let enoom = match string_repr.as_str() {
            "gt" => Predicate::GreaterThan,
            "gte" => Predicate::GreaterThanOrEqual,
            "lt" => Predicate::LessThan,
            "lte" => Predicate::LessThanOrEqual,
            "within" => Predicate::Within,
            "without" => Predicate::Without,
            "inside" => Predicate::Inside,
            "outside" => Predicate::Outside,
            "between" => Predicate::Between,
            "and" => Predicate::And,
            "or" => Predicate::Or,
            value => Predicate::Undocumented(value.to_string()),
        };
        Ok(enoom)
    }
}

impl<D: Dialect> GraphsonSerializer<P, D> for GraphSON<V2> {
    fn serialize(val: &P) -> Result<Value, Error> {
        Ok(json!({
            "predicate": val.predicate.serialize::<Self, D>()?,
            "value": (*val.value).serialize::<Self, D>()?
        }))
    }
}

impl<D: Dialect> GraphsonSerializer<Predicate, D> for GraphSON<V2> {
    fn serialize(val: &Predicate) -> Result<Value, Error> {
        Ok(json!(match val {
            Predicate::Equal => "eq",
            Predicate::NotEqual => "neq",
            Predicate::GreaterThan => "gt",
            Predicate::GreaterThanOrEqual => "gte",
            Predicate::LessThan => "lt",
            Predicate::LessThanOrEqual => "lte",
            Predicate::Within => "within",
            Predicate::Without => "without",
            Predicate::Inside => "inside",
            Predicate::Outside => "outside",
            Predicate::Between => "between",
            Predicate::And => "and",
            Predicate::Or => "or",
            Predicate::Undocumented(something_from_the_deep) => something_from_the_deep.as_str(),
        }))
    }
}
