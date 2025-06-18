mod deserialize;
mod dialects;
mod error;
mod format;
mod object;
mod serialize;
mod versions;

pub use deserialize::*;
pub use dialects::*;
pub use error::*;
pub use format::*;
pub use object::*;
pub use serialize::*;
pub use versions::*;

pub(crate) trait Sealed {}

mod blankets {
    use super::*;

    impl<T: Object> SerializeExt for T {}
    impl DeserializeExt for serde_json::Value {}
}
