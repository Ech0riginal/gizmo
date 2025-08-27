use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TextP {
    pub(crate) predicate: Text,
    pub(crate) value: Box<GValue>,
}

obj!(TextP);
tag!(TextP);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Text {
    Containing,
    EndingWith,
    StartingWith,
    NotContaining,
    NotEndingWith,
    NotStartingWith,
}

obj!(Text);
tag!(Text);
string_reprs! {
    Text,
    CONTAINING -> "containing",
    ENDING_WITH -> "endingWith",
    STARTING_WITH -> "startingWith",
    NOT_CONTAINING -> "notContaining",
    NOT_ENDING_WITH -> "notEndingWith",
    NOT_STARTING_WITH -> "notStartingWith",
}

macro_rules! expose {
    ($pred:ident, $func:ident) => {
        pub fn $func<V>(value: V) -> TextP
        where
            V: Into<GValue>,
        {
            TextP {
                predicate: Text::$pred,
                value: value.into().boxed(),
            }
        }
    };
}

impl TextP {
    pub fn predicate(&self) -> &Text {
        &self.predicate
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    expose!(Containing, containing);
    expose!(EndingWith, ending_with);
    expose!(StartingWith, starting_with);
    expose!(NotContaining, not_containing);
    expose!(NotEndingWith, not_ending_with);
    expose!(NotStartingWith, not_starting_with);
}
