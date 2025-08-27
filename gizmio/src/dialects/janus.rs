use crate::api::Dialect;

#[derive(Clone, Copy)]
pub struct Janus;
pub struct JanusHandler;
pub trait JanusMark {}

impl Dialect for Janus {
    type Handler = JanusHandler;
}
