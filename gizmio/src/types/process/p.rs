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
string_reprs! {
    Predicate,
    EQUAL -> "eq",
    NOT_EQUAL -> "neq",
    GREATER_THAN -> "gt",
    GREATER_THAN_OR_EQUAL -> "gte",
    LESS_THAN -> "lt",
    LESS_THAN_OR_EQUAL -> "lte",
    WITHIN -> "within",
    WITHOUT -> "without",
    INSIDE -> "inside",
    OUTSIDE -> "outside",
    BETWEEN -> "between",
    AND -> "and",
    OR -> "or",
}

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

    // pub fn within<V>(value: V) -> P
    // where
    //     V: Into<GValue>,
    // {
    //     P {
    //         predicate: Predicate::Within,
    //         value: value.into().boxed(),
    //     }
    // }

    expose!(Without, without);
    expose!(Inside, inside);
    expose!(Outside, outside);
    expose!(Between, between);
    expose!(And, and);
    expose!(Or, or);
}

// macro_rules! intzoo {
//     ($($var:ident),* => $($ind:expr),*) => {
//         impl<$($var),*> Into<GValue> for ($($var),*)
//             where
//                 $($var: Into<GValue>),*
//         {
//             fn into(self) -> GValue {
//                 GValue::List(list![])
//                 GValue::List(list![$(self.$ind.into()),*])
//             }
//         }
//     };
// }

impl<A, B> Into<GValue> for (A, B)
where
    A: Into<GValue>,
    B: Into<GValue>,
{
    fn into(self) -> GValue {
        GValue::List(list![self.0.into(), self.1.into()])
    }
}

impl<A, B, C> Into<GValue> for (A, B, C)
where
    A: Into<GValue>,
    B: Into<GValue>,
    C: Into<GValue>,
{
    fn into(self) -> GValue {
        GValue::List(list![self.0.into(), self.1.into(), self.2.into()])
    }
}
impl<A, B, C, D> Into<GValue> for (A, B, C, D)
where
    A: Into<GValue>,
    B: Into<GValue>,
    C: Into<GValue>,
    D: Into<GValue>,
{
    fn into(self) -> GValue {
        GValue::List(list![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into()
        ])
    }
}
impl<A, B, C, D, E> Into<GValue> for (A, B, C, D, E)
where
    A: Into<GValue>,
    B: Into<GValue>,
    C: Into<GValue>,
    D: Into<GValue>,
    E: Into<GValue>,
{
    fn into(self) -> GValue {
        GValue::List(list![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into()
        ])
    }
}
impl<A, B, C, D, E, F> Into<GValue> for (A, B, C, D, E, F)
where
    A: Into<GValue>,
    B: Into<GValue>,
    C: Into<GValue>,
    D: Into<GValue>,
    E: Into<GValue>,
    F: Into<GValue>,
{
    fn into(self) -> GValue {
        GValue::List(list![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into()
        ])
    }
}
impl<A, B, C, D, E, F, G> Into<GValue> for (A, B, C, D, E, F, G)
where
    A: Into<GValue>,
    B: Into<GValue>,
    C: Into<GValue>,
    D: Into<GValue>,
    E: Into<GValue>,
    F: Into<GValue>,
    G: Into<GValue>,
{
    fn into(self) -> GValue {
        GValue::List(list![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into()
        ])
    }
}
