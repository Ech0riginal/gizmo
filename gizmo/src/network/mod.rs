use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

mod cmd;
mod receiver;
mod sender;
mod socket;

mod manage;

pub(crate) use cmd::Cmd;
pub(self) use receiver::ReceiverLoop;
pub(self) use sender::SenderLoop;
pub use socket::*;

pub type WSStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
