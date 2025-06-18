mod aux;
mod core;
mod extended;
mod process;
mod structure;

use crate::graphson::GraphSON;
use crate::{Format, V3};

impl Format for GraphSON<V3> {
    const mime: &'static str = "application/vnd.gremlin-v3.0+json;types=true";
    type Serial = serde_json::Value;
}
