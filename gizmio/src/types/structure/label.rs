use crate::{GValue, List, T, list};

pub enum LabelType {
    Str(String),
    Bool(bool),
    T(T),
}

pub struct Labels(pub List<LabelType>);

impl From<&str> for Labels {
    fn from(param: &str) -> Labels {
        Labels(list![LabelType::Str(String::from(param))])
    }
}

impl From<String> for Labels {
    fn from(param: String) -> Labels {
        Labels(list![LabelType::Str(param)])
    }
}

impl From<T> for Labels {
    fn from(param: T) -> Labels {
        Labels(list![LabelType::T(param)])
    }
}

impl From<()> for Labels {
    fn from(_: ()) -> Labels {
        Labels(list![])
    }
}
impl From<List<&str>> for Labels {
    fn from(param: List<&str>) -> Labels {
        Labels(
            param
                .into_iter()
                .map(|val| LabelType::Str(String::from(val)))
                .collect(),
        )
    }
}
impl From<List<String>> for Labels {
    fn from(param: List<String>) -> Labels {
        Labels(param.into_iter().map(LabelType::Str).collect())
    }
}

impl From<bool> for Labels {
    fn from(param: bool) -> Labels {
        Labels(list![LabelType::Bool(param)])
    }
}

impl From<(bool, List<&str>)> for Labels {
    fn from(param: (bool, List<&str>)) -> Labels {
        let mut out = list![LabelType::Bool(param.0)];
        out.append(&mut Into::<Labels>::into(param.1).0.drain(..).collect());
        Labels(out)
    }
}

impl From<(bool, T, List<&str>)> for Labels {
    fn from(param: (bool, T, List<&str>)) -> Labels {
        let mut out = list![LabelType::Bool(param.0)];
        out.append(&mut Into::<Labels>::into(param.1).0.drain(..).collect());
        out.append(&mut Into::<Labels>::into(param.2).0.drain(..).collect());
        Labels(out)
    }
}

impl From<(T, List<&str>)> for Labels {
    fn from(param: (T, List<&str>)) -> Labels {
        let mut out = list![LabelType::T(param.0)];
        out.append(&mut Into::<Labels>::into(param.1).0.drain(..).collect());
        Labels(out)
    }
}

macro_rules! impl_into_labels_str {
    ($n:expr) => {
        impl From<[&str; $n]> for Labels {
            fn from(param: [&str; $n]) -> Labels {
                Labels(
                    param
                        .iter()
                        .map(|s| LabelType::Str(String::from(*s)))
                        .collect(),
                )
            }
        }
    };
}

impl_into_labels_str!(1);
impl_into_labels_str!(2);
impl_into_labels_str!(3);
impl_into_labels_str!(4);
impl_into_labels_str!(5);
impl_into_labels_str!(6);
impl_into_labels_str!(7);
impl_into_labels_str!(8);
impl_into_labels_str!(9);
impl_into_labels_str!(10);
impl_into_labels_str!(11);
impl_into_labels_str!(12);
impl_into_labels_str!(13);
impl_into_labels_str!(14);
impl_into_labels_str!(15);
impl_into_labels_str!(16);

macro_rules! impl_into_labels_string {
    ($n:expr) => {
        impl From<[String; $n]> for Labels {
            fn from(param: [String; $n]) -> Labels {
                Labels(
                    param
                        .iter()
                        .map(|val| LabelType::Str(val.clone()))
                        .collect(),
                )
            }
        }
    };
}

impl_into_labels_string!(1);
impl_into_labels_string!(2);
impl_into_labels_string!(3);
impl_into_labels_string!(4);
impl_into_labels_string!(5);
impl_into_labels_string!(6);
impl_into_labels_string!(7);
impl_into_labels_string!(8);
impl_into_labels_string!(9);
impl_into_labels_string!(10);
impl_into_labels_string!(11);
impl_into_labels_string!(13);
impl_into_labels_string!(14);
impl_into_labels_string!(15);
impl_into_labels_string!(16);
impl TryFrom<GValue> for Labels {
    type Error = crate::Error;

    fn try_from(value: GValue) -> Result<Self, Self::Error> {
        match value {
            GValue::String(value) => Ok(Labels::from(value)),
            GValue::Bool(value) => Ok(Labels::from(value.0)),
            GValue::T(value) => Ok(Labels::from(value)),
            value => Err(crate::Error::unsupported(value)),
        }
    }
}

// impl<A, B> Into<Labels> for (A, B)
// where
//     A: Into<GValue>,
//     B: Into<GValue>,
// {
//     fn into(self) -> Labels {
//         GValue::List(list![self.0.into(), self.1.into()])
//     }
// }
