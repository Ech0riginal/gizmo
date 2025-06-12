// mod id;
mod key;
mod tags;
#[cfg(test)]
pub(crate) mod tests;
mod utils;
mod v2;
mod v3;

pub use tags::Tag;
pub use utils::Ensure;
pub use v2::V2;
pub use v3::V3;

mod prelude {
    pub use indexmap::IndexMap;
    pub use serde_json::{Value, json};
    pub use snafu::location;

    pub use super::*;

    pub use crate::api::{Deserialize, Deserializer};
    pub use crate::api::{Serialize, Serializer};
    pub use crate::error::Error;
    pub use crate::graphson::tags::*;
    pub use crate::macros::*;
    pub use crate::types::*;
}
