//! P expects a single value of a List of values. There is special handling for List values when it
//! comes to `within`, `without`, `inside`, `outside` and `between`. For `inside`, `outside` and
//! `between`, the expectation is that the collection contain two objects (the rest will be ignored)
//! and those two objects become the arguments to those methods. For `within` and `without`, these
//! methods will accept an arbitrary number of objects in the collection.

use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<P, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<GValue, D>
{
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

impl<D: Dialect> GraphsonDeserializer<Predicate, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Predicate, Error> {
        let string_repr = get_value!(val, Value::String)?;
        let enoom = match string_repr.as_str() {
            Predicate::GREATER_THAN => Predicate::GreaterThan,
            Predicate::GREATER_THAN_OR_EQUAL => Predicate::GreaterThanOrEqual,
            Predicate::LESS_THAN => Predicate::LessThan,
            Predicate::LESS_THAN_OR_EQUAL => Predicate::LessThanOrEqual,
            Predicate::WITHIN => Predicate::Within,
            Predicate::WITHOUT => Predicate::Without,
            Predicate::INSIDE => Predicate::Inside,
            Predicate::OUTSIDE => Predicate::Outside,
            Predicate::BETWEEN => Predicate::Between,
            Predicate::AND => Predicate::And,
            Predicate::OR => Predicate::Or,
            value => Predicate::Undocumented(value.to_string()),
        };
        Ok(enoom)
    }
}

impl<D: Dialect> GraphsonSerializer<P, D> for GraphSON<V3>
where
    Self: Serializer<GValue, Value, D>,
    Self: Serializer<List<GValue>, Value, D>,
{
    fn serialize(val: &P) -> Result<Value, Error> {
        let value = match val.predicate {
            // This is fine, totally fine. (who thought this behavior's okay?)
            Predicate::Or
            | Predicate::And
            | Predicate::Inside
            | Predicate::Outside
            | Predicate::Between => match &*val.value {
                GValue::List(inner) => inner.serialize::<Self, D>(),
                any => any.serialize::<Self, D>(),
            },
            Predicate::Within | Predicate::Without => match &*val.value {
                GValue::List(inner) if inner.len() <= 2 => inner.serialize::<Self, D>(),
                any => any.serialize::<Self, D>(),
            },
            _ => (*val.value).serialize::<Self, D>(),
        }?;

        Ok(json!({
            "predicate": val.predicate.serialize::<Self, D>()?,
            "value": value,
        }))
    }
}

impl<D: Dialect> GraphsonSerializer<Predicate, D> for GraphSON<V3> {
    fn serialize(val: &Predicate) -> Result<Value, Error> {
        Ok(json!(match val {
            Predicate::Equal => Predicate::EQUAL,
            Predicate::NotEqual => Predicate::NOT_EQUAL,
            Predicate::GreaterThan => Predicate::GREATER_THAN,
            Predicate::GreaterThanOrEqual => Predicate::GREATER_THAN_OR_EQUAL,
            Predicate::LessThan => Predicate::LESS_THAN,
            Predicate::LessThanOrEqual => Predicate::LESS_THAN_OR_EQUAL,
            Predicate::Within => Predicate::WITHIN,
            Predicate::Without => Predicate::WITHOUT,
            Predicate::Inside => Predicate::INSIDE,
            Predicate::Outside => Predicate::OUTSIDE,
            Predicate::Between => Predicate::BETWEEN,
            Predicate::And => Predicate::AND,
            Predicate::Or => Predicate::OR,
            Predicate::Undocumented(something_from_the_deep) => something_from_the_deep.as_str(),
        }))
    }
}
