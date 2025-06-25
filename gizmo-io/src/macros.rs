#![allow(unused_macros)]

macro_rules! get_value {
    ($value:expr,Value::$v:ident) => {
        match $value {
            serde_json::Value::$v(e) => Ok(e),
            _ => Err($crate::api::Error::Unexpected {
                expectation: stringify!($v).to_string(),
                actual: format!("{:?}", $value),
                location: location!(),
            }),
        }
    };

    ($value:expr,GValue::$v:ident) => {
        match $value {
            $crate::GValue::$v(e) => Ok(e),
            _ => Err($crate::api::Error::Unexpected {
                expectation: stringify!($v).to_string(),
                actual: format!("{:?}", $value),
                location: location!(),
            }),
        }
    };
}

macro_rules! expect_i32 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v as i32),
            None => Err($crate::api::Error::Unexpected {
                expectation: "an i32".into(),
                actual: format!("{:?}", $value),
                location: snafu::location!(),
            }),
        }
    };
}

macro_rules! expect_i64 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v),
            None => Err($crate::api::Error::Unexpected {
                expectation: "an i64".to_string(),
                actual: format!("{:?}", $value),
                location: snafu::location!(),
            }),
        }
    };
}

macro_rules! expect_i128 {
    ($value:expr) => {
        match $value.as_i128() {
            Some(v) => Ok(v),
            None => Err($crate::api::Error::Unexpected {
                expectation: "An i128".to_string(),
                actual: format!("{:?}", $value),
                location: snafu::location!(),
            }),
        }?
    };
}

macro_rules! expect_f32 {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v as f32),
            None => Err($crate::api::Error::Unexpected {
                expectation: "An f32".to_string(),
                actual: format!("{:?}", $value),
                location: snafu::location!(),
            }),
        }
    };
}

macro_rules! expect_f64 {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err($crate::api::Error::Unexpected {
                expectation: "An f64".to_string(),
                actual: format!("{:?}", $value),
                location: snafu::location!(),
            }),
        }
    };
}

#[allow(unused_imports)]
pub(crate) use {expect_f32, expect_f64, expect_i32, expect_i64, expect_i128, get_value};

#[cfg(test)]
pub(crate) mod test_macros {
    macro_rules! test_prelude {
        () => {
            pub(self) use super::*;
            #[allow(unused_imports)]
            pub(self) use $crate::formats::graphson::tests::sanity::diff::{Diff, Difference};
            #[allow(unused_imports)]
            pub(self) use $crate::*;
        };
    }

    macro_rules! tests {
        ($type_:path) => {
            #[derive(Debug)]
            pub struct Test {
                pub(self) serial: serde_json::Value,
                pub(self) object: $type_,
            }
        };
    }

    macro_rules! module {
        ($engine:ty, $dialect:ident, serialize $ty:ty) => {
            mod serialization {
                pub(self) use super::*;

                #[test]
                fn ok() {
                    let result = TEST_CASE.object.serialize::<$engine, $dialect>();

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
                    let result = TEST_CASE.object.serialize::<$engine, $dialect>();
                    match result {
                        Err(e) => {
                            assert!(false, "serialization failed: {}", e);
                        }
                        Ok(item) => {
                            if (TEST_CASE.serial != item) {
                                let debug = TEST_CASE.serial.diff(&item);
                                assert!(
                                    debug.diff == Difference::Same,
                                    "serialization is not accurate: {}",
                                    debug
                                );
                            }
                        }
                    }
                }
            }
        };

        ($engine:ty, $dialect:ident, deserialize $ty:ty) => {
            mod deserialization {
                pub(self) use super::*;

                #[test]
                fn ok() {
                    let result = TEST_CASE.serial.deserialize::<$engine, $dialect, $ty>();
                    assert!(result.is_ok(), "deserialization failed: {:?}", result);
                }

                #[test]
                fn accurate() {
                    let result = TEST_CASE.serial.deserialize::<$engine, $dialect, $ty>();
                    match result {
                        Err(e) => {
                            assert!(false, "deserialization failed: {:#?}", e);
                        }
                        Ok(item) => {
                            if (TEST_CASE.object != item) {
                                let debug = TEST_CASE.object.diff(&item);
                                assert!(
                                    debug.diff == Difference::Same,
                                    "deserialization is not accurate: {}",
                                    debug
                                );
                            }
                        }
                    }
                }
            }
        };
    }

    pub(crate) use {module, test_prelude, tests};
}
