use crate::api::Dialect;

#[derive(Clone, Copy)]
pub struct Neptune;
pub struct NeptuneHandler;
pub trait NeptuneMark {}

impl Dialect for Neptune {
    type Handler = NeptuneHandler;
}
