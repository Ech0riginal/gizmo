crate::primitive_prelude!();
crate::primitive!(Class, String);
crate::deref!(Class, String);
crate::deref_mut!(Class);
crate::partial_eq!(Class);
crate::eq!(Class);
crate::hash!(Class);

impl From<&String> for Class {
    fn from(value: &String) -> Self {
        value.to_string().into()
    }
}

impl From<&str> for Class {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}
