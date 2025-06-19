pub use ensure::Ensure;
pub use typing::Typed;

mod ensure {
    use crate::Error;
    use serde_json::Value;
    use snafu::Location;

    pub trait Ensure<K: ?Sized, V> {
        #[track_caller]
        fn ensure(&self, key: &K) -> Result<&V, Error>;
    }

    impl<K> Ensure<K, Value> for serde_json::Map<String, Value>
    where
        K: ?Sized + AsRef<str>,
    {
        #[track_caller]
        fn ensure(&self, key: &K) -> Result<&Value, Error> {
            let key = key.as_ref().to_string();
            self.get(&key).ok_or({
                let caller = std::panic::Location::caller();
                Error::Missing {
                    key,
                    location: Location::new(caller.file(), caller.line(), caller.column()),
                }
            })
        }
    }

    impl Ensure<str, crate::GValue> for crate::Map<crate::GValue, crate::GValue> {
        #[track_caller]
        fn ensure(&self, key: &str) -> Result<&crate::GValue, Error> {
            let key = crate::GValue::from(key);
            self.get(&key).ok_or({
                let caller = std::panic::Location::caller();
                Error::Missing {
                    key: format!("{key:?}"),
                    location: Location::new(caller.file(), caller.line(), caller.column()),
                }
            })
        }
    }

    impl<K> Ensure<K, Value> for Value
    where
        K: ?Sized + AsRef<str>,
    {
        #[track_caller]
        fn ensure(&self, key: &K) -> Result<&Value, Error> {
            let key = key.as_ref().to_string();
            self.get(&key).ok_or({
                let caller = std::panic::Location::caller();
                Error::Missing {
                    key,
                    location: Location::new(caller.file(), caller.line(), caller.column()),
                }
            })
        }
    }
}

mod typing {
    use crate::Error;
    use crate::formats::graphson::Ensure;
    use serde_json::Value;
    use snafu::location;

    const TYPE_TAG: &str = "@type";
    const VALUE_TAG: &str = "@value";

    pub trait Typed {
        fn typed<'a>(&'a self) -> Result<Type<'a>, Error>;
    }

    #[derive(Debug)]
    pub struct Type<'a> {
        pub tag: &'a str,
        pub value: &'a Value,
    }

    impl Typed for Value {
        /// Validates a type against the expected { `@type`: ..., `@value`: ... } format
        fn typed<'a>(&'a self) -> Result<Type<'a>, Error> {
            let tag = match self.ensure(TYPE_TAG) {
                Ok(v) => v.as_str().ok_or(Error::Unexpected {
                    actual: format!("{:?}", v),
                    expectation: "a json string".into(),
                    location: location!(),
                }),
                Err(e) => Err(e),
            }?;

            match self.ensure(VALUE_TAG) {
                Ok(value) => Ok(Type { tag, value }),
                Err(e) => Err(e),
            }
        }
    }
}
