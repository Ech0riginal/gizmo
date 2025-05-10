crate::primitive_prelude!();
crate::primitive!(Class, String);
crate::partial_eq!(Class);
crate::eq!(Class);

impl From<&String> for Class {
    fn from(value: &String) -> Self {
        Self::from(value.clone())
    }
}

impl From<&str> for Class {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
