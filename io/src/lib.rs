#![feature(impl_trait_in_assoc_type)]

mod error;
mod graph;
mod io;

pub use error::Error;
pub use graph::*;

pub mod graphson {
    pub use super::io::graphson::{V2, V3};
}
