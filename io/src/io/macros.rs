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
pub struct Test<T> {
    pub serial: serde_json::Value,
    pub object: T,
}

#[macro_export]
macro_rules! test_prelude {
    () => {
        pub(self) use super::*;
        #[allow(unused_imports)]
        pub(self) use $crate::io::graphson::tests::diff::{Diff, Difference};
        #[allow(unused_imports)]
        pub(self) use $crate::io::*;
        #[allow(unused_imports)]
        pub(self) use $crate::*;
        #[allow(unused_imports)]
        pub(self) use $crate::{Args, Request, Response, Status};
    };
}
#[macro_export]
macro_rules! tests {
    ($type_:path) => {
        pub struct Test {
            pub(self) serial: serde_json::Value,
            pub(self) object: $type_,
        }
    };
}

#[macro_export]
macro_rules! module {
    ($engine:ident, serialize $ty:ty) => {
        mod serialization {
            pub(self) use super::*;

            #[test]
            fn ok() {
                let result = <$engine as $crate::io::Serializer<$ty>>::serialize(&TEST_CASE.object);
                match result {
                    Ok(_) => {
                        assert!(true);
                    }
                    Err(e) => {
                        assert!(false, "serialization failed: {:?}", e);
                    }
                }
            }

            #[test]
            fn accurate() {
                let result = <$engine as $crate::io::Serializer<$ty>>::serialize(&TEST_CASE.object);
                match result {
                    Err(e) => {
                        assert!(false, "serialization failed: {:?}", e);
                    }
                    Ok(item) => {
                        if (TEST_CASE.serial != item) {
                            let debug = TEST_CASE.serial.diff(&item);
                            assert!(debug.diff == Difference::Same, "{}", debug);
                        }
                    }
                }
            }
        }
    };

    ($engine:ident, deserialize $ty:ty) => {
        mod deserialization {
            pub(self) use super::*;

            #[test]
            fn ok() {
                let result =
                    <$engine as $crate::io::Deserializer<$ty>>::deserialize(&TEST_CASE.serial);
                assert!(result.is_ok(), "deserialization failed: {:?}", result);
            }

            #[test]
            fn accurate() {
                let result =
                    <$engine as $crate::io::Deserializer<$ty>>::deserialize(&TEST_CASE.serial);
                match result {
                    Err(e) => {
                        assert!(false, "deserialization failed: {:?}", e);
                    }
                    Ok(item) => {
                        if (TEST_CASE.object != item) {
                            let debug = TEST_CASE.object.diff(&item);
                            assert!(debug.diff == Difference::Same, "{}", debug);
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! test_case {
    ($case:expr) => {
        lazy_static::lazy_static! {
            static ref TEST_CASE: Test = $case;
        }
    };
}

#[macro_export]
macro_rules! gvalue_test {
    ($fun:ident, $engine:ident, $case:expr) => {
        mod $fun {
            $crate::test_prelude!();

            $crate::tests!($crate::GValue);

            lazy_static::lazy_static! {
                static ref TEST_CASE: Test = $case;
            }

            $crate::module!($engine, deserialize $crate::GValue);
            $crate::module!($engine, serialize $crate::GValue);
        }
    };
}

#[macro_export]
macro_rules! response_test {
    ($fun:ident, $engine:ident, $case:expr) => {
        mod $fun {
            $crate::test_prelude!();

            $crate::tests!($crate::Response);

            lazy_static::lazy_static! {
                pub static ref TEST_CASE: Test = $case;
            }

            $crate::module!($engine, deserialize $crate::Response);
            // $crate::module!($engine, serialize $crate::Response);
        }
    };
}

#[macro_export]
macro_rules! request_test {
    ($fun:ident, $engine:ident, $case:expr) => {
        mod $fun {
            $crate::test_prelude!();

            $crate::tests!($crate::Request);

            lazy_static::lazy_static! {
                pub static ref TEST_CASE: Test = $case;
            }

            // $crate::module!($engine, deserialize $crate::Request);
            $crate::module!($engine, serialize $crate::Request);
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
pub(crate) use {gvalue_test, io, test_prelude, types};
