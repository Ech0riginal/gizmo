mod id;
mod tags;
mod versions;

mod key;
#[cfg(test)]
pub(crate) mod tests;
mod v2;
mod v3;

pub use tags::{Tag, Type};
pub use v2::V2;
pub use v3::V3;

mod prelude {
    pub use serde_json::{Value, json};
    pub use std::collections::HashMap;

    pub use super::*;

    pub use crate::io::error::{Error, Missing};
    pub use crate::io::graphson::tags::*;
    pub use crate::io::macros::*;
    pub use crate::io::utils::Ensure;
    pub use crate::io::{Deserialize, Deserializer};
    pub use crate::io::{Serialize, Serializer};
    pub use crate::passthrough;
    pub use crate::graph::*;
}
