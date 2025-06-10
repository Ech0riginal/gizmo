#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("data store disconnected: {0}")]
    Generic(String),

    #[error("The channel or socket has closed.")]
    Closed,

    // #[error(transparent)]
    // Websocket(#[from] tungstenite::Error),
    #[error("Tungstenite Error {0}")]
    WebsocketClone(String),

    // #[error(transparent)]
    // Pool(#[from] r2d2::Error),
    #[error("Got wrong type {0:?}")]
    WrongType(String),

    #[error("Cannot cast {0} to {1}")]
    Cast(String, String),

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
    // #[error(transparent)]
    // ChannelSend(#[from] futures::channel::mpsc::SendError),
    // #[error(transparent)]
    // TokioChannelSend(#[from] tokio::sync::mpsc::error::SendError<tungstenite::Message>),
    // #[error(transparent)]
    // BrokenChannel(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    // #[error(transparent)]
    // Tls(#[from] rustls::Error),
    // #[error(transparent)]
    // Pem(#[from] rustls_pki_types::pem::Error),
    #[error("Timed out while making connection")]
    ConnectionTimeout,
    // #[error(transparent)]
    // Parse(#[from] crate::io::Error),
}

// impl From<bb8::RunError<Self>> for Error {
//     fn from(value: RunError<Self>) -> Self {
//         match value {
//             RunError::User(e) => e,
//             RunError::TimedOut => Self::ConnectionTimeout,
//         }
//     }
// }
