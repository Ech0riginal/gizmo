macro_rules! primitive {
    ($variant:ident, $primitive:ty) => {
        pub struct $variant($primitive);

        debug!($variant);
        display!($variant);
        deref!($variant, $primitive);
    };
}
macro_rules! deref {
    ($variant:ident, $primitive:ty) => {
        impl ops::Deref for $variant {
            type Target = $primitive;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
macro_rules! debug {
    ($variant:ident) => {
        impl fmt::Debug for $variant {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}
macro_rules! display {
    ($variant:ident) => {
        impl fmt::Display for $variant {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, stringify!($variant))
            }
        }
    };
}
macro_rules! hash {
    ($variant:ident) => {
        impl std::hash::Hash for $variant {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
}
macro_rules! eq {
    ($variant:ident) => {
        impl Eq for $variant {}

        impl PartialEq<Self> for $variant {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
    };
}
