macro_rules! io {
    ($id:ident, $mime:expr) => {
        #[derive(Clone, Debug, Default)]
        pub struct $id;

        impl crate::Sealed for $id {}

        impl crate::io::GremlinIO for $id {
            const version: &'static str = stringify!($id);
            const mime: &'static str = $mime;
        }

        impl crate::io::IOHelpers for $id {}

        unsafe impl Send for $id {}

        unsafe impl Sync for $id {}
    };
}

io!(V2, "application/vnd.gremlin-v2.0+json");
io!(V3, "application/vnd.gremlin-v3.0+json;types=true");
// io!(V3g, V3::mime);

io!(Undefined, "n/a");

mod undefined {
    //! A virtual placeholder for when we initialize the client.

    use crate::GValue;
    use crate::io::graphson::Undefined;
    use crate::io::{Deserializer, Error, Request, Response, Serializer};
    use crate::structure::GID;
    use serde_json::Value;

    impl Deserializer<Response> for Undefined {
        fn deserialize(_: &Value) -> Result<Response, Error> {
            Err(Error::Unsupported("()".into()))
        }
    }

    impl Deserializer<GValue> for Undefined {
        fn deserialize(_: &Value) -> Result<GValue, Error> {
            Err(Error::Unsupported("()".into()))
        }
    }

    impl Deserializer<GID> for Undefined {
        fn deserialize(_: &Value) -> Result<GID, Error> {
            Err(Error::Unsupported("()".into()))
        }
    }

    impl Serializer<Request> for Undefined {
        fn serialize(_: &Request) -> Result<Value, Error> {
            Err(Error::Unsupported("()".into()))
        }
    }

    impl Serializer<GValue> for Undefined {
        fn serialize(_: &GValue) -> Result<Value, Error> {
            Err(Error::Unsupported("()".into()))
        }
    }

    impl Serializer<GID> for Undefined {
        fn serialize(_: &GID) -> Result<Value, Error> {
            Err(Error::Unsupported("()".into()))
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
            fn serialize(val: &$class) -> Result<Value, Error> {
                <$og as Serializer<$class>>::serialize(val)
            }
        }
    };
}

#[macro_export]
macro_rules! passthrough {
    ($class:ident, $overlay:ident to $og:ident) => {
        crate::deserialize_shim!($overlay, $og, $class);
        crate::serialize_shim!($overlay, $og, $class);
    };
}
