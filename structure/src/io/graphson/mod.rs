mod id;
// mod request;
// mod response;
mod tags;
mod value;
mod versions;

mod key;
#[cfg(test)]
mod tests;
mod v2;
mod v3;

pub use tags::{Tag, Type};
pub use versions::*;

pub(self) mod prelude {
    pub use serde_json::{Value, json};
    pub use std::collections::HashMap;

    pub use super::versions::*;

    pub use crate::io::graphson::tags::*;
    pub use crate::io::macros::*;
    pub use crate::io::{Deserialize, Deserializer};
    pub use crate::io::{Error, IOHelpers};
    pub use crate::io::{Serialize, Serializer};
    pub use crate::passthrough;
    pub use crate::primitive::*;
    pub use crate::values::*;
}
