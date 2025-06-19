mod aux;
mod core;
mod extended;
mod process;
mod structure;

impl crate::Format for super::GraphSON<crate::V3> {

    const mime: &'static str = "application/vnd.gremlin-v3.0+json;types=true";
    type Serial = serde_json::Value;
}
