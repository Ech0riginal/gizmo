mod aux;
mod core;
mod process;
mod structure;

use crate::{Format, V2};
use crate::graphson::GraphSON;

impl Format for GraphSON<V2> {
    const mime: &'static str = "application/vnd.gremlin-v2.0+json";
    type Serial = serde_json::Value;
}
