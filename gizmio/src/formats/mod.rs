//! Wire format de/serialization implementations of our api.

// mod binary;
mod binary;
mod graphson;
mod typing;

pub use binary::GraphBinary;
pub use graphson::GraphSON;
pub(crate) use typing::*;
