crate::primitive_prelude!();
crate::very_primitive!(Date, chrono::DateTime<chrono::Utc>);
crate::hash!(Date);

impl Into<crate::GValue> for chrono::DateTime<chrono::Utc> {
    fn into(self) -> crate::GValue {
        crate::GValue::Date(Date(self))
    }
}

crate::primitive!(Timestamp, i64);
crate::partial_eq!(Timestamp);
crate::eq!(Timestamp);
crate::hash!(Timestamp);

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
