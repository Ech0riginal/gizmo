use crate::*;

primitive_prelude!();
very_primitive!(Date, chrono::DateTime<chrono::Utc>);
hash!(Date);
tag!(Date);

impl From<chrono::DateTime<chrono::Utc>> for crate::GValue {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        crate::GValue::Date(Date(val))
    }
}

primitive!(Timestamp, i64);
partial_eq!(Timestamp);
eq!(Timestamp);
hash!(Timestamp);
tag!(Timestamp);

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
