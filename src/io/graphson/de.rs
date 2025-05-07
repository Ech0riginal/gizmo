use crate::io::Error;
use serde_json::Value;

const TYPE_TAG: &'static str = "@type";
const VALUE_TAG: &'static str = "@value";

pub(crate) struct Blob<'a> {
    pub tag: &'a Value,
    pub value: &'a Value,
}

/// Validates a type against the expected { `@type`: ..., `@value`: ... } format
pub fn validate(value: &Value) -> Result<Blob, Error> {
    let tag = value.get(TYPE_TAG).ok_or(Error::Missing(TYPE_TAG))?;
    let value = value.get(VALUE_TAG).ok_or(Error::Missing(VALUE_TAG))?;
    Ok(Blob { tag, value })
}
