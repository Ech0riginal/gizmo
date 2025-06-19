mod aux;
mod core;
mod process;
mod structure;

impl crate::Format for super::GraphSON<crate::V2> {
    const mime: &'static str = "application/vnd.gremlin-v2.0+json";
    type Serial = serde_json::Value;
}
