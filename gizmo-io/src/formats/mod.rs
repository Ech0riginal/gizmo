//! Wire format de/serialization implementations of our api.

// mod binary;
mod graphson;

pub use graphson::{GraphSON, GraphsonDeserializer, GraphsonSerializer};
// pub use binary::{GraphBinary, GraphBinaryDeserializer, GraphBinarySerializer};
