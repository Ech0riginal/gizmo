mod core;
mod id;
mod process;
mod request;
mod response;
mod structure;
mod tags;
mod value;
mod versions;

mod key;
#[cfg(test)]
mod tests;

pub use tags::{Tag, Type};
pub use versions::*;

pub(self) mod prelude {
    pub use serde_json::{Value, json};
    pub use std::collections::HashMap;

    pub use super::versions::*;
    pub use crate::io::graphson::tags::{Typed, core::*, process::*, structure::*};
    pub use crate::io::macros::*;
    pub use crate::io::{Deserialize, Deserializer};
    pub use crate::io::{Error, IOHelpers};
    pub use crate::io::{Serialize, Serializer};
    pub use crate::passthrough;
    pub use crate::structure::*;
}
