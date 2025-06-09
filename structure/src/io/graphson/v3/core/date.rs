use crate::io::graphson::prelude::*;
use chrono::{TimeZone, Utc};

passthrough!(Date, V3 to V2);

// impl Deserializer<Date> for V3 {
//     fn deserialize(val: &Value) -> Result<Date, Error> {
//         todo!()
//     }
// }
//
// impl Serializer<Date> for V3 {
//     fn serialize(val: &Date) -> Result<Value, Error> {
//         todo!()
//     }
// }
