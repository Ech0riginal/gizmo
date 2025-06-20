pub trait Dialect: Sized {
    fn tag<T>() -> &'static str
    where
        T: crate::Tag_<Self>,
    {
        T::tag
    }
}

pub struct JanusGraph;

pub struct SQLg;

impl Dialect for JanusGraph {}

impl Dialect for SQLg {}
