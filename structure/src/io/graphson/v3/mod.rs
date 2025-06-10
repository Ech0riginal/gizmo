use crate::io::graphson::{Tag, Type};
use crate::io::{Deserialize, Error};
use crate::*;

mod core;
mod extended;
mod process;
mod request;
mod response;
mod structure;
mod value;

#[derive(Clone, Debug, Default)]
pub struct V3;

impl io::Sealed for V3 {}

impl io::GremlinIO for V3 {
    const version: &'static str = "v3";
    const mime: &'static str = "application/vnd.gremlin-v3.0+json;types=true";
}

unsafe impl Send for V3 {}
unsafe impl Sync for V3 {}
