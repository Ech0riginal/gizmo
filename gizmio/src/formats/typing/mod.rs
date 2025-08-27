mod tags;

pub use tags::TypeTag;

use crate::formats::graphson::Ensure;
use serde_json::Value;
use snafu::location;

const TYPE_TAG: &str = "@type";
const VALUE_TAG: &str = "@value";

pub trait Typed: Sized {
    fn typed<'t>(&'t self) -> Result<Type<'t, Self>, crate::Error>;
}

#[derive(Debug)]
pub struct Type<'a, T> {
    pub tag: TypeTag,
    pub value: &'a T,
}

impl Typed for Value {
    fn typed<'t>(&'t self) -> Result<Type<'t, Self>, crate::Error> {
        let tagd: &'t str = match self.ensure(TYPE_TAG) {
            Ok(v) => v.as_str().ok_or(crate::Error::Unexpected {
                actual: format!("{v:?}"),
                expectation: "a string @type value".into(),
                location: location!(),
            }),
            Err(e) => Err(e),
        }?;
        let tag = TypeTag::try_from(tagd).map_err(|_e| crate::Error::unsupported(tagd))?;

        match self.ensure(VALUE_TAG) {
            Ok(value) => Ok(Type { tag, value }),
            Err(e) => Err(e),
        }
    }
}
