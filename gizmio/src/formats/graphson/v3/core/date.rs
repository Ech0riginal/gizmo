use chrono::{TimeZone, Utc};

use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Date, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Date, Error> {
        let val = expect_i64!(val)?;
        let datetime = Utc.timestamp_millis_opt(val).unwrap();
        let date = Date(datetime);
        Ok(date)
    }
}

impl<D: Dialect> GraphsonSerializer<Date, D> for GraphSON<V3> {
    fn serialize(val: &Date) -> Result<Value, Error> {
        Ok(json!(val.timestamp_millis()))
    }
}
