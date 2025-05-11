use crate::io::Error;
use serde_json::Value;

const TYPE_TAG: &'static str = "@type";
const VALUE_TAG: &'static str = "@value";

pub(crate) struct Type<'a> {
    pub tag: &'a Value,
    pub value: &'a Value,
}

pub trait Typed {
    fn typed<'a>(&'a self) -> Result<Type<'a>, Error>;
}

impl Typed for Value {
    /// Validates a type against the expected { `@type`: ..., `@value`: ... } format
    /// Also I don't care to set up a proper API that's flexible re: lifetimes
    fn typed<'a>(&'a self) -> Result<Type<'a>, Error> {
        let tag = self.get(TYPE_TAG).ok_or(Error::Missing(TYPE_TAG))?;
        let value = self.get(VALUE_TAG).ok_or(Error::Missing(VALUE_TAG))?;
        Ok(Type { tag, value })
    }
}
