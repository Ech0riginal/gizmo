macro_rules! gvalue_test {
    ($fun:ident, $engine:ty, $dialect:ident, $case:expr) => {
        mod $fun {
            $crate::macros::test_macros::test_prelude!();
            $crate::macros::test_macros::tests!($crate::GValue);

            lazy_static::lazy_static! {
                static ref TEST_CASE: Test = $case;
            }

            $crate::macros::test_macros::module!($engine, $dialect, deserialize $crate::GValue);
            $crate::macros::test_macros::module!($engine, $dialect, serialize $crate::GValue);
        }
    };
}

macro_rules! response_test {
    ($fun:ident, $engine:ty, $dialect:ident, $case:expr) => {
        mod $fun {
            $crate::macros::test_macros::test_prelude!();
            $crate::macros::test_macros::tests!($crate::Response);

            lazy_static::lazy_static! {
                pub static ref TEST_CASE: Test = $case;
            }

            $crate::macros::test_macros::module!($engine, $dialect, deserialize $crate::Response);
            // module!($engine, serialize $crate::Response);
        }
    };
}

macro_rules! request_test {
    ($fun:ident, $engine:ty, $dialect:ident, $case:expr) => {
        mod $fun {
            $crate::macros::test_macros::test_prelude!();
            $crate::macros::test_macros::tests!($crate::Request);

            lazy_static::lazy_static! {
                pub static ref TEST_CASE: Test = $case;
            }

            // module!($engine, deserialize $crate::Request);
            $crate::macros::test_macros::module!($engine, $dialect, serialize $crate::Request);
        }
    };
}

pub(crate) use {gvalue_test, request_test, response_test};
