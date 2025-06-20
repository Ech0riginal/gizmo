use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct P {
    // TODO finish their P impl
    pub(crate) predicate: Predicate,
    pub(crate) value: Box<GValue>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Predicate {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Within,
    Without,
    Inside,
    Outside,
    Between,
    And,
    Or,
    Undocumented(String),
}

obj!(P);
tag!(P);

// We won't be de/serializing these from the top, so
// tagging doesn't matter, just gets us into the blankets
obj!(Predicate);
tag!(Predicate);

macro_rules! expose {
    ($pred:ident, $func:ident) => {
        pub fn $func<V>(value: V) -> P
        where
            V: Into<GValue>,
        {
            P {
                predicate: Predicate::$pred,
                value: value.into().boxed(),
            }
        }
    };
}

impl P {
    pub fn predicate(&self) -> &Predicate {
        &self.predicate
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub fn new(predicate: Predicate, value: GValue) -> P {
        P {
            predicate,
            value: Box::new(value),
        }
    }

    expose!(Equal, eq);
    expose!(NotEqual, neq);
    expose!(GreaterThan, gt);
    expose!(GreaterThanOrEqual, gte);
    expose!(LessThan, lt);
    expose!(LessThanOrEqual, lte);
    expose!(Within, within);
    expose!(Without, without);
    expose!(Inside, inside);
    expose!(Outside, outside);
    expose!(Between, between);
    expose!(And, and);
    expose!(Or, or);
}

pub trait IntoPredicate {
    fn into_predicate(self) -> Either2<P, TextP>;
}

pub struct Range {
    values: List<GValue>,
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
            values: list![v1, v2],
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
