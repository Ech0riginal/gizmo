crate::primitive_prelude!();
crate::primitive!(Date, chrono::DateTime<chrono::Utc>);
crate::primitive!(Timestamp, i64);
macro_rules! from {
    ($ty:ty) => {
        impl From<$ty> for Timestamp {
            fn from(m: $ty) -> Self {
                Timestamp(m as i64)
            }
        }
    };
}

from!(u8);
from!(u16);
from!(u32);
from!(u64);
from!(i8);
from!(i16);
from!(i32);
