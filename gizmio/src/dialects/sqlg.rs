use crate::api::Dialect;

#[derive(Clone, Copy)]
pub struct SQLg;
pub struct SQLgHandler;
pub trait SQLgMark {}

impl Dialect for SQLg {
    type Handler = SQLgHandler;
}
