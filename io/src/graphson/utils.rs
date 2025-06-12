use crate::error::Error;
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
