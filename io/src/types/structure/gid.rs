use crate::*;

#[derive(Debug, Clone)]
pub struct GIDs(pub(crate) Vec<GID>);

impl<T: Into<GID>> From<T> for GIDs {
    fn from(val: T) -> GIDs {
        GIDs(vec![val.into()])
    }
}

impl<T: Into<GID>> From<Vec<T>> for GIDs {
    fn from(val: Vec<T>) -> GIDs {
        GIDs(val.into_iter().map(|gid| gid.into()).collect())
    }
}

impl From<()> for GIDs {
    fn from(_val: ()) -> GIDs {
        GIDs(vec![])
    }
}

macro_rules! gid {
    ($($variant:ident),+) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        pub enum GID {
            $($variant($variant),)+
        }

        $(
            impl Into<GID> for $variant {
                fn into(self) -> GID {
                    GID::$variant(self)
                }
            }
        )+
    };
}

gid!(String, Integer, Long);

impl Object for GID {
    const name: &'static str = "GID";
}

impl From<&'static str> for GID {
    fn from(val: &str) -> Self {
        GID::String(String::from(val))
    }
}

impl From<i32> for GID {
    fn from(val: i32) -> Self {
        Integer(val).into()
    }
}

impl From<i64> for GID {
    fn from(val: i64) -> Self {
        Long(val).into()
    }
}

impl From<&GID> for GID {
    fn from(val: &GID) -> Self {
        val.clone()
    }
}

impl From<Uuid> for GID {
    fn from(val: Uuid) -> Self {
        GID::String(val.to_string())
    }
}

// impl TryFrom<GValue> for GID {
//     type Error = Error;
//
//     fn try_from(value: GValue) -> Result<Self, Self::Error> {
//         Self::try_from(&value)
//     }
// }
//
// impl TryFrom<&GValue> for GID {
//     type Error = Error;
//
//     fn try_from(value: &GValue) -> Result<Self, Self::Error> {
//         match value {
//             GValue::String(s) => {
//                 let gid = if let Ok(i) = s.parse::<i64>() {
//                     Long(i).into()
//                 } else {
//                     Self::String(s.to_string())
//                 };
//                 Ok(gid)
//             }
//             _ => Err(Error::Cast(format!("{value:?}"), "GID".into())),
//         }
//     }
// }
