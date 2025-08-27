//! Wire format de/serialization implementations of our api.

// mod binary;
mod binary;
mod graphson;
mod typing;

pub(crate) use typing::*;

pub use binary::GraphBinary;
pub use graphson::GraphSON;
