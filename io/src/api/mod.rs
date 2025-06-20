//! This is my API. There are many like it, but this one uses as many generics as possible
//! so we can draw de/serialization paths at compile-time, even if they're a little curvy.
//!
//!
//! Both the `deserialize` and `serialize` modules contain their respectively named traits
//! as well as blanket extension traits for each. These extensions attach an object's name
//! to its error context and are the entrypoint into the api. Our wire formats and versions
//! define their own serde traits, which allow us to hook into the 'global' extension
//! traits with the raw-ish `Deserializer<...>` and `Serializer<...>` bridge trait
//! implementations you'll find in each format's module.

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

mod blankets {
    use super::*;

    impl<T: Object> SerializeExt for T {}
}
