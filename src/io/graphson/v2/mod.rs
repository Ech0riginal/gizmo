use crate::io::{GremlinIO, IOHelpers};

pub(crate) mod types;

mod deserializers;
mod serializers;
#[cfg(test)]
mod tests;

crate::io::macros::io!(V2);

impl GremlinIO for V2 {
    const version: &'static str = "V2";

    fn mime() -> &'static str {
        "application/vnd.gremlin-v2.0+json"
    }
}

impl IOHelpers for V2 {}
