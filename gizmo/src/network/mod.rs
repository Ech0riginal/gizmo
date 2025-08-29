use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

mod connection;
mod socket;

mod manage;
mod sugar;

pub use socket::*;

pub type WSStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
