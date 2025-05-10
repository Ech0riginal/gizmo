use crate::structure::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TextP {
    pub(crate) operator: String,
    pub(crate) value: Box<GValue>,
}

impl TextP {
    pub fn operator(&self) -> &String {
        &self.operator
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub(crate) fn new<T>(operator: T, value: GValue) -> TextP
    where
        T: Into<String>,
    {
        TextP {
            operator: operator.into(),
            value: Box::new(value),
        }
    }
    pub fn containing<V>(value: V) -> TextP
    where
        V: Into<GValue>,
    {
        TextP::new("containing", value.into())
    }

    pub fn starting_with<V>(value: V) -> TextP
    where
        V: Into<GValue>,
    {
        TextP::new("startingWith", value.into())
    }

    pub fn ending_with<V>(value: V) -> TextP
    where
        V: Into<GValue>,
    {
        TextP::new("endingWith", value.into())
    }

    pub fn not_starting_with<V>(value: V) -> TextP
    where
        V: Into<GValue>,
    {
        TextP::new("notStartingWith", value.into())
    }

    pub fn not_ending_with<V>(value: V) -> TextP
    where
        V: Into<GValue>,
    {
        TextP::new("notEndingWith", value.into())
    }

    pub fn not_containing<V>(value: V) -> TextP
    where
        V: Into<GValue>,
    {
        TextP::new("notContaining", value.into())
    }
}
