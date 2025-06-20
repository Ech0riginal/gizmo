use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct P {
    // TODO finish their P impl
    pub(crate) predicate: Predicate,
    pub(crate) value: Box<GValue>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Predicate {
    Equal,              //(List<GValue>),
    NotEqual,           //(List<GValue>),
    GreaterThan,        //(List<GValue>),
    GreaterThanOrEqual, //(List<GValue>),
    LessThan,           //(List<GValue>),
    LessThanOrEqual,    //(List<GValue>),
    Within,             //(List<GValue>),
    Without,            //(List<GValue>),
    Inside,             //([Box<GValue>; 2]),
    Outside,            //([Box<GValue>; 2]),
    Between,            //([Box<GValue>; 2]),
    And,                //(List<GValue>),
    Or,                 //(List<GValue>),
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
