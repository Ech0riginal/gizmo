pub trait Dialect: Sized {
    fn tag<T>() -> &'static str
    where
        T: crate::Tag_<Self>,
    {
        T::tag
    }
}
