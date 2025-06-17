// mod core;
// mod extended;
// mod id;
// mod process;
// mod request;
// mod response;
// mod structure;
// mod value;

use crate::{Format, GValue};
use crate::graphson::GraphSON;
use crate::versions::V3;

impl Format for GraphSON<V3> {
    const mime: &'static str = "application/vnd.gremlin-v3.0+json;types=true";
    type Serial = serde_json::Value;
    type Object = GValue;
}