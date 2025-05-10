#![allow(unused_macros)]

macro_rules! io {
    ($id:ident) => {
        #[derive(Clone, Debug, Default)]
        pub struct $id;

        unsafe impl Send for $id {}

        unsafe impl Sync for $id {}
    };
}

macro_rules! types {
    ($name:ident, $value:expr) => {
        pub const $name: &'static str = $value;
    };
    { $module:ident, $($name:ident, $value:expr),* } => {
        pub mod $module {
            $(crate::io::macros::types!($name, $value);)*
        }
    };
}
#[cfg(test)]
pub struct Test {
    pub serial: serde_json::Value,
    pub object: crate::structure::GValue,
}

macro_rules! test_prelude {
    () => {
        use crate::io::macros::Test;
        use crate::structure::*;
        #[allow(unused_imports)]
        use chrono::TimeZone;
        use serde_json::json;
    };
}

macro_rules! test {
    ($fun:ident, $engine:ident, $case:expr) => {
        mod $fun {
            pub(self) use super::*;

            lazy_static::lazy_static! {
                pub static ref TEST_CASE: Test = $case;
            }

            mod deserialize {
                pub(self) use super::*;
                #[allow(unused_imports)]
                use crate::io::{Deserializer, GremlinIO};

                #[test]
                fn ok() {
                    let result = <$engine as Deserializer<GValue>>::deserialize(&TEST_CASE.serial);
                    match result {
                        Ok(_) => assert!(true),
                        Err(e) => {
                            assert!(false, "Deserialization failed: {:?}", e);
                        }
                    }
                }

                #[test]
                fn accurate() {
                    let result = <$engine as Deserializer<GValue>>::deserialize(&TEST_CASE.serial);
                    assert!(result.is_ok(), "Deserialization failed");
                    assert_eq!(
                        TEST_CASE.object,
                        result.unwrap(),
                        "Deserialization doesn't match expectation"
                    );
                }
            }

            mod serialize {
                pub(self) use super::*;
                #[allow(unused_imports)]
                use crate::io::{GremlinIO, Serializer};

                #[test]
                fn ok() {
                    let result = $engine::serialize(&TEST_CASE.object);
                    match result {
                        Ok(_) => assert!(true),
                        Err(e) => {
                            assert!(false, "Serialization failed: {:?}", e);
                        }
                    }
                }

                #[test]
                fn accurate() {
                    let result = $engine::serialize(&TEST_CASE.object);
                    assert!(result.is_ok(), "Serialization failed");
                    assert_eq!(
                        TEST_CASE.serial,
                        result.unwrap(),
                        "Serialization doesn't match expectation"
                    );
                }
            }
        }
    };
}

macro_rules! get_value {
    ($value:expr,Value::$v:ident) => {
        match $value {
            Value::$v(e) => Ok(e),
            _ => Err($crate::io::error::Error::UnexpectedJson {
                msg: format!("Expected {}", stringify!($v)),
                value: $value.clone(),
            }),
        }
    };

    ($value:expr,GValue::$v:ident) => {
        match $value {
            GValue::$v(e) => Ok(e),
            v => Err($crate::io::error::Error::UnexpectedGValue {
                msg: format!("Expected {}", stringify!($v)),
                value: v.clone(),
            }),
        }
    };
}

macro_rules! expect_i32 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v as i32),
            None => Err($crate::io::error::Error::UnexpectedJson {
                msg: "Expected i32".into(),
                value: $value.clone(),
            }),
        }? as i32
    };
}

macro_rules! expect_i64 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v),
            None => Err($crate::io::error::Error::UnexpectedJson {
                msg: "Expected i64".into(),
                value: $value.clone(),
            }),
        }?
    };
}

macro_rules! expect_i128 {
    ($value:expr) => {
        match $value.as_i128() {
            Some(v) => Ok(v),
            None => Err($crate::io::error::Error::UnexpectedJson {
                msg: "Expected i128".into(),
                value: $value.clone(),
            }),
        }?
    };
}

macro_rules! expect_float {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v as f32),
            None => Err($crate::io::error::Error::UnexpectedJson {
                msg: "Expected f32".into(),
                value: $value.clone(),
            }),
        }? as f32
    };
}

macro_rules! expect_double {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err($crate::io::error::Error::UnexpectedJson {
                msg: "Expected f64".into(),
                value: $value.clone(),
            }),
        }?
    };
}

#[allow(unused_imports)]
pub(crate) use {expect_double, expect_float, expect_i32, expect_i64, expect_i128, get_value};
#[allow(unused_imports)]
pub(crate) use {io, test, test_prelude, types};
