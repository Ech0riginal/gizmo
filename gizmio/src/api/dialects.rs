pub trait Dialect: Clone + Sized + Send + Sync + 'static {
    type Handler;

    fn tag<T>() -> &'static str
    where
        T: crate::Tag_<Self>,
    {
        T::tag
    }
}
