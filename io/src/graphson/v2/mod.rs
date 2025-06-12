mod core;
mod process;
mod request;
mod response;
mod structure;
mod value;

#[derive(Clone, Debug, Default)]
pub struct V2;

impl crate::Sealed for V2 {}

impl crate::GremlinIO for V2 {
    const version: &'static str = "v2";
    const mime: &'static str = "application/vnd.gremlin-v2.0+json";
}

unsafe impl Send for V2 {}
unsafe impl Sync for V2 {}
