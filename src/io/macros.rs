macro_rules! graphson_io {
    ($id:ident) => {
        #[derive(Clone, Debug, Default)]
        pub struct $id;

        impl crate::io::GraphSON for $id {
            fn new() -> Self { $id }
        }

        unsafe impl Send for $id {}

        unsafe impl Sync for $id {}
    };
}

macro_rules! graphson_types {
    ($name:ident, $value:expr) => {
        pub const $name: &'static str = $value;
    };
    { $module:ident, $($name:ident, $value:expr),* } => {
        pub mod $module {
            $(graphson_types!($name, $value);)*
        }
    };
}

macro_rules! get_value {
    ($value:expr,$v:path) => {
        match $value {
            $v(e) => Ok(e),
            _ => Err($crate::prelude::GremlinError::Json(String::from(
                stringify!($v),
            ))),
        }
    };
}

macro_rules! expect_i32 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v as i32),
            None => Err($crate::prelude::GremlinError::Json(String::from(
                "Expected i32",
            ))),
        }? as i32
    };
}

macro_rules! expect_i64 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v),
            None => Err($crate::prelude::GremlinError::Json(String::from(
                "Expected i64",
            ))),
        }?
    };
}
macro_rules! expect_float {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v as f32),
            None => Err($crate::prelude::GremlinError::Json(String::from(
                "Expected float",
            ))),
        }? as f32
    };
}
macro_rules! expect_double {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err($crate::prelude::GremlinError::Json(String::from(
                "Expected double",
            ))),
        }?
    };
}
