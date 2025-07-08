use crate::api::Dialect;

pub struct Tinker;
pub struct TinkerHandler;
pub trait TinkerMark {}

impl Dialect for Tinker {
    type Handler = TinkerHandler;
}
