use crate::graphson::{Error, MissingSnafu};
use serde_json::Value;
use snafu::OptionExt;

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
        self.get(&key).context(MissingSnafu { key })
    }
}

impl<K> Ensure<K, Value> for Value
where
    K: ?Sized + AsRef<str>,
{
    #[track_caller]
    fn ensure(&self, key: &K) -> Result<&Value, Error> {
        let key = key.as_ref().to_string();
        self.get(&key).context(MissingSnafu { key })
    }
}

// #[derive(std::fmt::Debug, Snafu)]
// #[snafu(visibility(pub))]
// pub enum IoError {
//     Io {
//         #[snafu(source)]
//         source: std::io::Error,
//         #[snafu(implicit)]
//         location: Location,
//     },
//     Serde {
//         #[snafu(source)]
//         source: serde_json::Error,
//         #[snafu(implicit)]
//         location: Location,
//     }
// }
