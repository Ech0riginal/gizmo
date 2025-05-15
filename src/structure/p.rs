use crate::structure::either::Either2;
use crate::structure::text_p::TextP;
use crate::structure::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct P {
    // TODO finish their P impl
    pub(crate) operator: String,
    pub(crate) value: Box<GValue>,
}

impl P {
    pub fn operator(&self) -> &String {
        &self.operator
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub(crate) fn new<T>(operator: T, value: GValue) -> P
    where
        T: Into<String>,
    {
        P {
            operator: operator.into(),
            value: Box::new(value),
        }
    }
    pub fn eq<V>(value: V) -> P
    where
        V: Into<GValue>,
    {
        P::new("eq", value.into())
    }

    pub fn neq<V>(value: V) -> P
    where
        V: Into<GValue>,
    {
        P::new("neq", value.into())
    }

    pub fn gt<V>(value: V) -> P
    where
        V: Into<GValue>,
    {
        P::new("gt", value.into())
    }

    pub fn gte<V>(value: V) -> P
    where
        V: Into<GValue>,
    {
        P::new("gte", value.into())
    }

    pub fn lt<V>(value: V) -> P
    where
        V: Into<GValue>,
    {
        P::new("lt", value.into())
    }

    pub fn lte<V>(value: V) -> P
    where
        V: Into<GValue>,
    {
        P::new("lte", value.into())
    }

    pub fn within<V>(value: V) -> P
    where
        V: IntoRange,
    {
        P::new("within", value.into_range().values.into())
    }
}

pub trait IntoPredicate {
    fn into_predicate(self) -> Either2<P, TextP>;
}

impl<T: Into<GValue>> IntoPredicate for T {
    fn into_predicate(self) -> Either2<P, TextP> {
        let val = self.into();
        match val {
            GValue::P(ref p) => Either2::A(p.clone()),
            GValue::TextP(ref p) => Either2::B(p.clone()),
            _ => Either2::A(P::new("eq", val)),
        }
    }
}

pub struct Range {
    values: Vec<GValue>,
}

pub trait IntoRange {
    fn into_range(self) -> Range;
}

impl<T> IntoRange for (T, T)
where
    T: Into<GValue>,
{
    fn into_range(self) -> Range {
        let v1 = self.0.into();
        let v2 = self.1.into();

        Range {
            values: vec![v1, v2],
        }
    }
}

impl<T> IntoRange for Vec<T>
where
    T: Into<GValue>,
{
    fn into_range(self) -> Range {
        Range {
            values: self.into_iter().map(|e| e.into()).collect(),
        }
    }
}
