use crate::io::{ContentType, MessageHandler};

pub(crate) mod de;
pub(crate) mod ser;
#[cfg(test)]
mod tests;
pub(crate) mod types;

graphson_io!(V3);

impl ContentType for V3 {
    fn content_type() -> &'static str {
        "application/vnd.gremlin-v3.0+json"
    }
}

impl MessageHandler for V3 {
    fn message<T>(
        op: String,
        processor: String,
        args: T,
        id: Option<uuid::Uuid>,
    ) -> crate::prelude::Message<T> {
        let request_id = id.unwrap_or_else(uuid::Uuid::new_v4);
        crate::prelude::Message::V3 {
            request_id,
            op,
            processor,
            args,
        }
    }
}
