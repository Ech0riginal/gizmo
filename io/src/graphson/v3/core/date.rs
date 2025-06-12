use crate::graphson::prelude::*;
use chrono::{TimeZone, Utc};

impl Deserializer<Date> for V3 {
    fn deserialize(val: &Value) -> Result<Date, Error> {
        let val = expect_i64!(val)?;
        let datetime = Utc.timestamp_millis_opt(val).unwrap();
        let date = Date(datetime);
        Ok(date)
    }
}

impl Serializer<Date> for V3 {
    fn serialize(val: &Date) -> Result<Value, Error> {
        Ok(json!({
            "@type" : Tag::Date,
            "@value" : val.timestamp_millis()
        }))
    }
}
