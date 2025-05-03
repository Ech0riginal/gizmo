// mod binary;

mod graphson;

mod error;

mod macros;

pub(crate) use macros::{get_value, expect_i32, expect_i64, expect_float, expect_double};

pub use error::Error;
pub use graphson::{V2, V3, V3g};

pub trait Gremlin:
    Send
    + Sync
    + Clone
    + std::fmt::Debug
    + Default
    + 'static
{
    fn mime() -> &'static str;

    fn deserialize(value: &serde_json::Value) -> crate::GremlinResult<crate::GValue>;

    fn serialize(value: &crate::GValue) -> crate::GremlinResult<serde_json::Value>;

    fn message<T>(op: String, processor: String, args: T, id: Option<uuid::Uuid>) -> crate::message::Message<T>;
}

