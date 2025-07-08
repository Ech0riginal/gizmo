use crate::api::Dialect;

pub struct SQLg;
pub struct SQLgHandler;
pub trait SQLgMark {}

impl Dialect for SQLg {
    type Handler = SQLgHandler;
}
