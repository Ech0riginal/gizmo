use crate::io::Error;
use crate::io::error::Missing;
use serde_json::Value;

pub trait Ensure<K: ?Sized, V> {
    fn ensure(&self, key: &K) -> Result<&V, Error>;
}

impl<K> Ensure<K, Value> for serde_json::Map<String, Value>
where
    K: ?Sized + AsRef<str>,
{
    fn ensure(&self, key: &K) -> Result<&Value, Error> {
        let sref = key.as_ref();
        self.get(sref).ok_or(sref.missing())
    }
}

impl<K> Ensure<K, Value> for Value
where
    K: ?Sized + AsRef<str>,
{
    fn ensure(&self, key: &K) -> Result<&Value, Error> {
        let key = key.as_ref();
        self.get(key).ok_or(key.missing())
    }
}
