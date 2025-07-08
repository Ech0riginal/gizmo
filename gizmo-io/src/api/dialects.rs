pub trait Dialect: Sized {
    type Handler;
    
    fn tag<T>() -> &'static str
    where
        T: crate::Tag_<Self>,
    {
        T::tag
    }
}

pub trait SupportsGeometry {}