use crate::io::graphson::prelude::*;
use chrono::{TimeZone, Utc};

impl Deserializer<Date> for V2 {
    fn deserialize(val: &Value) -> Result<Date, Error> {
        let val = expect_i64!(val);
        let datetime = Utc.timestamp_millis_opt(val).unwrap();
        let date = Date(datetime);
        Ok(date.into())
    }
}

impl Serializer<Date> for V2 {
    fn serialize(val: &Date) -> Result<Value, Error> {
        Ok(json!({
            "@type" : Tag::Date,
            "@value" : val.timestamp_millis()
        }))
    }
}
