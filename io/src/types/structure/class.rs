use crate::*;

primitive_prelude!();
primitive!(Class, String);
self::deref!(Class, String);
deref_mut!(Class);
partial_eq!(Class);
eq!(Class);
hash!(Class);
tag!(Class);

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
