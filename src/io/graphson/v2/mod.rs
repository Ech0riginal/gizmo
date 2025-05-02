use crate::io::{ContentType, MessageHandler};
use crate::message::{Message, RequestIdV2};

pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

#[cfg(test)]
mod tests;

graphson_io!(V2);

impl ContentType for V2 {
    fn content_type() -> &'static str {
        "application/vnd.gremlin-v2.0+json"
    }
}

impl MessageHandler for V2 {
    fn message<T>(op: String, processor: String, args: T, id: Option<uuid::Uuid>) -> Message<T> {
        let request_id = id.unwrap_or_else(uuid::Uuid::new_v4);

        Message::V2 {
            request_id: RequestIdV2 {
                id_type: "g:UUID".to_string(),
                value: request_id,
            },
            op,
            processor,
            args,
        }
    }
}
