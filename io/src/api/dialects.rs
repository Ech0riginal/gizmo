pub struct JanusGraph;

pub struct SQLg;

pub trait Dialect: Sized {
    fn tag<T>() -> &'static str where T: crate::Tag_<Self> { T::tag }
}

impl Dialect for JanusGraph {}

impl Dialect for SQLg {}

