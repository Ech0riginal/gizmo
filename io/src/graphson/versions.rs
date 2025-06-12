#[derive(Clone, Debug, Default)]
pub struct Undefined;
impl crate::Sealed for Undefined {}
impl crate::GremlinIO for Undefined {
    const version: &'static str = "Undefined";
    const mime: &'static str = "n/a";
}
unsafe impl Send for Undefined {}
unsafe impl Sync for Undefined {}

mod undefined {
    //! A virtual placeholder for when we initialize the client.

    use super::Undefined;
    use crate::api::*;
    use crate::graphson::Leaf;
    use crate::{GID, GValue, Request, Response};
    use serde_json::Value;

    impl Deserializer<Response> for Undefined {
        fn deserialize(_: &Value) -> Result<Response, Leaf> {
            Err(Leaf::Infallible)
        }
    }

    impl Deserializer<GValue> for Undefined {
        fn deserialize(_: &Value) -> Result<GValue, Leaf> {
            Err(Leaf::Infallible)
        }
    }

    impl Deserializer<GID> for Undefined {
        fn deserialize(_: &Value) -> Result<GID, Leaf> {
            Err(Leaf::Infallible)
        }
    }

    impl Serializer<Request> for Undefined {
        fn serialize(_: &Request) -> Result<Value, Leaf> {
            Err(Leaf::Infallible)
        }
    }

    impl Serializer<GValue> for Undefined {
        fn serialize(_: &GValue) -> Result<Value, Leaf> {
            Err(Leaf::Infallible)
        }
    }

    impl Serializer<GID> for Undefined {
        fn serialize(_: &GID) -> Result<Value, Leaf> {
            Err(Leaf::Infallible)
        }
    }
}

#[macro_export]
macro_rules! deserialize_shim {
    ($overlay:ident, $og:ident, $class:ident) => {
        impl Deserializer<$class> for $overlay {
            fn deserialize(val: &Value) -> Result<$class, Error> {
                <$og as Deserializer<$class>>::deserialize(val)
            }
        }
    };
}

#[macro_export]
macro_rules! serialize_shim {
    ($overlay:ident, $og:ident, $class:ident) => {
        impl Serializer<$class> for $overlay {
            fn serialize(val: &$class) -> Result<Value, Leaf> {
                <$og as Serializer<$class>>::serialize(val)
            }
        }
    };
}

#[macro_export]
macro_rules! passthrough {
    ($class:ident, $overlay:ident to $og:ident) => {
        $crate::deserialize_shim!($overlay, $og, $class);
        $crate::serialize_shim!($overlay, $og, $class);
    };
}
