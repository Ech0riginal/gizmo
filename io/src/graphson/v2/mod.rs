// mod core;
// mod id;
// mod process;
// mod request;
// mod response;
// mod structure;
// mod value;

use crate::{Format, GValue};
use crate::graphson::GraphSON;
use crate::versions::V2;

impl Format for GraphSON<V2> {
    const mime: &'static str = "application/vnd.gremlin-v2.0+json";
    type Serial = serde_json::Value;
    type Object = GValue;
}
