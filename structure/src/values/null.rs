use crate::GValue;

#[derive(Debug, Clone)]
pub struct Null {}

impl From<GValue> for Null {
    fn from(value: GValue) -> Self {
        match value {
            GValue::Null => Null {},
            _ => panic!("Inconceivable!"),
        }
    }
}
