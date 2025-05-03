use crate::structure::GValue;
use std::sync::Arc;

use thiserror::Error;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum GremlinError {
    #[error("data store disconnected: {0}")]
    Generic(String),

    #[error(transparent)]
    Websocket(#[from] tungstenite::Error),

    #[error("Tungstenite Error {0}")]
    WebsocketClone(String),

    // #[error(transparent)]
    // Pool(#[from] r2d2::Error),
    #[error("Got wrong type {0:?}")]
    WrongType(String),

    #[error("Cast error: {0}")]
    Cast(String),

    #[error("JSON error: {0}")]
    Json(String),

    #[error("Request error: {0:?} ")]
    Request((i16, String)),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("An error occurred while performing handshake: {0}")]
    WebSocketHandshake(String),
    #[error("An error occurred while performing handshake: {0}")]
    WebSocketTlsHandshake(String),
    #[error(transparent)]
    WebSocketPool(#[from] Arc<mobc::Error<GremlinError>>),
    #[error(transparent)]
    ChannelSend(#[from] futures::channel::mpsc::SendError),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Tls(#[from] rustls::Error),
    #[error(transparent)]
    Pem(#[from] rustls_pki_types::pem::Error),
    #[error(transparent)]
    Parse(#[from] crate::io::Error),
}

impl From<mobc::Error<GremlinError>> for GremlinError {
    fn from(e: mobc::Error<GremlinError>) -> GremlinError {
        match e {
            mobc::Error::Inner(e) => e,
            other => GremlinError::WebSocketPool(Arc::new(other)),
        }
    }
}
