pub trait Dialect: Clone + Sized + Send + Sync + 'static {
    type Handler;

    fn tag<T>() -> &'static str
    where
        T: crate::AST<Self>,
    {
        T::tag
    }
}
