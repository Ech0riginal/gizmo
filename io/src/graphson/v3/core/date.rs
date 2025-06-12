use crate::graphson::prelude::*;
use chrono::{TimeZone, Utc};

impl Deserializer<Date> for V3 {
    fn deserialize(val: &Value) -> Result<Date, Leaf> {
        let val = expect_i64!(val).ctx::<Class>()?;
        let datetime = Utc.timestamp_millis_opt(val).unwrap();
        let date = Date(datetime);
        Ok(date)
    }
}

impl Serializer<Date> for V3 {
    fn serialize(val: &Date) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Date,
            "@value" : val.timestamp_millis()
        }))
    }
}
